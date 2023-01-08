use super::sizable::Sizable;

#[derive(Clone, Debug)]
pub struct File {
    pub name: String,
    pub size: u64
}

impl File {

    pub fn new(name: &str, size: Option<u64>) -> Self {
        let size = size.unwrap_or(0_u64);
        let name = name.to_string();
        Self {
            name, size
        }
    }

}

impl Sizable for File {

    fn get_size(&self) -> u64 {
        self.size
    }

}
