use piston_window::{Context, G2d, Key};
use piston_window::types::Color;
use rand::{Rng, thread_rng};
use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{MovementDirection, Snake};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    is_food_exists: bool,
    x_food_coordinate: i32,
    y_food_coordinate: i32,

    window_width: i32,
    window_height: i32,

    is_game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            is_food_exists: true,
            x_food_coordinate: 6,
            y_food_coordinate: 4,
            window_width: width,
            window_height: height,
            is_game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.is_game_over {
            return;
        }

        let direction = match key {
            Key::Up => Some(MovementDirection::Up),
            Key::Down => Some(MovementDirection::Down),
            Key::Left => Some(MovementDirection::Left),
            Key::Right => Some(MovementDirection::Right),
            _ => Some(self.snake.head_direction()),
        };

        if let Some(direction) = direction {
            if direction == self.snake.head_direction().opposite_direction() {
                return;
            }

            self.update_snake(Some(direction));
        }
    }

    pub fn draw(&self, context: &Context, graphical_buffer: &mut G2d) {
        self.snake.draw(context, graphical_buffer);

        if self.is_food_exists {
            draw_block(self.x_food_coordinate, self.y_food_coordinate, FOOD_COLOR, context, graphical_buffer);
        }

        draw_rectangle(0, 0, self.window_width, 1, BORDER_COLOR, context, graphical_buffer);
        draw_rectangle(0, self.window_height - 1, self.window_width, 1, BORDER_COLOR, context, graphical_buffer);
        draw_rectangle(0, 0, 1, self.window_height, BORDER_COLOR, context, graphical_buffer);
        draw_rectangle(self.window_width - 1, 0, 1, self.window_height, BORDER_COLOR, context, graphical_buffer);

        if self.is_game_over {
            draw_rectangle(0, 0, self.window_width, self.window_height, GAMEOVER_COLOR, context, graphical_buffer);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.is_game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.is_food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (x_head_coordinate, y_head_coordinate) = self.snake.head_position();
        if self.x_food_coordinate == x_head_coordinate && self.y_food_coordinate == y_head_coordinate {
            self.is_food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, direction: Option<MovementDirection>) -> bool {
        let (x_next_head_coordinate, y_next_head_coordinate) = self.snake.next_head_position(direction);

        if self.snake.overlap_tail(x_next_head_coordinate, y_next_head_coordinate) {
            return false;
        }

        x_next_head_coordinate > 0 && y_next_head_coordinate > 0 &&
            x_next_head_coordinate < self.window_width && y_next_head_coordinate < self.window_height
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x_food_coordinate = rng.gen_range(1..self.window_width - 1);
        let mut new_y_food_coordinate = rng.gen_range(1..self.window_height - 1);

        while self.snake.overlap_tail(new_x_food_coordinate, new_y_food_coordinate) {
            new_x_food_coordinate = rng.gen_range(1..self.window_width - 1);
            new_y_food_coordinate = rng.gen_range(1..self.window_height - 1);
        }

        self.is_food_exists = true;
        self.x_food_coordinate = new_x_food_coordinate;
        self.y_food_coordinate = new_y_food_coordinate;
    }

    fn update_snake(&mut self, direction: Option<MovementDirection>) {
        if self.check_if_snake_alive(direction) {
            self.snake.move_forward(direction);
            self.check_eating();
        } else {
            self.is_game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.is_game_over = false;
        self.is_food_exists = true;
        self.x_food_coordinate = 5;
        self.y_food_coordinate = 5;
        self.waiting_time = 0.0;
    }
}
