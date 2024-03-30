mod game;
mod solver;

use std::vec;

use game::Game;
use solver::Solver;

fn main() {
    let size: u8 = 4;
    // let game = Game::random(4);
    let game = Game::fixed(vec![1, 2, 3, 4]);
    game.describe();

    let mut solver = Solver::new(size);

    loop {
        let pred = solver.get();
        let (b, c) = game.play(&pred);
        eprintln!("Prediction: {:?}, Bulls: {}, Cows: {}", pred, b, c);
        if b == size {
            println!("Found the answer: {:?}", pred);
            break;
        }
        solver.update(pred, b, c);
    }
}
