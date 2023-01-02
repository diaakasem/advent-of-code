use crate::cleaning_area::CleaningArea;
use crate::meal::Meal;

#[derive(Clone, Debug)]
pub struct Elve {
    id: u8,
    meals: Vec<Meal>,
    cleaning_area: Option<CleaningArea>,
}

impl Elve {
    pub fn new(id: u8) -> Self {
        Self {
            id,
            meals: Vec::new(),
            cleaning_area: None,
        }
    }

    fn add_meal(&mut self, meal: Meal) {
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

pub struct ElvesPact {
    elves: Vec<Elve>,
}

impl ElvesPact {
    pub fn new() -> Self {
        Self { elves: Vec::new() }
    }

    pub fn new_elve(&mut self) {
        self.elves.push(Elve::new(self.elves.len() as u8));
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
        self.get_elves().iter().for_each(|elve| {
            println!("Elve {}", elve.id);
            elve.meals.iter().for_each(|meal| {
                println!("Meal {}", meal.calories);
            });
            println!("Total calories: {}", elve.total_calories());
        });
    }

    pub fn get_total_calories(&self) -> u32 {
        self.elves.iter().map(|elve| elve.total_calories()).sum()
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
        assert_eq!(4, 4);
    }
}
