use piston_window::{Button, clear, PistonWindow, PressEvent, UpdateEvent, WindowSettings};
use piston_window::types::Color;
use crate::draw::to_coordinate_u32;
use crate::game::Game;

mod draw;
mod snake;
mod game;

const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    println!("Hello, friend! It's a 'simple_snake' game!");

    let (window_width, window_height) = (30, 30);

    let mut main_window: PistonWindow =
        WindowSettings::new("simple_snake", [to_coordinate_u32(window_width), to_coordinate_u32(window_height)])
            .build().expect("Some error occurred when creating main window!");

    let mut game = Game::new(window_width, window_height);
    while let Some(event) = main_window.next() { //instead loop and match
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        main_window.draw_2d(&event, |c, g, _| {
            clear(BACKGROUND_COLOR, g);
            game.draw(&c, g);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
