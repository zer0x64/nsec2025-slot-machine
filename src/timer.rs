use std::{thread::JoinHandle, time::SystemTime};

use chrono::DateTime;
use flume::{Receiver, Sender};
use tauri::ipc::Channel;

pub struct Timer {
    // Thread handle
    thread: Option<JoinHandle<()>>,

    // Used to send stop signals
    thread_sender: Sender<()>,
}

impl Timer {
    pub fn new(
        js_channel: Channel<String>,
        master_clock: Receiver<SystemTime>,
        time_secs: u64,
        timeout_sender: Sender<()>,
    ) -> Self {
        let (thread_sender, thread_receiver) = flume::unbounded();

        let thread = Option::Some(std::thread::spawn(move || {
            timer_thread(
                js_channel,
                master_clock,
                thread_receiver,
                time_secs,
                timeout_sender,
            )
        }));

        Timer {
            thread,
            thread_sender,
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        // Send stop signal
        let _ = self.thread_sender.send(());

        // Wait for the thread to stop
        if let Some(h) = self.thread.take() {
            let _ = h.join();
        };
    }
}

fn timer_thread(
    js_channel: Channel<String>,
    master_clock: Receiver<SystemTime>,
    thread_receiver: Receiver<()>,
    mut time_secs: u64,
    timeout_sender: Sender<()>,
) {
    loop {
        // Stop thread if requested or if the sender is dropped
        match thread_receiver.try_recv() {
            Ok(_) => break,
            Err(flume::TryRecvError::Disconnected) => break,
            Err(flume::TryRecvError::Empty) => {}
        }

        // Wait for master clock
        match master_clock.recv() {
            Ok(_) => time_secs -= 1,
            Err(flume::RecvError::Disconnected) => break,
        }

        let datetime =
            DateTime::from_timestamp(time_secs as i64, 0).expect("timer outside allowed range!");

        match js_channel.send(datetime.format("%M:%S").to_string()) {
            Ok(()) => {}
            Err(e) => tracing::warn!("Couldn't send clock event to frontend! {}", e),
        }

        if time_secs <= 0 {
            let _ = timeout_sender.send(());
            break;
        }
    }
}
