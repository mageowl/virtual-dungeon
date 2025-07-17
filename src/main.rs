use std::env;

use macroquad::window::next_frame;
use world::{character::Character, grid::Grid};

mod interface;
mod world;

#[macroquad::main("virtual dungeon")]
async fn main() {
    let mut grid = Grid::new();

    let mut characters: Vec<Character> = env::args()
        .skip(1)
        .filter_map(|file_name| Character::new(file_name, &mut grid))
        .collect();

    loop {
        grid.update();
        grid.draw();

        for character in &mut characters {
            character.update(&mut grid);
            character.draw(&grid);
        }

        next_frame().await;
    }
}
