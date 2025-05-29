mod level1;
mod level2;
mod level3;
mod level4;

use level1::Level1Rng;
use level2::Level2Rng;
use level3::Level3Rng;
use level4::Level4Rng;

use crate::{clock::Clock, models::Symbol};

use strum_macros::{EnumCount, FromRepr};

const DEFAULT_TIME_SECS: u64 = 600; // 10 minutes

const DEFAULT_REEL_LAYOUT: [Symbol; 13] = [
    Symbol::Cherry,
    Symbol::Bar,
    Symbol::Seven,
    Symbol::Bar,
    Symbol::MinorJackpot,
    Symbol::Cherry,
    Symbol::DoubleBar,
    Symbol::Bar,
    Symbol::MajorJackpot,
    Symbol::Cherry,
    Symbol::TripleBar,
    Symbol::DoubleBar,
    Symbol::GrandJackpot,
];

#[derive(Clone, Debug)]
pub struct LevelMetadata {
    #[allow(dead_code)]
    pub level_num: u8,
    pub n_wheels: u8,
    pub reel_layout: Vec<Symbol>,
    pub starting_credits: usize,
    pub required_credits: usize,
    pub denomination: u32,
    pub alloted_time: u64,
}

pub trait SlotRng: Send + Sync {
    fn get_metadata(&self) -> LevelMetadata;
    fn get_flag(&self) -> &str;
    fn get_byte(&mut self) -> u8;

    // Update the RNG according to the elapsed time
    fn tick(&mut self) {}

    // Get a bunch of random data to be able to clone the RNG
    fn get_debug_info(&mut self) -> Option<Vec<u8>> {
        None
    }

    fn get_payout(&self, symbol: &Symbol) -> usize {
        match symbol {
            Symbol::Cherry => 3,
            Symbol::Bar => 5,
            Symbol::DoubleBar => 10,
            Symbol::TripleBar => 15,
            Symbol::Seven => 20,
            Symbol::MinorJackpot => 25,
            Symbol::MajorJackpot => 50,
            Symbol::GrandJackpot => 100,
        }
    }
}

#[derive(EnumCount, FromRepr)]
#[repr(usize)]
pub enum SlotLevel {
    Level1 = 1,
    Level2,
    Level3,
    Level4,
}

impl SlotLevel {
    pub fn get_rng(&self, clock: &Clock) -> Box<dyn SlotRng> {
        match self {
            SlotLevel::Level1 => Box::new(Level1Rng::new()),
            SlotLevel::Level2 => Box::new(Level2Rng::new(clock)),
            SlotLevel::Level3 => Box::new(Level3Rng::new()),
            SlotLevel::Level4 => Box::new(Level4Rng::new()),
        }
    }
}
