use std::collections::HashSet;

use rand::seq::SliceRandom;

pub struct Game {
    pub board: Vec<u8>,
}

impl Game {
    #[allow(dead_code)]
    pub fn random(size: usize) -> Game {
        let mut board: Vec<u8> = (0..=9).collect();
        while board[0] == 0 {
            board.shuffle(&mut rand::thread_rng());
        }
        Game {
            board: board.drain(..size).collect(),
        }
    }

    #[allow(dead_code)]
    pub fn fixed(board: Vec<u8>) -> Game {
        Game { board }
    }

    #[allow(dead_code)]
    pub fn describe(&self) {
        println!("Answer : {:?}", self.board);
    }

    pub fn play(&self, prediction: &[u8]) -> (u8, u8) {
        let mut bulls = 0;
        let mut cows = 0;

        for (i, &digit) in self.board.iter().enumerate() {
            if digit == prediction[i] {
                bulls += 1;
            } else if prediction.contains(&digit) {
                cows += 1;
            }
        }

        (bulls, cows)
    }

    #[allow(dead_code)]
    pub fn is_valid_submission(&self, submission: &[u8]) -> bool {
        if submission.len() != self.board.len() {
            return false;
        }

        if submission[0] == 0 {
            return false;
        }

        let set: HashSet<&u8> = submission.iter().collect();
        set.len() == submission.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let game = Game::random(4);
        assert_eq!(game.board.len(), 4);
    }

    #[test]
    fn test_fixed() {
        let game = Game::fixed(vec![1, 2, 3, 4]);
        assert_eq!(game.board, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_play() {
        let game = Game::fixed(vec![1, 2, 3, 4]);
        assert_eq!(game.play(&[1, 4, 6, 2]), (1, 2));
    }

    #[test]
    fn test_is_valid_submission() {
        let game = Game::fixed(vec![1, 2, 3, 4]);
        assert!(game.is_valid_submission(&[1, 2, 3, 4]));
        assert!(!game.is_valid_submission(&[1, 2, 3, 3]));
        assert!(!game.is_valid_submission(&[1, 2, 3]));
        assert!(!game.is_valid_submission(&[0, 1, 2, 3]));
    }
}
