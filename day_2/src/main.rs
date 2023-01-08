use clap::Parser;
use elves::elves::Elve;
use ioutils::read_file_lines;
use rock_paper_scissors::{Game, Hand, Player};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// data file path
    #[arg(short, long)]
    data: String,
}

fn mapper(line: &str) -> Hand {
    match line {
        "A" => Hand::Rock,
        "X" => Hand::Rock,
        "B" => Hand::Paper,
        "Y" => Hand::Paper,
        "C" => Hand::Scissors,
        "Z" => Hand::Scissors,
        _ => panic!("Unknown Rock Paper Scissors Hand"),
    }
}

fn pick_hand(opponent_hand: &Hand, lose_win_tie: &str) -> Hand {
    match lose_win_tie {
        "Z" => match opponent_hand {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        },
        "X" => match opponent_hand {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        },
        "Y" => match opponent_hand {
            Hand::Rock => Hand::Rock,
            Hand::Paper => Hand::Paper,
            Hand::Scissors => Hand::Scissors,
        },
        _ => panic!("Unknown Rock Paper Scissors Hand"),
    }
}

fn init_explained_game(lines: &Vec<String>) -> Game {
    let player1 = Player::new(Elve::new());
    let player2 = Player::new(Elve::new());
    let mut game = Game::new(player1, player2);
    lines.iter().for_each(|line| {
        if line.len() == 0 {
            return;
        }
        let hands = line
            .split_whitespace()
            .collect::<Vec<&str>>();
        let opponent_hand = mapper(hands[0]);
        let lose_win_tie = hands[1];
        game.play(opponent_hand, pick_hand(&opponent_hand, lose_win_tie));
    });
    game
}


fn init_game(lines: &Vec<String>) -> Game {
    let player1 = Player::new(Elve::new());
    let player2 = Player::new(Elve::new());
    let mut game = Game::new(player1, player2);
    lines.iter().for_each(|line| {
        if line.len() == 0 {
            return;
        }
        let hands = line
            .split_whitespace()
            .map(|s| mapper(s))
            .collect::<Vec<Hand>>();
        game.play(hands[0], hands[1]);
    });
    game
}

fn main() {
    let args = Args::parse();
    let lines = read_file_lines(&args.data);
    part_1(&lines);
    part_2(&lines);
}

fn part_1(lines: &Vec<String>) {
    let game = init_game(&lines);
    println!("=======================================");
    println!(
        "My expected score with the elve cheatsheet is = {}",
        game.get_player2().get_score()
    );
}

fn part_2(lines: &Vec<String>) {
    let game = init_explained_game(&lines);
    println!("=======================================");
    println!(
        "My expected score with the elve cheatsheet after explaination is = {}",
        game.get_player2().get_score()
    );
}
