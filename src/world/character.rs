use std::{
    io::Write,
    process::{Child, ChildStdin},
    sync::mpsc::{self, Receiver, TryRecvError},
};

use macroquad::{
    color::{self, Color},
    math::FloatExt,
    shapes::{draw_rectangle, draw_rectangle_ex, draw_rectangle_lines},
    text::{TextDimensions, draw_text, measure_text},
    time::get_frame_time,
};

use super::grid::{self, GRID_HEIGHT, GRID_WIDTH, Grid, Tile};
use crate::interface::{
    handler::spawn_handler,
    request::{Direction, Request},
    spawn_from_file,
};

pub enum State {
    WaitingForRequest,
    Timeout(f32, bool),
    Dying(f32),
}

struct ScanVisual {
    offset: (i8, i8),
    time: f32,
}

pub struct Character {
    reciever: Receiver<Request>,
    stdin: ChildStdin,
    process: Child,

    state: State,
    tile_pos: (usize, usize),
    x: f32,
    y: f32,

    label_size: TextDimensions,
    name: String,
    color: Color,
    scan_visuals: Vec<ScanVisual>,
}

impl Character {
    pub fn new(file_name: String, grid: &mut Grid) -> Option<Self> {
        let (sender, reciever) = mpsc::channel();

        let mut process = spawn_from_file(&file_name)?;
        let r = rand::random_range(64..255);
        let g = rand::random_range(64..255);
        let b = rand::random_range(64..255);
        spawn_handler(
            file_name.clone(),
            format!("38;2;{r};{g};{b}",),
            &mut process,
            sender,
        );

        let mut tile_pos = (
            rand::random_range(1..GRID_WIDTH - 1),
            rand::random_range(1..GRID_HEIGHT - 1),
        );
        while *grid.get(tile_pos.0, tile_pos.1) != Tile::Empty {
            tile_pos = (
                rand::random_range(1..GRID_WIDTH - 1),
                rand::random_range(1..GRID_HEIGHT - 1),
            );
        }
        *grid.get_mut(tile_pos.0, tile_pos.1) = Tile::Character;

        Some(Self {
            reciever,
            stdin: process.stdin.take()?,
            process,

            state: State::WaitingForRequest,
            tile_pos,
            x: tile_pos.0 as f32,
            y: tile_pos.1 as f32,

            label_size: measure_text(&file_name, None, Self::LABEL_FONT_SIZE, 1.0),
            name: file_name,
            color: Color {
                r: r as f32 / 255.0,
                g: g as f32 / 255.0,
                b: b as f32 / 255.0,
                a: 1.0,
            },
            scan_visuals: Vec::new(),
        })
    }

    pub fn update(&mut self, grid: &mut Grid) {
        if !matches!(self.state, State::Dying(_)) {
            self.x = self.x.lerp(self.tile_pos.0 as f32, 0.2);
            self.y = self.y.lerp(self.tile_pos.1 as f32, 0.2);
            if matches!(grid.get(self.tile_pos.0, self.tile_pos.1), Tile::Empty) {
                self.state = State::Dying(-20.0);
            }
        }

        match &mut self.state {
            State::WaitingForRequest => match self.reciever.try_recv() {
                Ok(Request::Move(dir)) => {
                    self.state = State::Timeout(0.1, true);

                    *grid.get_mut(self.tile_pos.0, self.tile_pos.1) = Tile::Empty;
                    self.tile_pos += dir;
                    if *grid.get(self.tile_pos.0, self.tile_pos.1) != Tile::Empty {
                        self.tile_pos -= dir;
                    }
                    *grid.get_mut(self.tile_pos.0, self.tile_pos.1) = Tile::Character;
                }
                Ok(Request::Attack(dir)) => {
                    let target = self.tile_pos + dir;
                    if matches!(grid.get(target.0, target.1), Tile::Character) {
                        *grid.get_mut(target.0, target.1) = Tile::Empty;
                        self.state = State::Timeout(0.5, true);
                    } else {
                        println!(
                            "[\x1b[33m{}\x1b[0m] Tried to attack empty space.",
                            self.name
                        );
                        let _ = self.stdin.write(b"invalid\n");
                        self.state = State::Timeout(0.5, false);
                    }
                }
                Ok(Request::Scan(x, y)) => {
                    if x > 3 || x < -3 || y > 3 || y < -3 {
                        println!(
                            "[\x1b[33m{}\x1b[0m] Tried to scan out of bounds.",
                            self.name
                        );
                        let _ = self.stdin.write(b"invalid\n");
                    } else {
                        self.scan_visuals.push(ScanVisual {
                            offset: (x, y),
                            time: 0.3,
                        });
                        let tile = grid.get(
                            (self.tile_pos.0 as i8 + x).max(0).min(GRID_WIDTH as i8 - 1) as usize,
                            (self.tile_pos.1 as i8 + y)
                                .max(0)
                                .min(GRID_HEIGHT as i8 - 1) as usize,
                        );
                        let _ = self.stdin.write(format!("tile {tile}\n").as_bytes());
                    }
                }

                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => {
                    *grid.get_mut(self.tile_pos.0, self.tile_pos.1) = Tile::Empty;
                    self.state = State::Dying(-20.0);
                }
            },
            State::Timeout(time, should_msg) => {
                *time -= get_frame_time();
                if *time <= 0.0 {
                    if *should_msg {
                        let _ = self.stdin.write(b"done\n");
                    }
                    self.state = State::WaitingForRequest;
                }
            }
            State::Dying(vy) => {
                self.x += 5.0 * get_frame_time();
                self.y += *vy * get_frame_time();
                *vy += 100.0 * get_frame_time();
            }
        }

        self.scan_visuals.retain_mut(|sv| {
            sv.time -= get_frame_time();
            sv.time > 0.0
        });
    }

    const LABEL_FONT_SIZE: u16 = 16;
    pub fn draw(&self, grid: &Grid) {
        let dx = self.x * grid.tw();
        let dy = self.y * grid.th();
        draw_rectangle(dx, dy, grid.tw(), grid.th(), self.color);

        if !matches!(self.state, State::Dying(_)) {
            draw_rectangle_lines(
                dx - 3.0 * grid.tw(),
                dy - 3.0 * grid.th(),
                grid.tw() * 7.0,
                grid.th() * 7.0,
                3.0,
                self.color,
            );
        }

        draw_text(
            &self.name,
            dx + grid.tw() / 2.0 - self.label_size.width / 2.0,
            dy - 10.0,
            Self::LABEL_FONT_SIZE as f32,
            self.color,
        );

        for sv in &self.scan_visuals {
            let color = Color {
                a: sv.time,
                ..self.color
            };
            let dx = dx + (sv.offset.0) as f32 * grid.tw();
            let dy = dy + (sv.offset.1) as f32 * grid.th();
            draw_rectangle(dx, dy, grid.tw(), grid.th(), color);
        }
    }
}

impl Drop for Character {
    fn drop(&mut self) {
        match self.process.kill() {
            Ok(_) => (),
            Err(e) => {
                println!("Failed to kill process: {e}");
            }
        }
    }
}
