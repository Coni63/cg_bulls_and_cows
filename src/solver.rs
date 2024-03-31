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
    possibilities_to_sort: Vec<Vec<u8>>,
    possibilities_sorted: Vec<Vec<u8>>,
    sorted_idx: usize,
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
            possibilities_to_sort: possibilities,
            possibilities_sorted: Vec::new(),
            rng: rand::thread_rng(),
            sorted_idx: 0,
        }
    }

    pub fn get(&mut self) -> Vec<u8> {
        if !self.possibilities_sorted.is_empty() {
            let idx: usize = self.rng.gen_range(0..self.possibilities_sorted.len());
            self.possibilities_sorted.get(idx).unwrap().clone()
        } else {
            let idx: usize = self.rng.gen_range(0..self.possibilities_to_sort.len());
            self.possibilities_to_sort.get(idx).unwrap().clone()
        }
    }

    pub fn update(&mut self, prediction: Vec<u8>, bulls: u8, cows: u8) {
        let mut used_digits = [false; 10];
        for &digit in prediction.iter() {
            used_digits[digit as usize] = true;
        }

        let starting_time = Instant::now();

        // filter out possibilities that are not consistent with the prediction
        self.possibilities_sorted
            .retain(|candidate| is_candidate(candidate, &prediction, &used_digits, bulls, cows));

        // for the remaining time, sort the possibilities_to_sort
        while starting_time.elapsed().as_millis() < 40 {
            for _ in 0..1000 {
                if let Some(candidate) = self.possibilities_to_sort.get(self.sorted_idx) {
                    if is_candidate(candidate, &prediction, &used_digits, bulls, cows) {
                        self.possibilities_sorted.push(candidate.clone());
                    }
                    self.sorted_idx += 1;
                }
            }
        }

        // eprintln!("Remaining possibilities: {}", self.possibilities.len());
    }
}
