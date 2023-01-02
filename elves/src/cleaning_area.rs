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

    pub fn overlaps(&self, area: &CleaningArea) -> bool {
        self.start <= area.end && self.end >= area.start
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleaning_area_contains() {
        let area = CleaningArea::new(0, 10);
        let area2 = CleaningArea::new(1, 9);
        assert!(area.contains(&area2));
    }

    #[test]
    fn test_cleaning_area_overlaps() {
        let area = CleaningArea::new(0, 10);
        let area2 = CleaningArea::new(5, 11);
        assert!(area.overlaps(&area2));
    }
}
