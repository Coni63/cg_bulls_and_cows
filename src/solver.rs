use rand::seq::SliceRandom;
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

pub struct Solver {
    possibilities: Vec<Vec<u8>>,
    rng: rand::rngs::ThreadRng,
}

impl Solver {
    pub fn new(length: u8) -> Self {
        let start = Instant::now();
        let possibilities = generate_possibilities(length as usize);
        eprintln!(
            "Generated {} permutations in {:?}",
            possibilities.len(),
            start.elapsed()
        );

        Self {
            possibilities,
            rng: rand::thread_rng(),
        }
    }

    pub fn get(&mut self) -> Vec<u8> {
        self.possibilities.shuffle(&mut self.rng);
        self.possibilities[0].clone()
    }

    pub fn update(&mut self, prediction: Vec<u8>, bulls: u8, cows: u8) {
        self.possibilities = self
            .possibilities
            .iter()
            .filter(|&p| {
                let mut b = 0;
                let mut c = 0;
                for (i, &digit) in p.iter().enumerate() {
                    if digit == prediction[i] {
                        b += 1;
                    } else if prediction.contains(&digit) {
                        c += 1;
                    }
                }
                b == bulls && c == cows
            })
            .map(|p| p.clone())
            .collect();

        eprintln!("Remaining possibilities: {}", self.possibilities.len());
    }
}
