use std::{
    borrow::{Borrow},
    cell::RefCell,
    rc::Rc,
};

use super::file::File;

#[derive(Debug, Clone)]
pub struct Dir {
    pub name: String,
    pub files: Vec<RefCell<File>>,
    pub dirs: Vec<RefCell<Dir>>,
    pub parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    pub fn new(name: &str, parent: Option<Rc<RefCell<Self>>>) -> Self {
        let name = name.to_string();
        let files: Vec<RefCell<File>> = vec![];
        let dirs: Vec<RefCell<Self>> = vec![];
        Self {
            name,
            files,
            dirs,
            parent: match parent {
                Some(parent) => Some(parent),
                None => None,
            },
        }
    }

    pub fn add_file(self: &mut Self, file: File) {
        self.files.push(RefCell::new(file));
    }

    pub fn add_dir(self: &mut Self, dir: Self) {
        self.dirs.push(RefCell::new(dir))
    }

    pub fn pwd(self: &Self, mut p: String) -> String {
        if let Some(parent) = &self.parent {
            p = format!("{}/{}", self.name, p);
            match parent.try_borrow() {
                Ok(parent) => parent.pwd(p),
                Err(_) => p,
            }
        } else {
            format!("/{}/{}", self.name, p)
        }
    }

    pub fn get_size(self: &Self) -> u64 {
        let mut size: u64 = 0;
        for file in &self.files {
            size += file.borrow().size;
            // println!("  |-> [FIL] {} \t\t {}", file.borrow().name, size);
        }
        for dir in &self.dirs {
            size += dir.borrow().get_size();
            // println!("  |-> [DIR] {} \t\t {}", dir.borrow().name, size);
        }
        size
    }

    pub fn tree(self: &Self, level: u8) {
        let tabs = "\t".repeat(level as usize);
        println!(
            "{}|-> [DIR] {} {}",
            tabs,
            self.pwd("".to_string()),
            self.get_size()
        );
        for dir in &self.dirs {
            dir.borrow().tree(level + 1);
        }
        // for file in &self.files {
        // println!("{}|-> [FIL] {} {}", tabs, file.borrow().name, file.borrow().size);
        // }
    }

    pub fn get_child_dir(self: &Self, name: &str) -> Option<Rc<RefCell<Self>>> {
        let res = self.dirs.iter().find(|dir| (*dir).borrow().name == name);
        match res {
            Some(dir) => Some(Rc::new(dir.clone())),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dir() {
        // use super::super::file::File;
        // use super::Dir;
        // let mut dir = Dir::new("root");
        // let mut dir1 = Dir::new("dir1");
        // let mut dir2 = Dir::new("dir2");
        // let mut dir3 = Dir::new("dir3");
        // dir1.add_file(File::new("file1", Some(1)));
        // dir1.add_file(File::new("file2", Some(2)));
        // dir2.add_file(File::new("file3", Some(3)));
        // dir2.add_file(File::new("file4", Some(4)));
        // dir3.add_file(File::new("file5", Some(5)));
        // dir3.add_file(File::new("file6", Some(6)));
        // dir.add_dir(&dir1);
        // dir.add_dir(&dir2);
        // dir.add_dir(&dir3);
        // assert_eq!(dir.get_size(), 21);
        // assert_eq!(dir1.get_size(), 3);
        // assert_eq!(dir2.get_size(), 7);
        // assert_eq!(dir3.get_size(), 11);
        // assert_eq!(dir.get_child_dir("dir1").unwrap().get_size(), 3);
        // assert_eq!(dir.get_child_dir("dir2").unwrap().get_size(), 7);
        // assert_eq!(dir.get_child_dir("dir3").unwrap().get_size(), 11);
    }
}
