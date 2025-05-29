use clap::Parser;
use rand::{Rng, RngCore, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const STARTUP_CASH: usize = 50;
const OBJECTIVE: usize = 1000;
const MAX_SPIN: usize = 200;
const N_SIMULATIONS: u64 = 100_000_000;
const MARTINGALE_BASE_BET: usize = 10;
const ADAPTATIVE_FRACTION: f64 = 0.20;
const CONSTANT_FRACTION: f64 = 0.025;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t = STARTUP_CASH)]
    start_cash: usize,
    #[arg(short, long, default_value_t = OBJECTIVE)]
    objective: usize,
    #[arg(short, long, default_value_t = MAX_SPIN)]
    max_spin: usize,
    #[arg(short, long, default_value_t = N_SIMULATIONS)]
    n_simulations: u64,
    #[arg(long, value_parser, num_args = 0.., value_delimiter = ',')]
    strategies: Vec<String>,
    #[arg(long, default_value_t = MARTINGALE_BASE_BET)]
    martingale_base_bet: usize,
    #[arg(long, default_value_t = ADAPTATIVE_FRACTION)]
    adaptative_fraction: f64,
    #[arg(long, default_value_t = CONSTANT_FRACTION)]
    constant_fraction: f64,
}

fn main() {
    let args = Cli::parse();

    let strategies = if args.strategies.is_empty() {
        &vec![
            "allin".to_owned(),
            "adaptative".to_owned(),
            "constant".to_owned(),
            "martingale".to_owned(),
        ]
    } else {
        &args.strategies
    };

    for s in strategies {
        match s.to_lowercase().as_ref() {
            "allin" => run_strategy(AllIn, &args),
            "adaptative" => run_strategy(Adaptative::new(args.adaptative_fraction), &args),
            "constant" => run_strategy(Constant::new(args.constant_fraction), &args),
            "martingale" => run_strategy(Martingale::new(args.martingale_base_bet), &args),
            _ => println!("Unknown strategy: {}", s),
        }
    }
}

fn run_strategy<T>(strategy: T, args: &Cli)
where
    T: Strategy,
{
    let wins: Vec<usize> = (0..N_SIMULATIONS)
        .into_par_iter()
        .filter_map(|seed| Simulator::new(strategy.clone(), seed, args).run())
        .collect();

    let n_wins = wins.len();
    let avg_spins = wins.into_iter().sum::<usize>() as f64 / n_wins as f64;
    let win_probability = n_wins as f64 / N_SIMULATIONS as f64;

    println!(
        "===================================\nStrategy: {:?}\n\nGot {} wins!\nWin probability: {:.2}%\nAverage spins per win: {:.2}\n",
        strategy,
        n_wins,
        win_probability * 100.0,
        avg_spins
    );
}

#[derive(PartialEq, Eq)]
pub enum Symbol {
    Cherry,
    Bar,
    DoubleBar,
    TripleBar,
    Seven,
    MinorJackpot,
    MajorJackpot,
    GrandJackpot,
}

