use clap::Parser;
use ioutils::read_all;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// data file path
    #[arg(short, long)]
    data: String,
}

fn main() {
    let args = Args::parse();
    let text = read_all(&args.data);
    part_1(&text);
    part_2(&text);
}

#[derive(Debug)]
struct SignalDelimiter {
    delimiter: String,
    delimiter_len: usize,
}

impl SignalDelimiter {

    fn new(delimiter: &str, delimiter_len: usize) -> Self {
        Self {
            delimiter: delimiter.to_string(),
            delimiter_len,
        }
    }

    fn add_char(&mut self, c: char) {
        if self.delimiter.len() >= self.delimiter_len {
            self.delimiter.remove(0);
        }
        self.delimiter.push(c);
    }

    fn is_delimiter(&self) -> bool {
        if self.delimiter.len() < self.delimiter_len {
            return false;
        }
        !self.delimiter.chars().enumerate()
            .any(|(i, c)| self.delimiter[(i+1)..].chars().any(|d| c == d))
    }
}

/// Detect signal start position
///
fn find_signal_start_index(text: &str) -> usize {
    let mut delimiter = SignalDelimiter::new("", 4);
    let mut index = 0;
    for c in text.chars() {
        delimiter.add_char(c);
        if delimiter.is_delimiter() {
            break;
        }
        index += 1;
    }
    index + 1
}

/// Detect signal start position
///
fn find_message_start_index(text: &str) -> usize {
    let mut delimiter = SignalDelimiter::new("", 14);
    let mut index = 0;
    for c in text.chars() {
        delimiter.add_char(c);
        if delimiter.is_delimiter() {
            break;
        }
        index += 1;
    }
    index + 1
}

fn part_1(text: &str) {
    let i = find_signal_start_index(&text);
    println!("== Part 1 ==");
    println!("Start of signal = {}", i);
}

fn part_2(text: &str) {
    let i = find_message_start_index(&text);
    println!("== Part 2 ==");
    println!("Start of message = {}", i);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_signal_start_index() {
        let i = find_signal_start_index("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(i, 5);
        let i = find_signal_start_index("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(i, 6);
        let i = find_signal_start_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(i, 10);
        let i = find_signal_start_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(i, 11);
    }

    #[test]
    fn test_find_message_start_index() {
        let i = find_message_start_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(i, 19);
        let i = find_message_start_index("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(i, 23);
        let i = find_message_start_index("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(i, 23);
        let i = find_message_start_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(i, 29);
        let i = find_message_start_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(i, 26);
    }
}

