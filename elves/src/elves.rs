use crate::cleaning_area::CleaningArea;
use crate::meal::Meal;

#[derive(Clone, Debug)]
pub struct Elve {
    pub meals: Vec<Meal>,
    pub cleaning_area: Option<CleaningArea>,
}

impl Elve {
    pub fn new() -> Self {
        Self {
            meals: Vec::new(),
            cleaning_area: None,
        }
    }

    pub fn add_meal(&mut self, meal: Meal) {
        self.meals.push(meal);
    }

    pub fn total_calories(&self) -> u32 {
        self.meals.iter().map(|m| m.calories).sum()
    }

    pub fn set_cleaning_area(&mut self, area: CleaningArea) {
        self.cleaning_area = Some(area);
    }

    pub fn get_cleaning_area(&self) -> Option<CleaningArea> {
        self.cleaning_area.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elve_add_meal() {
        let mut elve = Elve::new();
        elve.add_meal(Meal::new(100));
        elve.add_meal(Meal::new(100));
        elve.add_meal(Meal::new(100));
        assert_eq!(elve.total_calories(), 300);
    }

    #[test]
    fn test_elve_set_cleaning_area() {
        let mut elve = Elve::new();
        elve.set_cleaning_area(CleaningArea::new(0, 10));
        assert_eq!(elve.get_cleaning_area().unwrap().start, 0);
        assert_eq!(elve.get_cleaning_area().unwrap().end, 10);
    }
}
