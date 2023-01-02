#[derive(Clone, Copy, Debug)]
pub struct Meal {
    pub calories: u32,
}

impl Meal {
    pub fn new(calories: u32) -> Self {
        Self { calories }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meal_new() {
        let meal = Meal::new(100);
        assert_eq!(meal.calories, 100);
    }
}