impl From<u8> for Symbol {
    fn from(value: u8) -> Self {
        match value {
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

impl Symbol {
    fn get_payout(&self) -> usize {
        match self {
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

trait Strategy: Send + Sync + Clone + std::fmt::Debug {
    fn get_bet_value(
        &self,
        current_cash: usize,
        objective: usize,
        previous_bet: Option<(usize, bool)>,
    ) -> usize;
}

#[derive(Clone, Debug)]
struct Martingale {
    base_bet: usize,
}

impl Martingale {
    fn new(base_bet: usize) -> Self {
        Self { base_bet }
    }
}

impl Strategy for Martingale {
    fn get_bet_value(
        &self,
        current_cash: usize,
        objective: usize,
        previous_bet: Option<(usize, bool)>,
    ) -> usize {
        let next_bet = match previous_bet {
            // First bet or previous bet was a win
            None | Some((_, true)) => self.base_bet,
            // After a loss, double the previous bet
            Some((prev, false)) => prev * 2,
        };

        // Ensure we don't bet more than we have and don't overshoot our objective
        next_bet
            .min(current_cash)
            .min((objective - current_cash) / 2 + 1)
    }
}

#[derive(Clone, Debug)]
struct AllIn;

impl Strategy for AllIn {
    fn get_bet_value(
        &self,
        current_cash: usize,
        objective: usize,
        previous_bet: Option<(usize, bool)>,
    ) -> usize {
        let _ = previous_bet;

        current_cash.min((objective - current_cash) / 2 + 1)
    }
}

#[derive(Clone, Debug)]
struct Adaptative {
    fraction: f64,
}

impl Adaptative {
    fn new(fraction: f64) -> Self {
        Adaptative { fraction }
    }
}

impl Strategy for Adaptative {
    fn get_bet_value(
        &self,
        current_cash: usize,
        objective: usize,
        previous_bet: Option<(usize, bool)>,
    ) -> usize {
        let _ = previous_bet;

        (((current_cash as f64) * self.fraction).ceil() as usize)
            .min((objective - current_cash) / 2 + 1)
            .min(current_cash)
    }
}

#[derive(Clone, Debug)]
struct Constant {
    fraction: f64,
}

impl Constant {
    fn new(fraction: f64) -> Self {
        Constant { fraction }
    }
}

impl Strategy for Constant {
    fn get_bet_value(
        &self,
        current_cash: usize,
        objective: usize,
        previous_bet: Option<(usize, bool)>,
    ) -> usize {
        let _ = previous_bet;

        ((objective as f64 * self.fraction).ceil() as usize)
            .min((objective - current_cash) / 2 + 1)
            .min(current_cash)
    }
}

struct Simulator<T: Strategy> {
    strategy: T,
    rng: SimpleLfsr,
    current_cash: usize,
    objective: usize,
    max_spins: usize,
}

impl<T> Simulator<T>
where
    T: Strategy,
{
    fn new(strategy: T, seed: u64, args: &Cli) -> Self {
        let rng = SimpleLfsr::seed_from_u64(seed);

        Self {
            strategy,
            rng,
            current_cash: args.start_cash,
            objective: args.objective,
            max_spins: args.max_spin,
        }
    }

    fn run(&mut self) -> Option<usize> {
        let mut current_spin = 0;
        let mut previous_bet: Option<(usize, bool)> = None;

        loop {
            let bet = self
                .strategy
                .get_bet_value(self.current_cash, self.objective, previous_bet);
            self.current_cash -= bet;

            let payout = {
                let x = <Symbol as From<u8>>::from(self.rng.random());
                let y = <Symbol as From<u8>>::from(self.rng.random());
                let z = <Symbol as From<u8>>::from(self.rng.random());

                if x == y && x == z { x.get_payout() } else { 0 }
            };

            let win = payout > 0;
            self.current_cash += payout * bet;

            // Record this bet and whether it was a win
            previous_bet = Some((bet, win));

            current_spin += 1;

            if self.current_cash == 0 {
                break None;
            } else if self.current_cash >= self.objective {
                break Some(current_spin);
            } else if current_spin > self.max_spins {
                break None;
            }
        }
    }
}

pub struct SimpleLfsr(u64, usize);

impl SeedableRng for SimpleLfsr {
    type Seed = [u8; 8];

    fn from_seed(seed: Self::Seed) -> Self {
        Self(u64::from_ne_bytes(seed), 0)
    }
}

impl RngCore for SimpleLfsr {
    fn next_u32(&mut self) -> u32 {
        if self.1 >= 5 {
            self.1 = 0;

            // Masks so it's 40 bits instead of 64
            self.0 ^= self.0 << 10;
            self.0 &= 0xFFFFFFFFFF;

            self.0 ^= self.0 >> 26;
            self.0 &= 0xFFFFFFFFFF;
        }

        // Mask with a random constant for added difficulty
        let result = (self.0 ^ 0xf267bcb3b2) >> (self.1 * 8);

        self.1 += 1;
        result as u32
    }

    fn next_u64(&mut self) -> u64 {
        rand::rand_core::impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand::rand_core::impls::fill_bytes_via_next(self, dest);
    }
}
