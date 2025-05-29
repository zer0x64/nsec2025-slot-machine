use std::{io, ops::RangeInclusive};

use anyhow::Result;
use itertools::Itertools;
use rayon::prelude::*;

fn range_from_symbol(char: &char) -> RangeInclusive<u8> {
    match char {
        'c' => 0..=127,
        '1' => 128..=191,
        '2' => 192..=217,
        '3' => 218..=239,
        '7' => 240..=248,
        '-' => 249..=252,
        '+' => 253..=254,
        '!' => 255..=255,
        _ => panic!("Unknown symbol!"), // Default case for any unhandled inputs
    }
}

fn symbol_from_num(value: u8) -> &'static str {
    match value {
        0..128 => "c",
        128..192 => "1",
        192..218 => "2",
        218..240 => "3",
        240..249 => "7",
        249..253 => "-",
        253..255 => "+",
        255 => "!",
    }
}

fn main() -> Result<()> {
    println!("Spin a bit until you get a few rare hits and write it here:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let mut ranges: Vec<RangeInclusive<u8>> = buffer
        .replace("\n", "")
        .replace(" ", "")
        .chars()
        .map(|c| range_from_symbol(&c))
        .collect();

    println!("Trying to find the best starting point.");

    let (best_start, n_seeds) = ranges
        .chunks_exact(5)
        .enumerate()
        .map(|(i, ranges)| {
            let mut prod: usize = (ranges[0].end() - ranges[0].start()) as usize;
            for r in &ranges[1..] {
                prod *= (r.end() - r.start()) as usize;
            }

            (i, prod)
        })
        .min_by(|(_, x), (_, y)| x.cmp(y))
        .expect("No valid start!");

    println!(
        "Starting at index {}, there is {} possible seeds",
        best_start * 5,
        n_seeds
    );

    ranges = ranges[best_start * 5..].to_vec();

    println!("Bruteforce begins...");

    let mut potential_seeds: Vec<u64> = ranges[0]
        .clone()
        .cartesian_product(ranges[1].clone())
        .cartesian_product(ranges[2].clone())
        .cartesian_product(ranges[3].clone())
        .cartesian_product(ranges[4].clone())
        .par_bridge()
        .filter_map(|((((s0, s1), s2), s3), s4)| {
            let seed = ((s0 as u64)
                | ((s1 as u64) << 8)
                | ((s2 as u64) << 16)
                | ((s3 as u64) << 24)
                | ((s4 as u64) << 32))
                ^ 0xf267bcb3b2;

            let mut rng = SimpleLfsr(seed, 5);

            for r in ranges[5..].iter() {
                if !r.contains(&(rng.next() as u8)) {
                    return None;
                }
            }

            Some(seed)
        })
        .collect();

    if potential_seeds.len() <= 0 {
        println!("No seed found! Something didn't work well");

        return Ok(());
    } else if potential_seeds.len() == 1 {
        println!("Found the seed!");
        predict(&potential_seeds[0..1], ranges.len() - 1);

        return Ok(());
    }

    println!("Multiple seed found! Add more spins so I can find it...");

    loop {
        println!("Enter a few spins:");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        if buffer.contains("break") {
            break;
        }

        let mut new_ranges: Vec<RangeInclusive<u8>> = buffer
            .replace("\n", "")
            .replace(" ", "")
            .chars()
            .map(|c| range_from_symbol(&c))
            .collect();

        ranges.append(&mut new_ranges);

        potential_seeds = potential_seeds
            .into_iter()
            .par_bridge()
            .filter(|s| {
                let mut rng = SimpleLfsr(*s, 5);

                for r in ranges[5..].iter() {
                    if !r.contains(&(rng.next() as u8)) {
                        // The seed can be discarded
                        return false;
                    }
                }

                return true;
            })
            .collect();

        if potential_seeds.len() <= 1 {
            break;
        }

        println!("{} seeds left!", potential_seeds.len())
    }

    if potential_seeds.len() <= 0 {
        println!("No seed found! Something didn't work well");
    } else if potential_seeds.len() == 1 {
        println!("Found the seed!");
        predict(&potential_seeds[0..1], ranges.len() - 5);
    } else {
        println!("Called break with multiple seeds!");
        println!("Will run them all in parrallels for the prediction");

        predict(&potential_seeds, ranges.len() - 5);
    }

    Ok(())
}

fn predict(seeds: &[u64], num_gen: usize) {
    println!("Using seeds {:X?}", seeds);

    let mut rngs: Vec<SimpleLfsr> = seeds
        .into_iter()
        .map(|s| {
            let mut rng = SimpleLfsr(*s, 5);
            for _ in 0..num_gen {
                let _ = rng.next();
            }

            rng
        })
        .collect();

    let mut num_wins = 0;
    let mut n_spins = 0;

    while num_wins < 20 {
        n_spins += 1;

        let x = symbol_from_num(rngs[0].next() as u8);
        let y = symbol_from_num(rngs[0].next() as u8);
        let z = symbol_from_num(rngs[0].next() as u8);

        let mut equals = x == y && x == z;

        for r in &mut rngs[1..] {
            let x2 = symbol_from_num(r.next() as u8);
            let y2 = symbol_from_num(r.next() as u8);
            let z2 = symbol_from_num(r.next() as u8);

            equals &= x == x2 && x == y2 && x == z2
        }

        if equals {
            println!("Spin {}: {}", n_spins, x);
            num_wins += 1;
        }
    }
}

// Code copied from the challenge, it should be pretty easy to reverse
pub struct SimpleLfsr(pub u64, pub usize);

impl SimpleLfsr {
    fn next(&mut self) -> u32 {
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
}
