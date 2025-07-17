use std::fmt::Display;

use macroquad::{
    color,
    math::Vec2,
    shapes::{draw_circle, draw_rectangle},
    text::draw_text,
    texture::{DrawTextureParams, Texture2D, draw_texture, draw_texture_ex, load_texture},
    window::{screen_height, screen_width},
};

pub const GRID_WIDTH: usize = 80;
pub const GRID_HEIGHT: usize = 60;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Character,
    Coins,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => f.write_str("empty"),
            Tile::Wall => f.write_str("wall"),
            Tile::Character => f.write_str("robot"),
            Tile::Coins => f.write_str("coins"),
        }
    }
}

pub struct Grid {
    tiles: [Tile; GRID_WIDTH * GRID_HEIGHT],
    tw: f32,
    th: f32,

    coin_texture: Texture2D,
}

impl Grid {
    pub async fn new() -> Self {
        let mut this = Self {
            tiles: [Tile::Empty; GRID_WIDTH * GRID_HEIGHT],
            tw: 10.0,
            th: 10.0,

            coin_texture: load_texture("assets/coin.png").await.unwrap(),
        };
        this.gen_bst(
            Rect {
                x: 0,
                y: 0,
                w: GRID_WIDTH,
                h: GRID_HEIGHT,
            },
            0,
        );
        this
    }

    pub fn get(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[x + y * GRID_WIDTH]
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.tiles[x + y * GRID_WIDTH]
    }

    pub fn update(&mut self) {
        self.tw = screen_width() / GRID_WIDTH as f32;
        self.th = screen_height() / GRID_HEIGHT as f32;
    }

    pub fn draw(&self) {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let tile = self.get(x, y);
                let x = x as f32 * self.tw;
                let y = y as f32 * self.th;
                let r = self.tw.min(self.th) / 2.0;
                match tile {
                    Tile::Wall => {
                        draw_rectangle(x, y, self.tw, self.th, color::BROWN);
                    }
                    Tile::Coins => {
                        draw_texture_ex(
                            &self.coin_texture,
                            x,
                            y,
                            color::WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(self.tw, self.th)),
                                ..Default::default()
                            },
                        );
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn th(&self) -> f32 {
        self.th
    }
    pub fn tw(&self) -> f32 {
        self.tw
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}
