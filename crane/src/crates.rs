use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Crate {
    pub name: String,
}

impl Crate {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn display(&self) {
        println!("Crate: {}", self.name);
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate() {
        let c = Crate::new("test".to_string());
        assert_eq!(c.name, "test");
    }

}
