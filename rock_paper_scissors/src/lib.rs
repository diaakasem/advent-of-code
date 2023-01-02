use elves::elves::Elve;
// use rand::Rng;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    // fn get_random_hand() -> Self {
    //     let mut rng = rand::thread_rng();
    //     let random_number = rng.gen_range(0..3);
    //     match random_number {
    //         0 => Self::Rock,
    //         1 => Self::Paper,
    //         2 => Self::Scissors,
    //         _ => panic!("Invalid random number"),
    //     }
    // }

    fn get_score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Clone)]
pub struct Player {
    pub elve: Elve,
    pub score: i32,
}

impl Player {
    pub fn new(elve: Elve) -> Self {
        Self { elve, score: 0 }
    }

    pub fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    pub fn get_score(&mut self) -> i32 {
        self.score
    }
}

pub struct Game {
    pub player1: Player,
    pub player2: Player,
}

impl Game {
    pub fn new(
        player1: Player,
        player2: Player,
    ) -> Self {
        Self { player1, player2 }
    }

    pub fn play(&mut self, player1_hand: Hand, player2_hand: Hand) {
        const WIN: i32 = 6;
        const TIE: i32 = 3;
        const LOSE: i32 = 0;

        let paper_wins = player1_hand == Hand::Paper && player2_hand == Hand::Rock;
        let rock_wins = player1_hand == Hand::Rock && player2_hand == Hand::Scissors;
        let scissors_wins = player1_hand == Hand::Scissors && player2_hand == Hand::Paper;
        let same_hand = player1_hand == player2_hand;
        let player_1_wins = rock_wins || paper_wins || scissors_wins;

        if same_hand {
            self.player1.add_score(player1_hand.get_score() + TIE);
            self.player2.add_score(player2_hand.get_score() + TIE);
        } else if player_1_wins {
            self.player1.add_score(player1_hand.get_score() + WIN);
            self.player2.add_score(player2_hand.get_score() + LOSE);
        } else {
            self.player1.add_score(player1_hand.get_score() + LOSE);
            self.player2.add_score(player2_hand.get_score() + WIN);
        }
    }

    pub fn get_player2(&self) -> Player {
        self.player2.clone()
    }

    pub fn get_player1(&self) -> Player {
        self.player1.clone()
    }
}

// pub struct Tournament {
//     pub games: Vec<Game>,
// }

// impl Tournament {
//     pub fn new() -> Self {
//         Self { games: Vec::new() }
//     }
//
//     pub fn add_game(&mut self, game: Game) {
//         self.games.push(game);
//     }
//
//     pub fn play_games(&mut self) {
//         self.games.iter_mut().for_each(|game| {
//             let player1_hand = Hand::get_random_hand();
//             let player2_hand = Hand::get_random_hand();
//             game.play(player1_hand, player2_hand);
//         });
//     }
//
//     pub fn get_winner(&self) -> &Player {
//         let mut max_score = 0;
//         let mut winner = &self.games[0].player1;
//         self.games.iter().for_each(|game| {
//             if game.player1.score > max_score {
//                 max_score = game.player1.score;
//                 winner = &game.player1;
//             }
//             if game.player2.score > max_score {
//                 max_score = game.player2.score;
//                 winner = &game.player2;
//             }
//         });
//         winner
//     }
// }
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_paper_scissors_game_works() {
        let player1 = Player::new(Elve::new(1 as u8));
        let player2 = Player::new(Elve::new(2 as u8));
        let mut game = Game::new(player1, player2);
        game.play(Hand::Rock, Hand::Paper);
        game.play(Hand::Paper, Hand::Rock);
        game.play(Hand::Scissors, Hand::Scissors);
        assert_eq!(game.get_player2().get_score(), 15);
        assert_eq!(game.get_player1().get_score(), 15);
    }
}
