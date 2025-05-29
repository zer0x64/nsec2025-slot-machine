use flume::Receiver;
use tauri::ipc::Channel;

use crate::{
    clock::Clock,
    models::Symbol,
    rngs::{LevelMetadata, SlotLevel, SlotRng},
    timer::Timer,
};

#[derive(Default, Debug)]
pub struct SlotMachine(Option<SlotMachineInner>);

impl SlotMachine {
    pub fn new(level: &SlotLevel, js_channel: Channel<String>, clock: &Clock) -> Self {
        Self(Some(SlotMachineInner::new(level, js_channel, clock)))
    }

    pub fn stop(&mut self) {
        self.0.take();
    }

    pub fn get_mut(&mut self) -> Option<&mut SlotMachineInner> {
        // The block is necessary to avoid lifetime issues
        let is_timeout = {
            let x: &mut SlotMachineInner = {
                // If there is no slot machine, return none
                match self.0.as_mut() {
                    Some(x) => x,
                    None => return None,
                }
            };

            x.check_timer()
        };

        // If out of time, drop the clock
        if is_timeout {
            let _ = self.0.take();
            None
        } else {
            self.0.as_mut()
        }
    }
}

pub struct SlotMachineInner {
    pub credits: usize,
    rng: Box<dyn SlotRng>,
    _timer: Timer,
    finish_receiver: Receiver<()>,
}

impl std::fmt::Debug for SlotMachineInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.rng.get_metadata())
    }
}

impl SlotMachineInner {
    pub fn new(level: &SlotLevel, js_channel: Channel<String>, clock: &Clock) -> Self {
        let rng = level.get_rng(clock);
        let metadata = rng.get_metadata();
        let credits = metadata.starting_credits;

        // Subscribe timer to master clock
        let receiver = clock.subscribe();

        // Channel to communicate time out
        let (finish_sender, finish_receiver) = flume::bounded(1);
        let timer = Timer::new(js_channel, receiver, metadata.alloted_time, finish_sender);

        Self {
            credits,
            rng,
            _timer: timer,
            finish_receiver,
        }
    }

    pub fn spin(&mut self) -> Vec<Symbol> {
        let n_wheels = self.rng.get_metadata().n_wheels as usize;

        // Tick time-sensitive RNGs
        self.rng.tick();
        (0..n_wheels)
            .into_iter()
            .map(|_| Symbol::from_rng_number(self.rng.get_byte()))
            .collect()
    }

    pub fn calculate_payout(&self, bet: usize, symbols: &[Symbol]) -> usize {
        // Check if they all match
        let first = symbols[0];

        if symbols.iter().all(|&item| item == first) {
            self.rng.get_payout(&first) * bet
        } else {
            0
        }
    }

    pub fn get_metadata(&self) -> LevelMetadata {
        self.rng.get_metadata().clone()
    }

    pub fn get_debug_info(&mut self) -> Option<Vec<u8>> {
        self.rng.get_debug_info()
    }

    pub fn get_flag(&self) -> Option<String> {
        if self.credits >= self.rng.get_metadata().required_credits {
            Some(self.rng.get_flag().to_string())
        } else {
            None
        }
    }

    pub(super) fn check_timer(&self) -> bool {
        match self.finish_receiver.try_recv() {
            Ok(_) => true,
            Err(flume::TryRecvError::Disconnected) => true,
            Err(flume::TryRecvError::Empty) => false,
        }
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Symbol {
    pub fn from_rng_number(number: u8) -> Self {
        match number {
            0..128 => Self::Cherry,
            128..192 => Self::Bar,
            192..218 => Self::DoubleBar,
            218..240 => Self::TripleBar,
            240..249 => Self::Seven,
            249..253 => Self::MinorJackpot,
            253..255 => Self::MajorJackpot,
            255 => Self::GrandJackpot,
        }
    }
}
