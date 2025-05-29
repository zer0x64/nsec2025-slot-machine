mod simple_lfsr;
use simple_lfsr::SimpleLfsr;

use rand::{Rng, SeedableRng};

use super::{SlotRng, DEFAULT_REEL_LAYOUT, DEFAULT_TIME_SECS};

pub struct Level4Rng {
    rng: SimpleLfsr,
}

impl Level4Rng {
    pub fn new() -> Self {
        // Level 4: Clone a LFSR's state
        Self {
            rng: SimpleLfsr::from_os_rng(),
        }
    }
}

impl SlotRng for Level4Rng {
    fn get_metadata(&self) -> super::LevelMetadata {
        super::LevelMetadata {
            level_num: 4,
            n_wheels: 3,
            reel_layout: DEFAULT_REEL_LAYOUT.to_vec(),
            starting_credits: 500_000,
            required_credits: 100_000_000,
            denomination: 10_000,
            alloted_time: DEFAULT_TIME_SECS,
        }
    }

    fn get_flag(&self) -> &str {
        #[cfg(feature = "prod")]
        return include_str!("../flags/FLAG4.txt");
        #[cfg(not(feature = "prod"))]
        return include_str!("../flags/FLAG4-DEBUG.txt");
    }

    fn get_byte(&mut self) -> u8 {
        self.rng.random()
    }
}
