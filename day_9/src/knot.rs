use std::{collections::HashSet, cell::RefCell, rc::Rc};

use crate::traits::*;

pub struct XYPoint {
    x: i32,
    y: i32,
}

impl XYPoint {
    pub fn new(x: i32, y: i32) -> XYPoint {
        XYPoint { x, y }
    }
}

impl Point for XYPoint {
    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }
}

pub struct Knot {
    name: String,
    points: HashSet<Box<dyn Point>>,
    subscribers: Vec<Rc<RefCell<dyn MoveSubscriber>>>,
    x: i32,
    y: i32,
}

impl Knot {
    pub fn new(name: &str) -> Knot {
        let mut k = Knot {
            name: name.to_string(),
            points: HashSet::new(),
            subscribers: Vec::new(),
            x: 0,
            y: 0,
        };
        let point = Box::new(XYPoint::new(0, 0));
        k.points.insert(point);
        // println!("{}: x: {:?}, y: {:?}", k.name, k.x, k.y);
        k
    }
}

impl Point for Knot {
    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }
}

impl MovePublisher for Knot {
    fn subscribe(&mut self, subscriber: Rc<RefCell<dyn MoveSubscriber>>) {
        self.subscribers.push(subscriber);
    }

    fn subscribers(&mut self) -> &mut Vec<Rc<RefCell<dyn MoveSubscriber>>> {
        &mut self.subscribers
    }

    fn publish(&mut self) {
        let x = self.get_x();
        let y = self.get_y();
        for subscriber in self.subscribers().iter() {
            subscriber.borrow_mut().on_move(x, y)
        }
    }
}

impl Movable for Knot {
    fn move_towards(&mut self, dir: &Direction, steps: i32) {
        match dir {
            Direction::R => self.x += steps,
            Direction::L => self.x -= steps,
            Direction::U => self.y += steps,
            Direction::D => self.y -= steps,
        }
        for _ in 0..steps {
            self.publish();
        }
    }
}

impl MoveSubscriber for Knot {

    fn get_points(&self) -> &HashSet<Box<dyn Point>> {
        &self.points
    }

    fn on_move(&mut self, x: i32, y: i32) {
        // println!("ONMOVE: {}: x: {:?}, y: {:?}", self.name, x, y);
        let same = self.x == x && self.y == y;
        let same_x = self.x == x;
        let same_y = self.y == y;
        let close_y = same_x && (self.y - y).abs() == 1;
        let close_x = same_y && (self.x - x).abs() == 1;
        let close_x_y = (self.x - x).abs() == 1 && (self.y - y).abs() == 1;
        let is_fine = same || close_x || close_y || close_x_y;
        if is_fine {
            return;
        }
        if same_x {
            if self.y > y {
                self.y -= 1;
            } else {
                self.y += 1;
            }
        } else if same_y {
            if self.x > x {
                self.x -= 1;
            } else {
                self.x += 1;
            }
        } else {
            if self.x > x {
                self.x -= 1;
            } else {
                self.x += 1;
            }
            if self.y > y {
                self.y -= 1;
            } else {
                self.y += 1;
            }
        }
        let point = Box::new(XYPoint::new(self.x, self.y));
        // println!("{}: x: {:?}, y: {:?}", self.name, self.x, self.y);
        self.points.insert(point);
        self.publish();
    }
}

