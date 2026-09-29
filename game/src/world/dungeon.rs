use common::Tile;

use super::grid::{Grid, Rect};

const BST_MAX_DEPTH: usize = 4;
const BST_MIN_DEPTH: usize = 3;
const MIN_ROOM_SIZE: usize = 5;
// const BST_MAX_DEPTH: usize = 8;
// const BST_MIN_DEPTH: usize = 7;
// const MIN_ROOM_SIZE: usize = 10;

impl Grid {
    pub(super) fn gen_bst(&mut self, rect: Rect, depth: usize) {
        let vertical = depth % 2 == 0;
        let size = if vertical { rect.h } else { rect.w };

        if size < MIN_ROOM_SIZE * 2
            || depth >= BST_MAX_DEPTH
            || (depth > BST_MIN_DEPTH && rand::random_bool(1. / 3.))
        {
            for x in rect.x..rect.x + rect.w {
                *self.get_mut(x, rect.y) = Tile::Wall;
                *self.get_mut(x, rect.y + rect.h - 1) = Tile::Wall;
            }
            for y in rect.y + 1..rect.y + rect.h - 1 {
                *self.get_mut(rect.x, y) = Tile::Wall;
                *self.get_mut(rect.x + rect.w - 1, y) = Tile::Wall;
            }

            let max_coins = (rect.w * rect.h / 70).max(2);
            for _ in 0..rand::random_range(max_coins.saturating_sub(2)..max_coins) {
                *self.get_mut(
                    rand::random_range(rect.x + 2..rect.x + rect.w - 2),
                    rand::random_range(rect.y + 2..rect.y + rect.h - 2),
                ) = Tile::Coins;
            }
        } else {
            let split = rand::random_range(MIN_ROOM_SIZE..=size - MIN_ROOM_SIZE);
            self.gen_bst(
                Rect {
                    x: rect.x,
                    y: rect.y,
                    w: if vertical { rect.w } else { rect.w - split },
                    h: if vertical { rect.h - split } else { rect.h },
                },
                depth + 1,
            );
            self.gen_bst(
                Rect {
                    x: if vertical {
                        rect.x
                    } else {
                        rect.x + rect.w - split
                    },
                    y: if vertical {
                        rect.y + rect.h - split
                    } else {
                        rect.y
                    },
                    w: if vertical { rect.w } else { split },
                    h: if vertical { split } else { rect.h },
                },
                depth + 1,
            );

            // Make a corridor
            if vertical {
                *self.get_mut(rect.x + rect.w / 2, rect.y + rect.h - split + 1) = Tile::Empty;
                *self.get_mut(rect.x + rect.w / 2, rect.y + rect.h - split) = Tile::Empty;
                *self.get_mut(rect.x + rect.w / 2, rect.y + rect.h - split - 1) = Tile::Empty;
                *self.get_mut(rect.x + rect.w / 2, rect.y + rect.h - split - 2) = Tile::Empty;
            } else {
                *self.get_mut(rect.x + rect.w - split + 1, rect.y + rect.h / 2) = Tile::Empty;
                *self.get_mut(rect.x + rect.w - split, rect.y + rect.h / 2) = Tile::Empty;
                *self.get_mut(rect.x + rect.w - split - 1, rect.y + rect.h / 2) = Tile::Empty;
                *self.get_mut(rect.x + rect.w - split - 2, rect.y + rect.h / 2) = Tile::Empty;
            }
        }
    }
}
