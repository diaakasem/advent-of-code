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
        let mut segments = line.split(" ");
        if segments.nth(0).unwrap() == "$" {
            match segments.nth(1) {
                Some(command) => {
                    if command == "cd" {
                        prompt.cd(command);
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
}
