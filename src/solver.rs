use rand::{seq::SliceRandom, Rng};
use std::collections::HashSet;
use std::time::Instant;

use itertools::Itertools;

fn generate_possibilities(length: usize) -> Vec<Vec<u8>> {
    let digits: Vec<u8> = (0..=9).collect();
    digits
        .into_iter()
        .permutations(length)
        .filter_map(|p| {
            // ensure no leading zeros except for the single digit '0' case
            if p[0] == 0 {
                None
            } else {
                Some(p.into_iter().collect())
            }
        })
        .collect()
}

fn is_candidate(
    candidate: &[u8],
    prediction: &[u8],
    used_digits: &[bool],
    bulls: u8,
    cows: u8,
) -> bool {
    let mut b = 0;
    let mut c = 0;
    for (i, &digit) in candidate.iter().enumerate() {
        if digit == prediction[i] {
            b += 1;
        } else if used_digits[digit as usize] {
            c += 1;
        }
    }
    b == bulls && c == cows
}

pub struct Solver {
    possibilities: Vec<Vec<u8>>,
    rng: rand::rngs::ThreadRng,
}

impl Solver {
    pub fn new(length: usize) -> Self {
        // let start = Instant::now();
        let possibilities = generate_possibilities(length);
        // eprintln!(
        //     "Generated {} permutations in {:?}",
        //     possibilities.len(),
        //     start.elapsed()
        // );

        Self {
            possibilities,
            rng: rand::thread_rng(),
        }
    }

    pub fn get(&mut self) -> Vec<u8> {
        let idx: usize = self.rng.gen_range(0..self.possibilities.len());
        self.possibilities.get(idx).unwrap().clone()
    }

    pub fn update(&mut self, prediction: Vec<u8>, bulls: u8, cows: u8) {
        let mut used_digits = [false; 10];
        for &digit in prediction.iter() {
            used_digits[digit as usize] = true;
        }

        self.possibilities
            .retain(|candidate| is_candidate(candidate, &prediction, &used_digits, bulls, cows));

        // eprintln!("Remaining possibilities: {}", self.possibilities.len());
    }
}
