use forest::{TreeGrid, Tree};
use clap::Parser;
use ioutils::read_file_lines;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// data file path
    #[arg(short, long)]
    data: String,
}

fn init_forest(lines: Vec<String>) -> TreeGrid {
    let mut forest = TreeGrid::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let height = c.to_digit(10).unwrap();
            let tree = Tree::new(height, col as u32, row as u32);
            forest.add_tree(tree);
        }
    }
    forest
}

fn main() {
    let args = Args::parse();
    let lines = read_file_lines(&args.data);
    let forest = init_forest(lines);
    // part_1(&forest);
    part_2(&forest);
}

fn part_1(forest: &TreeGrid) {
    println!("Part 1: Count of visible trees is {}", forest.get_visible_trees_count());
}

fn part_2(forest: &TreeGrid) {
    println!("Part 2: get max scenic view is {}", forest.get_max_scenic_view());
}

