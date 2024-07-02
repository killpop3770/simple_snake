use std::collections::LinkedList;

use piston_window::{Context, G2d};
use piston_window::types::Color;

use super::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.30, 0.00, 1.0];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

impl MovementDirection {
    pub fn opposite_direction(&self) -> MovementDirection {
        match self {
            MovementDirection::Up => MovementDirection::Down,
            MovementDirection::Down => MovementDirection::Up,
            MovementDirection::Left => MovementDirection::Right,
            MovementDirection::Right => MovementDirection::Left,
        }
    }
}

#[derive(Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: MovementDirection,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut snake_body = LinkedList::new();
        snake_body.push_front(Block { x, y });
        snake_body.push_front(Block { x: x + 1, y });
        snake_body.push_front(Block { x: x + 2, y });

        Snake {
            direction: MovementDirection::Right,
            body: snake_body,
            tail: None,
        }
    }

    pub fn draw(&self, context: &Context, graphical_buffer: &mut G2d) {
        for block in &self.body {
            draw_block(block.x, block.y, SNAKE_COLOR, context, graphical_buffer);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, direction: Option<MovementDirection>) {
        match direction {
            None => (),
            Some(d) => { self.direction = d }
        }

        let (x_head_coordinate, y_head_coordinate) = self.head_position();

        let new_block = match self.direction {
            MovementDirection::Up => { Block { x: x_head_coordinate, y: y_head_coordinate - 1 } }
            MovementDirection::Down => { Block { x: x_head_coordinate, y: y_head_coordinate + 1 } }
            MovementDirection::Left => { Block { x: x_head_coordinate - 1, y: y_head_coordinate } }
            MovementDirection::Right => { Block { x: x_head_coordinate + 1, y: y_head_coordinate } }
        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> MovementDirection {
        self.direction
    }

    pub fn next_head_position(&self, direction: Option<MovementDirection>) -> (i32, i32) {
        let (x_head_coordinate, y_head_coordinate) = self.head_position();

        let mut moving_direction = self.direction;
        match direction {
            None => {}
            Some(d) => { moving_direction = d }
        }
        match moving_direction {
            MovementDirection::Up => (x_head_coordinate, y_head_coordinate - 1),
            MovementDirection::Down => (x_head_coordinate, y_head_coordinate + 1),
            MovementDirection::Left => (x_head_coordinate - 1, y_head_coordinate - 1),
            MovementDirection::Right => (x_head_coordinate + 1, y_head_coordinate),
        }
    }

    pub fn restore_tail(&mut self) {
        let block = self.tail.clone().unwrap();
        self.body.push_back(block);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut cnt = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            //TODO:
            cnt += 1;
            if cnt == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
}