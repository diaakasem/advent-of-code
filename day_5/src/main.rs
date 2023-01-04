use clap::Parser;
use crane::{
    crane::Crane,
    crates::Crate,
    stack::Stack,
};
use ioutils::read_file_lines;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// data file path
    #[arg(short, long)]
    data: String,
}

fn init_stacks(crane_structure: &mut Vec<String>) -> Vec<Stack> {
    crane_structure.reverse();
    // println!("Crane structure: {:?}", crane_structure);
    let stacks_count = ((crane_structure[0].len() as f32 / 4.0) as usize) + 1;
    println!("Crane structure: {:?}", crane_structure[0].len());
    println!("Stacks count: {}", stacks_count);
    let mut stacks = (0..stacks_count).map(|_| Stack::new()).collect::<Vec<Stack>>();
    println!("Stacks length: {:?}", stacks.len());
    crane_structure[1..].iter().enumerate().for_each(|(_, line)| {
        line.chars().collect::<Vec<char>>().chunks(4).enumerate().for_each(|(j, chunk)| {
            if chunk[0] != ' ' {
                let crate_name = chunk[1].to_string();
                stacks[j].push(Crate::new(crate_name));
            }
        });
    });
    stacks
}

fn init_crane(lines: &Vec<String>) -> Crane {
    let mut crane_structure: Vec<String> = Vec::new();
    let mut movements: Vec<String> = Vec::new();
    let mut done_with_structure = false;
    lines.iter().for_each(|line| {
        if line == "" {
            done_with_structure = true;
        } else {
            if done_with_structure {
                movements.push(line.to_string());
            } else {
                crane_structure.push(line.to_string());
            }
        }
    });
    let mut crane = Crane::new();
    crane.stacks = init_stacks(&mut crane_structure);

    movements.iter().for_each(|line| {
        let tokens = line
            .split(" ")
            .collect::<Vec<&str>>();
        let count = tokens[1].parse().unwrap();
        let from: usize = tokens[3].parse().unwrap();
        let to: usize = tokens[5].parse().unwrap();
        // crane.move_blocks(count, from - 1, to - 1);
        crane.move_blocks_9001(count, from - 1, to - 1);
    });
    crane
}

fn main() {
    let args = Args::parse();
    let lines = read_file_lines(&args.data);
    let mut crane = init_crane(&lines);
    part_1(&mut crane);
    // part_2(&pacts);
}

fn part_1(crane: &mut Crane) {
    let names = crane.stacks.iter_mut().map(|s| s.pop().unwrap().name).collect::<Vec<String>>();
    println!("== Part 1 ==");
    println!("Names of the top crates of the stacks = {}", names.join(""));
}

// fn part_2(pacts: &Vec<ElvesPact>) {
//     let count = pacts.iter().map(|pact| match pact.is_cleaning_area_overlapping() {
//         true => 1,
//         false => 0
//     }).sum::<u32>();
//     println!("Number of redundant cleaning areas = {}", count);
// }
