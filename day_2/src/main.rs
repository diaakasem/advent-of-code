use clap::Parser;
use elves::ElvesPact;
use ioutils::read_file_lines;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// data file path
    #[arg(short, long)]
    data: String,
}

fn init_elves(lines: &Vec<String>) -> ElvesPact {
    let mut elves = ElvesPact::new();
    let mut i = 0;
    lines.iter().for_each(|line| {
        if line.len() > 0 {
            if i == 0 {
                elves.new_elve();
            }
            let calories = line.parse::<u32>().expect("Not a number");
            elves.add_meal_by_calories_to_last_elve(calories);
        } else {
            elves.new_elve();
        }
        i += 1;
    });
    elves
}

fn main() {
    let args = Args::parse();
    let lines = read_file_lines(&args.data);
    let elves = init_elves(&lines);
    elves.display_elves();
    println!("=======================================");
    let elves = elves.get_n_elves_with_most_calories(3);
    println!("Elves with the most calories = {} calories", elves.get_total_calories());
}
