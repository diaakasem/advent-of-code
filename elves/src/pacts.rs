use crate::elves::Elve;
use crate::meal::Meal;

pub struct ElvesPact {
    elves: Vec<Elve>,
}

impl ElvesPact {
    pub fn new() -> Self {
        Self { elves: Vec::new() }
    }

    pub fn new_elve(&mut self) {
        self.elves.push(Elve::new());
    }

    /// Adds a list of elves to the pact
    /// # Arguments
    /// * `elves` - A list of elves
    /// # Example
    /// ```
    /// use elves::elves::{Elve, ElvesPact};
    /// let mut elves = ElvesPact::new();
    /// let elve = Elve::new(0);
    /// elves.add_elves(vec![elve]);
    /// ```
    /// # Panics
    /// Panics if the list of elves is empty
    /// # Remarks
    /// This function is used to add a list of elves to the pact
    /// # See also
    /// [get_elves](#method.get_elves)
    /// [new_elve](#method.new_elve)
    pub fn add_elves(&mut self, elves: Vec<Elve>) {
        self.elves.extend(elves);
    }

    fn get_elves(&self) -> &Vec<Elve> {
        &self.elves
    }

    pub fn add_meal_by_calories_to_last_elve(&mut self, calories: u32) {
        let last_elve = self.elves.last_mut().expect("No elves");
        last_elve.add_meal(Meal { calories });
    }

    pub fn get_elve_with_most_calories(&self) -> &Elve {
        let mut max_calories = 0;
        let mut max_elve = &self.elves[0];
        self.elves.iter().for_each(|elve| {
            if elve.total_calories() > max_calories {
                max_calories = elve.total_calories();
                max_elve = &elve;
            }
        });
        max_elve
    }

    pub fn get_n_elves_with_most_calories(&self, n: u32) -> ElvesPact {
        let mut elves = self.elves.clone();
        elves.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));
        ElvesPact {
            elves: elves[0..n as usize].to_vec(),
        }
    }

    pub fn display_elves(&self) {
        let mut i = 0;
        self.get_elves().iter().for_each(|elve| {
            println!("Elve {}", i);
            elve.meals.iter().for_each(|meal| {
                println!("Meal {}", meal.calories);
            });
            println!("Total calories: {}", elve.total_calories());
        });
    }

    pub fn get_total_calories(&self) -> u32 {
        self.elves.iter().map(|elve| elve.total_calories()).sum()
    }

    /// Checks wheather cleaning areas of the elves are the redundant, that is one fully contains
    /// the other
    /// # Arguments
    /// # Example
    /// ```
    /// use elves::elves::{Elve, ElvesPact};
    /// let mut elves = ElvesPact::new();
    /// let elve1 = Elve::new(0);
    /// elve1.set_cleaning_area(CleaningArea::new(7, 8));
    /// let elve2 = Elve::new(1);
    /// elve2.set_cleaning_area(CleaningArea::new(6, 10));
    /// elves.add_elves(vec![elve1, elve2]);
    /// assert_eq!(elves.are_cleaning_areas_redundant(), true);
    /// ```
    ///
    /// # Remarks
    /// This function is used to check wheather cleaning areas of the elves are the redundant, that is one fully contains
    ///
    ///
    ///
    pub fn is_cleaning_area_redundant(&self) -> bool {
        match self.elves[0].get_cleaning_area() {
            Some(first_area) => self.elves[1..]
                .iter()
                .all(|elve| match elve.get_cleaning_area() {
                    Some(area) => {
                        first_area.contains(&area) || area.contains(&first_area)
                    }
                    None => false,
                }),
            None => false,
        }
    }

    pub fn is_cleaning_area_overlapping(&self) -> bool {
        match self.elves[0].get_cleaning_area() {
            Some(first_area) => self.elves[1..]
                .iter()
                .all(|elve| match elve.get_cleaning_area() {
                    Some(area) => {
                        first_area.overlaps(&area)
                    }
                    None => false,
                }),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_elve() {
        let mut elves = ElvesPact::new();
        elves.new_elve();
        assert_eq!(elves.get_elves().len(), 1);
    }

    #[test]
    fn test_add_meal_by_calories_to_last_elve() {
        let mut elves = ElvesPact::new();
        elves.new_elve();
        elves.add_meal_by_calories_to_last_elve(10);
        assert_eq!(elves.get_elves()[0].total_calories(), 10);
    }

    #[test]
    fn test_get_elve_with_most_calories() {
        let mut elves = ElvesPact::new();
        elves.new_elve();
        elves.add_meal_by_calories_to_last_elve(10);
        elves.new_elve();
        elves.add_meal_by_calories_to_last_elve(20);
        assert_eq!(elves.get_elve_with_most_calories().total_calories(), 20);
    }

}
