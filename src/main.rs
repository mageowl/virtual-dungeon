use std::env;

use macroquad::{
    color::{self, Color},
    input::{KeyCode, is_key_pressed},
    shapes::draw_rectangle,
    text::draw_text,
    window::{next_frame, screen_height, screen_width},
};
use world::{character::Character, grid::Grid};

mod interface;
mod world;

enum Screen {
    Main,
    Leaderboard,
}

#[macroquad::main("virtual dungeon")]
async fn main() {
    let mut grid = Grid::new().await;

    let mut characters: Vec<Character> = env::args()
        .skip(1)
        .filter_map(|file_name| Character::new(file_name, &mut grid))
        .collect();

    let mut screen = Screen::Main;

    loop {
        match screen {
            Screen::Main => {
                grid.update();
                grid.draw();

                for character in &mut characters {
                    character.update(&mut grid);
                    character.draw(&grid);
                }

                if is_key_pressed(KeyCode::L) {
                    screen = Screen::Leaderboard;
                }
            }
            Screen::Leaderboard => {
                draw_leaderboard(&mut grid, &characters);

                if is_key_pressed(KeyCode::L) {
                    screen = Screen::Main;
                }
            }
        }

        next_frame().await;
    }
}

fn draw_leaderboard(grid: &mut Grid, characters: &Vec<Character>) {
    grid.update();
    grid.draw();

    for character in characters {
        character.draw(&grid);
    }

    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color {
            a: 0.5,
            ..color::BLACK
        },
    );

    draw_rectangle(
        50.0,
        30.0,
        500.0,
        300.0,
        Color {
            r: 0.05,
            g: 0.05,
            b: 0.05,
            a: 1.0,
        },
    );

    draw_text("leaderboard", 50.0, 50.0, 32.0, color::WHITE);

    let mut sorted_characters = characters.iter().collect::<Vec<_>>();
    sorted_characters.sort_by_key(|c| c.points());
    for (i, character) in sorted_characters.into_iter().rev().enumerate() {
        let text = if character.is_dead() {
            format!("[dead] {}: {}pts", character.name(), character.points())
        } else {
            format!("{}: {}pts", character.name(), character.points())
        };
        draw_text(
            &text,
            55.0,
            70.0 + (20 * i) as f32,
            16.0,
            *character.color(),
        );
    }
}
