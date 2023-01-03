use crate::stack::Stack;

pub struct Crane {
    pub stacks: Vec<Stack>,
}

impl Crane {

    pub fn new() -> Self {
        Crane {
            stacks: vec![],
        }
    }

    pub fn add_stack(&mut self, s: Stack) {
        self.stacks.push(s);
    }

    pub fn move_block(&mut self, from: usize, to: usize) {
        match self.stacks[from].pop() {
            Some(c) => self.stacks[to].push(c),
            None => println!("No block to move, stack {} is empty", from),
        }
    }

    pub fn move_blocks(&mut self, count: usize, from: usize, to: usize) {
        println!("--------------------------------------------------");
        println!("Moving {} blocks from {} to {}", count, from, to);
        println!("--------------------------------------------------");
        self.display();
        (0..count).for_each(|_| self.move_block(from, to));
        println!("--------------------------------------------------");
        self.display();
        println!("==================================================");
    }

    /// Prints the current state of the crane
    ///
    /// # Examples
    /// ```
    /// use crane::crane::Crane;
    /// use crane::stack::Stack;
    /// use crane::crates::Crate;
    /// let mut crane = Crane::new();
    /// let mut stack = Stack::new();
    /// stack.push(Crate::new("A".to_string()));
    /// stack.push(Crate::new("B".to_string()));
    /// crane.add_stack(stack);
    /// crane.print();
    /// ```
    /// Output:
    /// ```text
    ///  1   2   3   4   5   6   7   8   9
    /// [W] [P] [P] [D] [G] [P] [B] [P] [V]
    /// [B] [Z] [Z] [T] [V] [S] [V] [S] [D]
    /// [D] [V] [B] [L] [B] [Q] [D] [M] [T]
    /// [N] [Q] [G] [J] [J]     [F] [F] [R]
    /// [C] [L] [J] [Z] [S]     [L] [B]
    /// [F] [S] [T] [B]         [M] [D]
    /// [J] [T]     [H]         [P] [L]
    ///             [C]         [N] [R]
    ///  ```

    pub fn display(&self) {
        let mut max_height = 0;
        self.stacks.iter().for_each(|s| {
            if s.height() > max_height {
                max_height = s.height();
            }
        });
        let mut i = max_height;
        while i > 0 {
            let mut line = String::new();
            self.stacks.iter().for_each(|s| {
                match s.crates.get(i - 1) {
                    Some(c) => line.push_str(&format!("[{}] ", c)),
                    None => line.push_str("    "),
                }
            });
            println!("{}", line);
            i -= 1;
        }
        let mut line = String::new();
        (1..=self.stacks.len()).for_each(|i| line.push_str(&format!(" {}  ", i)));
        println!("{}", line);
        let mut line2 = String::new();
        (0..self.stacks.len()).for_each(|i| line2.push_str(&format!(" {}  ", self.stacks[i].height())));
        println!("{}", line2);
    }

    pub fn get_stack(&self, index: usize) -> Option<&Stack> {
        self.stacks.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crates::Crate;

    #[test]
    fn test_crane() {
        let mut c = Crane::new();
        let mut s = Stack::new();
        s.push(Crate::new("[ 1 ]".to_string()));
        let mut s2 = Stack::new();
        s2.push(Crate::new("[ 2 ]".to_string()));
        let mut s3 = Stack::new();
        s3.push(Crate::new("[ 3 ]".to_string()));
        c.add_stack(s);
        c.add_stack(s2);
        c.add_stack(s3);
        c.move_block(0, 1);
        c.move_block(2, 1);
        assert_eq!(c.stacks[1].crates.len(), 3);
    }
}

