use common::Tile;
use macroquad::{
    color,
    math::Vec2,
    shapes::draw_rectangle,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture},
    window::{screen_height, screen_width},
};

// pub const GRID_WIDTH: usize = 80;
// pub const GRID_HEIGHT: usize = 60;
pub const GRID_WIDTH: usize = 40;
pub const GRID_HEIGHT: usize = 30;

pub struct Grid {
    tiles: [Tile; GRID_WIDTH * GRID_HEIGHT],
    tile_width: f32,
    tile_height: f32,

    coin_texture: Texture2D,
}

impl Grid {
    pub async fn new() -> Self {
        let mut this = Self {
            tiles: [Tile::Empty; GRID_WIDTH * GRID_HEIGHT],
            tile_width: 10.0,
            tile_height: 10.0,

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
        self.tile_width = screen_width() / GRID_WIDTH as f32;
        self.tile_height = screen_height() / GRID_HEIGHT as f32;
    }

    pub fn draw(&self) {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let tile = self.get(x, y);
                let x = x as f32 * self.tile_width;
                let y = y as f32 * self.tile_height;
                // let r = self.tile_width.min(self.tile_height) / 2.0;
                match tile {
                    Tile::Wall => {
                        draw_rectangle(x, y, self.tile_width, self.tile_height, color::BROWN);
                    }
                    Tile::Coins => {
                        draw_texture_ex(
                            &self.coin_texture,
                            x,
                            y,
                            color::WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(self.tile_width, self.tile_height)),
                                ..Default::default()
                            },
                        );
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn tile_height(&self) -> f32 {
        self.tile_height
    }
    pub fn tile_width(&self) -> f32 {
        self.tile_width
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}
