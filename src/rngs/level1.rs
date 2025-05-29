use rand::{rngs::StdRng, Rng, SeedableRng};

use super::{SlotRng, DEFAULT_REEL_LAYOUT, DEFAULT_TIME_SECS};

pub struct Level1Rng {
    rng: StdRng,
}

impl Level1Rng {
    pub fn new() -> Self {
        // Level 1: Hardcoded seed
        Self {
            rng: StdRng::seed_from_u64(0),
        }
    }
}

impl SlotRng for Level1Rng {
    fn get_metadata(&self) -> super::LevelMetadata {
        super::LevelMetadata {
            level_num: 1,
            n_wheels: 3,
            reel_layout: DEFAULT_REEL_LAYOUT.to_vec(),
            starting_credits: 50,
            required_credits: 1_000,
            denomination: 1,
            alloted_time: DEFAULT_TIME_SECS,
        }
    }

    fn get_flag(&self) -> &str {
        #[cfg(feature = "prod")]
        return include_str!("./flags/FLAG1.txt");
        #[cfg(not(feature = "prod"))]
        return include_str!("./flags/FLAG1-DEBUG.txt");
    }

    fn get_byte(&mut self) -> u8 {
        self.rng.random()
    }
}
