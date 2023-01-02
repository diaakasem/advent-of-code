#[derive(Clone, Copy, Debug)]
pub struct CleaningArea {
    pub start: u32,
    pub end: u32,
}

impl CleaningArea {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, area: &CleaningArea) -> bool {
        self.start <= area.start && self.end >= area.end
    }
}

