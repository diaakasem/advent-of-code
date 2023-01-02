use clap::Parser;
use elves::cleaning_area::CleaningArea;
use elves::elves::{Elve, ElvesPact};
use ioutils::read_file_lines;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// data file path
    #[arg(short, long)]
    data: String,
}

fn init_pacts(lines: &Vec<String>) -> Vec<ElvesPact> {
    lines.iter().map(|line| {
        let mut pact = ElvesPact::new();
        let elves = line
            .split(",")
            .map(|s| {
                let start_end = s
                    .split("-")
                    .map(|s| s.parse::<u32>().expect("Not a number"))
                    .collect::<Vec<u32>>();
                let cleaning_area = CleaningArea::new(start_end[0], start_end[1]);
                let mut e = Elve::new(0);
                e.set_cleaning_area(cleaning_area);
                e
            })
            .collect::<Vec<Elve>>();
        pact.add_elves(elves);
        pact
    })
    .collect::<Vec<ElvesPact>>()
}

fn main() {
    let args = Args::parse();
    let lines = read_file_lines(&args.data);
    let pacts = init_pacts(&lines);
    part_1(pacts);
}

fn part_1(pacts: Vec<ElvesPact>) {
    let count = pacts.iter().map(|pact| match pact.is_cleaning_area_redundant() {
        true => 1,
        false => 0
    }).sum::<u32>();
    println!("Number of redundant cleaning areas = {}", count);
}

// fn part_2(elves: &mut ElvesPact) {
//     let elves = elves.get_n_elves_with_most_calories(3);
//     println!(
//         "Elves with the most calories = {} calories",
//         elves.get_total_calories()
//     );
// }
