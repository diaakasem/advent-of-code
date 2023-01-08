use clap::Parser;
use ioutils::read_file_lines;
use os::shell::sh::Prompt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// data file path
    #[arg(short, long)]
    data: String,
}

fn init_prompt(lines: Vec<String>) -> Prompt {
    let mut prompt = Prompt::new();
    for line in lines.iter() {
        // println!("======================================================");
        // println!("line: {}", line);
        let segments = line.split(" ").collect::<Vec<_>>();
        if segments.get(0).unwrap().to_string() == "$".to_string() {
            match segments.get(1) {
                Some(command) => {
                    // println!("command: {}", command);
                    if command.to_string() == "cd".to_string() {
                        let path = segments.get(2).unwrap().to_string();
                        // println!("path: {}", path);
                        prompt.cd(path.as_str());
                    } else {
                        continue;
                    }
                },
                _ => continue
            }
        } else {
            prompt.parse_line(&line)
        }
    }
    prompt
}

fn main() {
    let args = Args::parse();
    let lines = read_file_lines(&args.data);
    let prompt = init_prompt(lines);
    prompt.tree();
    println!("======================================================");
    part_1(&prompt)
}

fn part_1(prompt: &Prompt) {
    let dirs = prompt.get_dir_less_than(100000);
    dirs.iter().for_each(|(path, size)| {
        println!("{} {}", path, size);
    });
    println!("dirs: {:?}", dirs.values().sum::<u64>());
}
