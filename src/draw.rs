use piston_window::{Context, G2d, rectangle};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coordinate(game_coordinate: i32) -> f64 {
    (game_coordinate as f64) * BLOCK_SIZE
}

pub fn draw_block(x: i32, y: i32, color: Color, context: &Context, graphical_buffer: &mut G2d) {
    let normalized_x = to_coordinate(x);
    let normalized_y = to_coordinate(y);

    rectangle(color,
              [normalized_x, normalized_y, BLOCK_SIZE, BLOCK_SIZE],
              context.transform,
              graphical_buffer);
}

fn draw_rectangle(x: i32, y: i32, width: i32, height: i32, color: Color, context: &Context, graphical_buffer: &mut G2d) {
    let normalized_x = to_coordinate(x);
    let normalized_y = to_coordinate(y);

    rectangle(color,
              [normalized_x, normalized_y,
                  BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)],
              context.transform,
              graphical_buffer);
}