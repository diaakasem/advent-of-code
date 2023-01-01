#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Item {
    pub name: char,
    pub priority: u8,
}

impl Item {
    pub fn new(name: char) -> Item {
        if char::is_alphabetic(name) {
            let index;
            if name.is_uppercase() {
                index = name as u8 - 'A' as u8 + 26;
            } else {
                index = name as u8 - 'a' as u8;
            }
            let priority = index + 1;
            Item { name, priority }
        } else {
            panic!("Item name must be alphabetic");
        }
    }
}

pub struct RuckSack {
    right_compartment: Vec<Item>,
    left_compartment: Vec<Item>,
}

impl RuckSack {
    pub fn new() -> RuckSack {
        RuckSack {
            right_compartment: Vec::new(),
            left_compartment: Vec::new(),
        }
    }

    pub fn set_data(&mut self, letters: &str) {
        let right_compartment = letters.split_at(letters.len() / 2).0;
        let left_compartment = letters.split_at(letters.len() / 2).1;
        self.right_compartment = right_compartment.chars().map(|c| Item::new(c)).collect();
        self.left_compartment = left_compartment.chars().map(|c| Item::new(c)).collect();
    }

    pub fn get_duplicates(&self) -> Vec<Item> {
        let mut duplicates = Vec::new();
        for item in &self.right_compartment {
            if self.left_compartment.contains(item) {
                if !duplicates.contains(item) {
                    duplicates.push(item.clone());
                }
            }
        }
        duplicates
    }

    pub fn get_duplicates_waste(&self) -> u32 {
        let duplicates = self.get_duplicates();
        duplicates.iter().map(|i| i.priority as u32).sum()
    }

    pub fn set_compartments(&mut self, right: Vec<Item>, left: Vec<Item>) {
        self.right_compartment = right;
        self.left_compartment = left;
    }

    pub fn get_unique_items(&self) -> Vec<Item> {
        let mut items = self.right_compartment.clone();
        items.extend(self.left_compartment.clone());
        items.sort();
        items.dedup();
        items
    }

    pub fn add_items(&mut self, items: Vec<Item>) {
        for item in items {
            if item.priority <= 26 {
                self.left_compartment.push(item);
            } else {
                self.right_compartment.push(item);
            }
        }
    }

    pub fn add_item(&mut self, item: Item) {
        if self.right_compartment.len() == 0 {
            self.right_compartment.push(item);
        } else {
            let mut index = 0;
            for i in 0..self.right_compartment.len() {
                if self.right_compartment[i].priority > item.priority {
                    index = i;
                    break;
                }
            }
            self.right_compartment.insert(index, item);
        }
    }

    pub fn remove_item(&mut self, item: Item) {
        let mut index = 0;
        for i in 0..self.right_compartment.len() {
            if self.right_compartment[i].name == item.name {
                index = i;
                break;
            }
        }
        self.right_compartment.remove(index);
    }

    pub fn move_item(&mut self, item: Item) {
        self.remove_item(item);
        self.left_compartment.push(item);
    }

    pub fn print(&self) {
        println!("Right Compartment:");
        for i in 0..self.right_compartment.len() {
            println!("{}: {}", i, self.right_compartment[i].name);
        }
        println!("Left Compartment:");
        for i in 0..self.left_compartment.len() {
            println!("{}: {}", i, self.left_compartment[i].name);
        }
    }
}

pub struct RuckSacksGroup {
    rucksacks: Vec<RuckSack>,
}

impl RuckSacksGroup {
    pub fn new() -> RuckSacksGroup {
        RuckSacksGroup {
            rucksacks: Vec::new(),
        }
    }

    pub fn add_rucksack(&mut self, rucksack: RuckSack) {
        self.rucksacks.push(rucksack);
    }

    pub fn get_common_items(&mut self) -> Vec<Item> {
        let mut main_items = vec![];
        let mut still_standing = vec![];
        for rucksack in &self.rucksacks {
            let items = rucksack.get_unique_items();
            if main_items.len() == 0 {
                for item in items {
                    main_items.push(item);
                }
            } else {
                for item in items {
                    if let Some(index) = main_items.iter().position(|i| i.name == item.name) {
                        still_standing.push(main_items[index]);
                    }
                }
                main_items = still_standing;
                still_standing = vec![];
            }
        }
        main_items
    }

    pub fn get_duplicates_waste(&mut self) -> u32 {
        let duplicates = self.get_common_items();
        duplicates.iter().map(|i| i.priority as u32).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_priority_for_uppercase() {
        let item = Item::new('A');
        assert_eq!(item.priority, 27);
        let item = Item::new('L');
        assert_eq!(item.priority, 38);
        let item = Item::new('P');
        assert_eq!(item.priority, 42);
        let item = Item::new('Z');
        assert_eq!(item.priority, 52);
    }

    #[test]
    fn correct_priority_for_lowercase() {
        let item = Item::new('a');
        assert_eq!(item.priority, 1);
        let item = Item::new('p');
        assert_eq!(item.priority, 16);
        let item = Item::new('s');
        assert_eq!(item.priority, 19);
        let item = Item::new('t');
        assert_eq!(item.priority, 20);
        let item = Item::new('v');
        assert_eq!(item.priority, 22);
        let item = Item::new('z');
        assert_eq!(item.priority, 26);
    }

    #[test]
    fn common_items() {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];
        let mut rucksacks = RuckSacksGroup::new();
        for line in lines {
            let mut rucksack = RuckSack::new();
            let items = line.chars().map(|c| Item::new(c)).collect();
            rucksack.add_items(items);
            rucksacks.add_rucksack(rucksack);
        }
        let common_items = rucksacks.get_common_items();
        println!("Common Items: {:?}", common_items);

        assert_eq!(common_items.len(), 1);
        assert_eq!(common_items.iter().map(|i| i.priority).sum::<u8>(), 18);
    }
}
