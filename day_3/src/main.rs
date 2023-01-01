use rucksacks::{RuckSack, RuckSacksGroup};
use ioutils::read_file_lines;
use clap::Parser;

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
    part_1(&lines);
    part_2(&lines);
}

fn part_1(lines: &Vec<String>) {
    let mut total_waste: u32 = 0;
    for line in lines {
        let mut rucksack = RuckSack::new();
        rucksack.set_data(&line);
        // let duplicates = rucksack.get_duplicates();
        let waste = rucksack.get_duplicates_waste();
        total_waste = total_waste + waste;
        // println!("Total {:?} waste: {}", duplicates, waste);
    }
    println!("Total waste: {}", total_waste);
}

fn part_2(lines: &Vec<String>) {
    let mut total_waste: u32 = 0;
    let mut rucksacks_groups = Vec::<RuckSacksGroup>::new();
    for (i, line) in lines.iter().enumerate() {
        let mut rucksack = RuckSack::new();
        rucksack.set_data(&line);
        if i % 3 == 0 {
            let mut rucksacks_group = RuckSacksGroup::new();
            rucksacks_group.add_rucksack(rucksack);
            rucksacks_groups.push(rucksacks_group);
        } else {
            rucksacks_groups.last_mut().unwrap().add_rucksack(rucksack);
        }
    }

    for mut rucksacks_group in rucksacks_groups {
        let waste = rucksacks_group.get_common_items().iter().map(|i| i.priority as u32).sum::<u32>();
        total_waste = total_waste + waste;
    }
    println!("Total waste: {}", total_waste);
}
