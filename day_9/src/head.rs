use crate::traits::*;

pub struct Head {
    subscribers: Vec<Box<dyn MoveSubscriber>>,
    x: i32,
    y: i32,
}

impl Head {
    pub fn new() -> Head {
        Head {
            subscribers: Vec::new(),
            x: 0,
            y: 0,
        }
    }
}

impl Point for Head {
    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }
}

impl MovePublisher for Head {
    fn subscribe(&mut self, subscriber: Box<dyn MoveSubscriber>) {
        self.subscribers.push(subscriber);
    }

    fn subscribers(&mut self) -> &mut Vec<Box<dyn MoveSubscriber>> {
        &mut self.subscribers
    }

    fn publish(&mut self) {
        let x = self.get_x();
        let y = self.get_y();
        for subscriber in self.subscribers().iter_mut() {
            subscriber.on_move(x, y);
        }
    }
}

impl Movable for Head {
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
