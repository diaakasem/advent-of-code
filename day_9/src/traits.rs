use std::{collections::HashSet, hash::{Hash, Hasher}};

pub enum Direction { R, L, U, D, }

pub trait Point  {
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
}


impl PartialEq for Box<dyn Point> {
    fn eq(&self, other: &Self) -> bool {
        self.get_x() == other.get_x() && self.get_y() == other.get_y()
    }
}
impl Eq for Box<dyn Point> {}
impl Hash for Box<dyn Point> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_x().hash(state);
        self.get_y().hash(state);
    }
}


pub trait Movable {
    fn move_towards(&mut self, direction: &Direction, distance: i32);
}

pub trait MoveSubscriber {
    fn on_move(&mut self, x: i32, y: i32);
    fn get_points(&self) -> &HashSet<Box<dyn Point>>;
}

pub trait MovePublisher  {
    fn subscribers(&mut self) -> &mut Vec<Box<dyn MoveSubscriber>>;
    fn subscribe(&mut self, subscriber: Box<dyn MoveSubscriber>);
    fn publish(&mut self);
}
