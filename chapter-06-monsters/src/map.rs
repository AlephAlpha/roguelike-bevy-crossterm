use crate::rect::Rect;
use bevy::prelude::*;
use bevy_crossterm::{
    crossterm::style::{Color, Colors},
    Terminal,
};
use rand::{rngs::StdRng, Rng};
use roguelike_algorithms::fov::Map2D;
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Debug)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i16,
    pub height: i16,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    pub fn xy_idx(&self, x: i16, y: i16) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1..room.y2 {
            for x in room.x1..room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i16, x2: i16, y: i16) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i16, y2: i16, x: i16) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
}

impl FromResources for Map {
    fn from_resources(resources: &Resources) -> Self {
        let mut rng = resources.get_mut::<StdRng>().unwrap();

        let mut map = Map {
            tiles: vec![TileType::Wall; 80 * 24],
            rooms: Vec::new(),
            width: 80,
            height: 24,
            revealed_tiles: vec![false; 80 * 24],
            visible_tiles: vec![false; 80 * 24],
        };

        const MAX_ROOMS: u8 = 30;
        const MIN_SIZE: i16 = 6;
        const MAX_SIZE: i16 = 10;

        for _ in 0..MAX_ROOMS {
            let w = rng.gen_range(MIN_SIZE..MAX_SIZE);
            let h = rng.gen_range(MIN_SIZE..MAX_SIZE) / 2;
            let x = rng.gen_range(1..map.width - w - 1);
            let y = rng.gen_range(1..map.height - h - 1);
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.gen() {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }
}

impl Map2D for Map {
    fn is_opaque(&self, (x, y): (i16, i16)) -> bool {
        let idx = self.xy_idx(x, y);
        self.tiles[idx] == TileType::Wall
    }

    fn is_in_bound(&self, (x, y): (i16, i16)) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
}

pub fn draw_map_system(mut term: ResMut<Terminal>, map: Res<Map>) {
    let mut y = 0;
    let mut x = 0;
    for (idx, tile) in map.tiles.iter().enumerate() {
        // Render a tile depending upon the tile type
        if map.revealed_tiles[idx] {
            let glyph;
            let fg;
            match tile {
                TileType::Floor => {
                    glyph = '.';
                    fg = if map.visible_tiles[idx] {
                        Color::DarkCyan
                    } else {
                        Color::AnsiValue(8)
                    };
                }
                TileType::Wall => {
                    glyph = '#';
                    fg = if map.visible_tiles[idx] {
                        Color::Green
                    } else {
                        Color::AnsiValue(7)
                    };
                }
            }
            term.put_char_with_color(
                x,
                y,
                glyph,
                Colors {
                    foreground: Some(fg),
                    background: None,
                },
            );
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
