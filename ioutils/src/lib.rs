use std::fs::OpenOptions;
use std::io::Read;

pub fn read_file_lines(file_path: &str) -> Vec<String> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    contents.lines().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
