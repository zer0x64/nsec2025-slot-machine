use std::{
    sync::{Arc, RwLock},
    thread::JoinHandle,
    time::Duration,
};

use flume::{Receiver, Sender};
use gpiocdev::{
    line::{Bias, Direction, EdgeDetection, EdgeKind},
    Request,
};
use serde::{Deserialize, Serialize};
use tauri::ipc::Channel;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GpioEventMessage {
    event_type: String,
    code: String,
}

type Subscribers = Arc<RwLock<Vec<Channel<GpioEventMessage>>>>;

pub struct Gpio {
    // Channel to subscribers listening for gpio events
    subscribers: Subscribers,

    // Thread handle
    thread: Option<JoinHandle<()>>,

    // Used to send stop signals
    thread_channel: Sender<()>,
}

pub fn gpio_thread(stop_signal: Receiver<()>, subscribers: Subscribers) {
    const UP_OFFSET: u32 = 22;
    const DOWN_OFFSET: u32 = 23;
    const SPIN_OFFSET: u32 = 24;
    const BADGE_OFFSET: u32 = 25;

    let offsets = [UP_OFFSET, DOWN_OFFSET, SPIN_OFFSET, BADGE_OFFSET];

    let req = Request::builder()
        .on_chip("/dev/gpiochip0")
        .with_lines(&offsets)
        .with_direction(Direction::Input)
        .with_bias(Bias::PullDown)
        .with_debounce_period(Duration::from_millis(10))
        .with_edge_detection(EdgeDetection::BothEdges)
        .request()
        .expect("Failed to request GPIO lines");

    loop {
        // If we receive a signal or the sender is dropped, stop the thread
        match stop_signal.try_recv() {
            Ok(_) => break,
            Err(flume::TryRecvError::Disconnected) => break,
            Err(flume::TryRecvError::Empty) => {}
        }

        let event = req.read_edge_event().expect("Failed to read edge event");
        tracing::debug!("New gpio event: {:?}", event);

        let code = match event.offset {
            UP_OFFSET => "ArrowUp".to_string(),
            DOWN_OFFSET => "ArrowDown".to_string(),
            SPIN_OFFSET => "Space".to_string(),
            BADGE_OFFSET => "KeyB".to_string(),
            _ => unreachable!(),
        };

        let event_type = match event.kind {
            EdgeKind::Rising => "keydown".to_string(),
            EdgeKind::Falling => "keyup".to_string(),
        };

        let message = GpioEventMessage { event_type, code };
        tracing::debug!("Message to send: {:?}", message);

        for subscriber in subscribers.read().expect("poisonned lock!").iter() {
            subscriber
                .send(message.clone())
                .expect("Failed to send event to the frontend")
        }
    }
}

impl Gpio {
    pub fn new() -> Self {
        let (thread_sender, thread_receiver) = flume::unbounded();
        let subscribers: Subscribers = Default::default();

        tracing::debug!("GPIO feature is enabled, starting thread");
        let subscribers_clone = subscribers.clone();
        let thread = Option::Some(std::thread::spawn(|| {
            gpio_thread(thread_receiver, subscribers_clone)
        }));

        Self {
            subscribers,
            thread,
            thread_channel: thread_sender,
        }
    }

    pub fn subscribe(&mut self, js_channel: Channel<GpioEventMessage>) {
        self.subscribers
            .write()
            .expect("poisoned lock!")
            .push(js_channel);
    }
}

impl std::fmt::Debug for Gpio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gpio")
            .field("thread", &self.thread)
            .field("thread_channel", &self.thread_channel)
            .finish()
    }
}

impl Drop for Gpio {
    fn drop(&mut self) {
        // Send stop signal
        let _ = self.thread_channel.send(());

        // Wait for the thread to stop
        if let Some(h) = self.thread.take() {
            let _ = h.join();
        };
    }
}
