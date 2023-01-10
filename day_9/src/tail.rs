use std::collections::HashSet;

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

pub struct Tail {
    points: HashSet<Box<dyn Point>>,
    x: i32,
    y: i32,
}

impl Tail {
    pub fn new() -> Tail {
        let mut t = Tail {
            points: HashSet::new(),
            x: 0,
            y: 0,
        };
        let point = Box::new(XYPoint::new(0, 0));
        t.points.insert(point);
        t
    }
}

impl MoveSubscriber for Tail {

    fn get_points(&self) -> &HashSet<Box<dyn Point>> {
        &self.points
    }

    fn on_move(&mut self, x: i32, y: i32) {
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
        self.points.insert(point);
    }
}

