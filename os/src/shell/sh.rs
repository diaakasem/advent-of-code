use std::cell::RefCell;
use std::rc::Rc;

use crate::fs::dir::Dir;
use crate::fs::file::File;

#[derive(Debug)]
pub struct Prompt {
    pub root: Rc<RefCell<Dir>>,
    pub cwd: Rc<RefCell<Dir>>,
}

impl Prompt {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(Dir::new("/", None)));
        Self {
            root: root.clone(),
            cwd: root.clone(),
        }
    }

    pub fn tree(self: &Self) {
        self.root.borrow_mut().tree(0);
    }

    pub fn pwd(self: &Self) -> String {
        let mut path = String::new();
        let dir = self.cwd.clone();
        while dir.borrow().parent.is_some() {
            path = format!("{}/{}", dir.borrow().name, path);
            dir.swap(&dir.borrow().parent.as_ref().unwrap().clone());
        }
        path = format!("/{}/{}", dir.borrow().name, path);
        path
    }

    pub fn cd(self: &mut Self, path: &str) {
        match path {
            "/" => self.cwd.swap(&self.root),
            "." => (),
            ".." => match &self.cwd.borrow().parent {
                Some(parent) => self.cwd.swap(&parent),
                None => self.cwd.swap(&self.root),
            },
            _ => {
                let name = path;
                if let Some(dir) = self.cwd.borrow().get_child_dir(name) {
                    self.cwd.swap(&dir);
                } else {
                    let d = Dir::new(name, Some(self.cwd.clone()));
                    self.cwd.borrow_mut().add_dir(d);
                    self.cwd
                        .swap(&self.cwd.borrow().get_child_dir(name).unwrap());
                };
            }
        }
    }

    pub fn parse_line(self: &mut Self, line: &str) {
        let mut args: Vec<&str> = line.split_whitespace().collect();
        let size_type = args.remove(0);
        match size_type {
            "dir" => {
                let name = args.last().unwrap();
                self.cwd
                    .borrow_mut()
                    .add_dir(Dir::new(name, Some(self.cwd.clone())));
            }
            _ => {
                let size = size_type.parse::<u64>().unwrap();
                let name = args.last().unwrap();
                self.cwd.borrow_mut().add_file(File::new(name, Some(size)));
            }
        }
    }
}
