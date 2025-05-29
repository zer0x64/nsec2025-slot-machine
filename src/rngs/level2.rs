use flume::Receiver;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::SystemTime;

use crate::clock::Clock;

use super::{SlotRng, DEFAULT_REEL_LAYOUT, DEFAULT_TIME_SECS};

pub struct Level2Rng {
    rng: StdRng,
    clock_receiver: Receiver<SystemTime>,
}

impl Level2Rng {
    pub fn new(clock: &Clock) -> Self {
        // Level 2: Time based RNG
        let clock_receiver = clock.subscribe();
        Self {
            rng: StdRng::seed_from_u64(clock.last_tick.load(crate::clock::DEFAULT_ORDERING)),
            clock_receiver,
        }
    }
}

impl SlotRng for Level2Rng {
    fn get_metadata(&self) -> super::LevelMetadata {
        super::LevelMetadata {
            level_num: 2,
            n_wheels: 3,
            reel_layout: DEFAULT_REEL_LAYOUT.to_vec(),
            starting_credits: 1_000,
            required_credits: 20_000,
            denomination: 20,
            alloted_time: DEFAULT_TIME_SECS,
        }
    }

    fn get_flag(&self) -> &str {
        #[cfg(feature = "prod")]
        return include_str!("./flags/FLAG2.txt");
        #[cfg(not(feature = "prod"))]
        return include_str!("./flags/FLAG2-DEBUG.txt");
    }

    fn get_byte(&mut self) -> u8 {
        // We get a byte for every second that passed
        while let Ok(_) = self.clock_receiver.try_recv() {
            let _: u8 = self.rng.random();
        }

        self.rng.random()
    }
}
