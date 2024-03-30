mod game;

use std::vec;

use game::Game;

fn main() {
    // let game = Game::random(4);
    let game = Game::fixed(vec![1, 2, 3, 4]);
    game.describe();

    let pred = vec![1, 4, 6, 2];

    let (bulls, cows) = game.play(pred);
    eprintln!("Bulls: {}, Cows: {}", bulls, cows);
}
