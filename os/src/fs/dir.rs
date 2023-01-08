
use super::sizable::Sizable;
use std::rc::Rc;
use std::cell::RefCell;

use super::file::File;

#[derive(Debug, Clone)]
pub struct Dir {
    pub name: String,
    pub files: Vec<File>,
    pub dirs: Vec<Rc<RefCell<Dir>>>,
    pub parent: Option<Rc<RefCell<Dir>>>,
}

impl PartialEq for Dir {
    fn eq(&self, other: &Self) -> bool {
        let my_path = self.path();
        let other_path = other.path();
        self.name == other.name && my_path == other_path
    }
}

impl Sizable for Dir {

    fn get_size(&self) -> u64 {
        let mut size = self.files.iter().map(|file| file.size).sum::<u64>();
        // println!("Files size: {}", size);
        // println!("Files {}", self.files.len());
        size += self.dirs.iter().map(|dir| dir.borrow().get_size()).sum::<u64>();
        // println!("get_size: {} {}", self.name, size);
        size
    }
}


impl Dir {

    pub fn new(name: &str, parent: Option<Rc<RefCell<Self>>>) -> Self {
        let name = name.to_string();
        let files: Vec<File> = vec![];
        let dirs: Vec<Rc<RefCell<Self>>> = vec![];
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

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
        // let f = self.files.last().unwrap();
        // println!("New file: {:?} {}", f.name, f.size);
        // println!("Size: {}", self.get_size());
    }

    pub fn add_dir(&mut self, dir: Self) {
        self.dirs.push(Rc::new(RefCell::new(dir)));
        // println!("New dir: {:?}", self.dirs.last().unwrap().borrow().name);
    }

    pub fn tree(self: &Self, level: u8) {
        let tabs = "|  ".repeat(level as usize);
        println!(
            "{}- {} (dir, size={})",
            tabs,
            self.name,
            self.get_readable_size()
        );
        for dir in &self.dirs {
            dir.borrow().tree(level + 1);
        }
        for file in &self.files {
            println!("{}|- {} (file, size={})", tabs, file.name, file.get_readable_size());
        }
    }

    pub fn get_child_dir(&self, name: &str) -> Option<Rc<RefCell<Self>>> {
        let found_dir = self.dirs.iter().find(|dir| dir.borrow().name == name);
        match found_dir {
            Some(dir) => Some(dir.clone()),
            None => None,
        }
    }

    pub fn path(&self) -> String {
        let mut path = String::new();
        let my_parent = self.parent.clone();
        if let Some(parent) = my_parent {
            match parent.try_borrow() {
                Ok(parent) => path = format!("{}/{}", parent.path(), self.name),
                Err(_) => (),
            }
        }
        path
    }

    pub fn get_parent(self: &Self) -> Option<Rc<RefCell<Self>>> {
        self.parent.clone()
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
