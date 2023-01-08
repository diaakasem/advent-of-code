use std::cell::RefCell;
use std::rc::Rc;
// use std::sync::Arc;

use crate::fs::dir::Dir;
use crate::fs::file::File;

#[derive(Debug)]
pub struct Prompt {
    pub root: Rc<RefCell<Dir>>,
    pub cwd: Rc<RefCell<Dir>>,
}

impl Prompt {
    pub fn new() -> Self {
        let root = RefCell::new(Dir::new("/", None));
        Self {
            root: Rc::new(root.clone()),
            cwd: Rc::new(root.clone()),
        }
    }

    pub fn tree(self: &Self) {
        self.root.borrow_mut().tree(0);
    }

    pub fn pwd(self: &Self) -> String {
        self.cwd.borrow().path()
    }

    pub fn cd(&mut self, path: &str) {
        match path {
            "/" => {
                if self.cwd.borrow().eq(&self.root.borrow()) {
                    self.cwd = self.root.clone();
                    // println!("cd / to root: {:?}", self.cwd.borrow().name);
                }
            }
            "." => (),
            ".." => {
                let me = self.cwd.clone();
                let option_parent = self.cwd.borrow().get_parent();
                match option_parent {
                    Some(parent) => {
                        // println!("[Before] cd .. to parent: {:?}", me.borrow().name);
                        // println!("[Before] pwd: {}", self.pwd());
                        // println!("[Before] size: {}", me.borrow().get_size());
                        self.cwd = parent;
                        let cwd = self.cwd.borrow();
                        // println!("cd .. to parent: {:?}", cwd.name);
                        // println!("pwd: {}", self.pwd());
                        // println!("size: {}", cwd.get_size());
                    },
                    None => {
                        self.cwd = self.root.clone();
                        // println!("cd .. to root: {:?}", self.cwd.borrow().name);
                    }
                };
            },
            _ => {
                let name = path;
                let child_option = self.cwd.borrow().get_child_dir(name);
                if let Some(dir) = child_option {
                    self.cwd = dir;
                    // println!("cd to child: {:?}", self.cwd.borrow().name);
                } else {
                    let d = Dir::new(name, Some(self.cwd.clone()));
                    self.cwd.borrow_mut().add_dir(d);
                    let child_option = self.cwd.borrow().get_child_dir(name);
                    if let Some(dir) = child_option {
                        self.cwd = dir;
                        // println!("cd to new child: {:?}", self.cwd.borrow().name);
                    }
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
                let dir = Dir::new(name, Some(self.cwd.clone()));
                self.cwd
                    .borrow_mut()
                    .add_dir(dir);
            }
            _ => {
                let size = size_type.parse::<u64>().unwrap();
                let name = args.last().unwrap();
                self.cwd.borrow_mut().add_file(File::new(name, Some(size)));
            }
        }
    }
}
