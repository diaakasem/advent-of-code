mod traits;
mod head;
mod tail;


use clap::Parser;
use ioutils::read_file_lines;
use head::Head;
use tail::Tail;
use traits::{MovePublisher, Direction, Movable, Point};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// data file path
    #[arg(short, long)]
    data: String,
}


fn main() {
    let args = Args::parse();
    let lines = read_file_lines(&args.data);
    // part_1(&forest);
    let positions = part_1(lines);
    println!("part 1: {}", positions);
}

fn part_1(input: Vec<String>) -> usize {
    let mut head = Head::new();
    let tail = Tail::new();
    head.subscribe(Box::new(tail));
    for line in input {
        let mut dir_steps = line.split_whitespace();
        let dir = dir_steps.next().unwrap();
        let steps = dir_steps.next().unwrap().parse::<i32>().unwrap();
        let direction = match dir {
            "R" => Direction::R,
            "L" => Direction::L,
            "U" => Direction::U,
            "D" => Direction::D,
            _ => panic!("Unknown direction"),
        };
        head.move_towards(&direction, steps);
    }
    let tail = head.subscribers().first().unwrap();
    tail.get_points().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
        let result = part_1(input);
        assert_eq!(result, 13);
    }
}

