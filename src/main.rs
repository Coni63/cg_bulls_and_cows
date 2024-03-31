mod game;
mod solver;

use std::time::Instant;

use game::Game;
use solver::Solver;

fn solve_game(size: usize) {
    let game = Game::random(size);
    game.describe();

    let mut solver = Solver::new(size);

    loop {
        let pred = solver.get();
        let (b, c) = game.play(&pred);
        eprintln!("Prediction: {:?}, Bulls: {}, Cows: {}", pred, b, c);
        if b == size as u8 {
            println!("Found the answer: {:?}", pred);
            break;
        }
        solver.update(pred, b, c);
    }
}

fn benchmark_update() {
    for size in 6..=7 {
        println!("Size: {}", size);
        let mut times: Vec<u128> = Vec::new();

        for _ in 1..10 {
            let game = Game::random(size);
            let mut solver = Solver::new(size);

            let pred = solver.get();
            let (b, c) = game.play(&pred);
            if b == size as u8 {
                break;
            }

            let start = Instant::now();
            // solver.update(pred, b, c);
            solver.update(pred, b, c);
            times.push(start.elapsed().as_micros());
        }

        let avg: f64 = times.iter().sum::<u128>() as f64 / times.len() as f64;
        println!("Average time: {} μs", avg);
    }
}

fn main() {
    // solve_game(4_usize);
    benchmark_update();
}
