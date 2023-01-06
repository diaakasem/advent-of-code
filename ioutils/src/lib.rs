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

pub fn read_all(file_path: &str) -> String {
    let mut file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("File not found");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Something went wrong reading the file");
    buf
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines = read_file_lines("data/test.txt");
        assert_eq!(lines, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_read_all() {
        let buf = read_all("data/test.txt");
        assert_eq!(buf, "1\n2\n3\n");
    }

}
