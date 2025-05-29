mod mt19937;
use mt19937::Mt19937Rng;

use rand::{Rng, RngCore, SeedableRng};

use super::{SlotRng, DEFAULT_REEL_LAYOUT, DEFAULT_TIME_SECS};

pub struct Level3Rng {
    rng: Mt19937Rng,
}

impl Level3Rng {
    pub fn new() -> Self {
        // Level 3: Clone MT19937 state
        Self {
            rng: Mt19937Rng::from_os_rng(),
        }
    }
}

impl SlotRng for Level3Rng {
    fn get_metadata(&self) -> super::LevelMetadata {
        super::LevelMetadata {
            level_num: 3,
            n_wheels: 3,
            reel_layout: DEFAULT_REEL_LAYOUT.to_vec(),
            starting_credits: 20_000,
            required_credits: 400_000,
            denomination: 400,
            alloted_time: DEFAULT_TIME_SECS,
        }
    }

    fn get_flag(&self) -> &str {
        #[cfg(feature = "prod")]
        return include_str!("../flags/FLAG3.txt");
        #[cfg(not(feature = "prod"))]
        return include_str!("../flags/FLAG3-DEBUG.txt");
    }

    fn get_byte(&mut self) -> u8 {
        self.rng.random()
    }

    // This is the meat of the challenge
    // We dump the full state
    fn get_debug_info(&mut self) -> Option<Vec<u8>> {
        let mut debug_info = vec![0; 800 * size_of::<u32>()];

        self.rng.fill_bytes(&mut debug_info);

        Some(debug_info)
    }
}
