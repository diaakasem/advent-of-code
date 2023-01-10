mod traits;
mod knot;


use std::{sync::Mutex, cell::RefCell, rc::Rc};

use clap::Parser;
use ioutils::read_file_lines;
use knot::Knot;
use traits::{MovePublisher, Direction, Movable, Point, MoveSubscriber};

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
    let positions = part_1(&lines);
    println!("part 1: {}", positions);
    let positions = part_2(&lines);
    println!("part 2: {}", positions);
}

fn part_1(input: &Vec<String>) -> usize {
    let mut head = Knot::new("head");
    let tail = Knot::new("tail");
    head.subscribe(Rc::new(RefCell::new(tail)));
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
    let t = head.subscribers().first().unwrap().borrow().get_points().len();
    t
}

fn part_2(input: &Vec<String>) -> usize {
    let head = Rc::new(RefCell::new(Knot::new("head")));
    let h1 = Rc::new(RefCell::new(Knot::new("h1")));
    let h2 = Rc::new(RefCell::new(Knot::new("h2")));
    let h3 = Rc::new(RefCell::new(Knot::new("h3")));
    let h4 = Rc::new(RefCell::new(Knot::new("h4")));
    let h5 = Rc::new(RefCell::new(Knot::new("h5")));
    let h6 = Rc::new(RefCell::new(Knot::new("h6")));
    let h7 = Rc::new(RefCell::new(Knot::new("h7")));
    let h8 = Rc::new(RefCell::new(Knot::new("h8")));
    let h9 = Rc::new(RefCell::new(Knot::new("h9")));
    head.borrow_mut().subscribe(h1.clone());
    h1.borrow_mut().subscribe(h2.clone());
    h2.borrow_mut().subscribe(h3.clone());
    h3.borrow_mut().subscribe(h4.clone());
    h4.borrow_mut().subscribe(h5.clone());
    h5.borrow_mut().subscribe(h6.clone());
    h6.borrow_mut().subscribe(h7.clone());
    h7.borrow_mut().subscribe(h8.clone());
    h8.borrow_mut().subscribe(h9.clone());
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
        head.borrow_mut().move_towards(&direction, steps);
    }
    let t = h9.borrow().get_points().len();
    t
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
        let result = part_1(&input.iter().map(|s| s.to_string()).collect());
        assert_eq!(result, 13);
    }
}

