use clap::Parser;
use std::fs::OpenOptions;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// data file path
    #[arg(short, long)]
    data: String,
}

#[derive(Clone)]
struct Meal {
    calories: u32,
}

#[derive(Clone)]
struct Elve {
    id: u8,
    meals: Vec<Meal>,
}

impl Elve {
    fn new(id: u8) -> Self {
        Self {
            id,
            meals: Vec::new(),
        }
    }

    fn add_meal(&mut self, meal: Meal) {
        self.meals.push(meal);
    }

    fn total_calories(&self) -> u32 {
        self.meals.iter().map(|m| m.calories).sum()
    }
}

struct ElvesPact {
    elves: Vec<Elve>,
}

impl ElvesPact {
    fn new() -> Self {
        Self { elves: Vec::new() }
    }

    fn add_elve(&mut self, elve: Elve) {
        self.elves.push(elve);
    }

    fn new_elve(&mut self) {
        self.elves.push(Elve::new(self.elves.len() as u8));
    }

    fn get_elves(&self) -> &Vec<Elve> {
        &self.elves
    }

    fn add_meal_by_calories_to_last_elve(&mut self, calories: u32) {
        let last_elve = self.elves.last_mut().expect("No elves");
        last_elve.add_meal(Meal { calories });
    }

    fn get_elve_with_most_calories(& self) -> &Elve {
        let mut max_calories = 0;
        let mut max_elve = &self.elves[0];
        self.elves.iter().for_each(|elve| {
            if elve.total_calories() > max_calories {
                max_calories = elve.total_calories();
                max_elve = &elve;
            }
        });
        max_elve
    }

    fn get_n_elves_with_most_calories(&self, n: u32) -> ElvesPact {
        let mut elves = self.elves.clone();
        elves.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));
        ElvesPact { elves: elves[0..n as usize].to_vec() }
    }

    fn display_elves(&self) {
        self.get_elves().iter().for_each(|elve| {
            println!("Elve {}", elve.id);
            elve.meals.iter().for_each(|meal| {
                println!("Meal {}", meal.calories);
            });
            println!("Total calories: {}", elve.total_calories());
        });
    }

    fn get_total_calories(&self) -> u32 {
        self.elves.iter().map(|elve| elve.total_calories()).sum()
    }
}

fn read_file_lines(file_path: &str) -> Vec<String> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    contents.lines().map(|s| s.to_string()).collect()
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
