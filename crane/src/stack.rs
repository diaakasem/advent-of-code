use crate::crates::Crate;

pub struct Stack {
    pub crates: Vec<Crate>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            crates: Vec::new(),
        }
    }

    pub fn push(&mut self, c: Crate) {
        self.crates.push(c);
    }

    pub fn pop(&mut self) -> Option<Crate> {
        self.crates.pop()
    }

    pub fn height(&self) -> usize {
        self.crates.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut s = Stack::new();
        s.push(Crate::new("[ 1 ]".to_string()));
        s.push(Crate::new("[ 2 ]".to_string()));
        s.push(Crate::new("[ 3 ]".to_string()));
        assert_eq!(s.crates.len(), 3);
        let c = s.pop().unwrap();
        assert_eq!(c.name, "[ 3 ]");
        assert_eq!(s.crates.len(), 2);
    }
}
