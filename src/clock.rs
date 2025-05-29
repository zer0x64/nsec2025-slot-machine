use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, RwLock,
    },
    thread::JoinHandle,
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Local};
use flume::{Receiver, Sender};
use tauri::ipc::Channel;

pub const DEFAULT_ORDERING: Ordering = Ordering::Relaxed;

#[derive(Debug)]
pub struct Clock {
    // The tick is aligned with the seconds
    pub last_tick: Arc<AtomicU64>,

    // Senders for the time updates
    pub channels: Arc<RwLock<Vec<Sender<SystemTime>>>>,

    // Thread handle
    thread: Option<JoinHandle<()>>,

    // Used to send stop signals
    thread_channel: Sender<()>,
}

pub fn clock_thread(
    last_tick_atomic: Arc<AtomicU64>,
    stop_signal: Receiver<()>,
    js_channel: Channel<String>,
    channels: Arc<RwLock<Vec<Sender<SystemTime>>>>,
) {
    // We start by synchronizing the clock to the second
    // Convoluted, but this floors the SystemTime
    let now = SystemTime::now();

    let last_tick = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("clock can't be older then epoch!")
        .as_secs();

    // We sleep until the next full second
    if let Ok(to_wait) = SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_secs(last_tick + 1))
        .expect("time shouldn't be outside UNIX timestamp range")
        .duration_since(now)
    {
        std::thread::sleep(to_wait);
    }

    last_tick_atomic.store(last_tick, DEFAULT_ORDERING);

    loop {
        // If we receive a signal or the sender is dropped, stop the thread
        match stop_signal.try_recv() {
            Ok(_) => break,
            Err(flume::TryRecvError::Disconnected) => break,
            Err(flume::TryRecvError::Empty) => {}
        }

        // Read the last tick
        let last_tick = last_tick_atomic.load(DEFAULT_ORDERING);

        let next_tick = SystemTime::UNIX_EPOCH
            .checked_add(Duration::from_secs(last_tick + 1))
            .expect("time shouldn't be outside UNIX timestamp range");

        let now = SystemTime::now();

        // Sleep until the next second
        // An error here mean we don't have to wait
        if let Ok(to_wait) = next_tick.duration_since(now) {
            std::thread::sleep(to_wait);
        }

        last_tick_atomic.store(last_tick + 1, DEFAULT_ORDERING);

        // Send to rust to update rng
        let to_close: Vec<usize> = {
            channels
                .read()
                .expect("poisonned lock!")
                .iter()
                .enumerate()
                .filter_map(|(i, c)| match c.send(next_tick) {
                    Ok(_) => None,
                    Err(_) => Some(i),
                })
                .collect()
        };

        // Drop identified channels
        if !to_close.is_empty() {
            let mut channels_writer = channels.write().expect("poisonned lock!");
            for i in to_close.into_iter().rev() {
                // swap_remove is more performant, but doesn't keep the order
                channels_writer.swap_remove(i);
            }
        }

        // Format the time for javascript
        let current_second_datetime: DateTime<Local> = next_tick.into();
        match js_channel.send(current_second_datetime.format("%k:%M:%S").to_string()) {
            Ok(()) => {}
            Err(e) => tracing::warn!("Couldn't send clock event to frontend! {}", e),
        }
    }
}

impl Clock {
    pub fn new(js_channel: Channel<String>) -> Self {
        let (thread_sender, thread_receiver) = flume::unbounded();

        let last_tick: Arc<AtomicU64> = Default::default();
        let senders: Arc<RwLock<Vec<Sender<SystemTime>>>> = Default::default();

        let last_tick_clone = last_tick.clone();
        let senders_clone = senders.clone();
        let thread = Option::Some(std::thread::spawn(|| {
            clock_thread(last_tick_clone, thread_receiver, js_channel, senders_clone)
        }));

        Self {
            last_tick,
            channels: senders,
            thread,
            thread_channel: thread_sender,
        }
    }

    pub fn subscribe(&self) -> Receiver<SystemTime> {
        let (sender, receiver) = flume::unbounded();
        self.channels.write().expect("poisoned lock!").push(sender);

        receiver
    }
}

impl Drop for Clock {
    fn drop(&mut self) {
        // Send stop signal
        let _ = self.thread_channel.send(());

        // Wait for the thread to stop
        if let Some(h) = self.thread.take() {
            let _ = h.join();
        };
    }
}
