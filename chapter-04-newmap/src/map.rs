use crate::rect::Rect;
use bevy::prelude::*;
use bevy_crossterm::{
    crossterm::style::{Color, Colors},
    Terminal,
};
use rand::{rngs::StdRng, Rng};
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
}

impl FromResources for Map {
    fn from_resources(resources: &Resources) -> Self {
        let mut rng = resources.get_mut::<StdRng>().unwrap();

        let mut tiles = vec![TileType::Wall; 80 * 24];
        let mut rooms: Vec<Rect> = Vec::new();

        const MAX_ROOMS: u8 = 30;
        const MIN_SIZE: i16 = 6;
        const MAX_SIZE: i16 = 10;

        for _ in 0..MAX_ROOMS {
            let w = rng.gen_range(MIN_SIZE..MAX_SIZE);
            let h = rng.gen_range(MIN_SIZE..MAX_SIZE) / 2;
            let x = rng.gen_range(1..80 - w - 1);
            let y = rng.gen_range(1..24 - h - 1);
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                apply_room_to_map(&new_room, &mut tiles);

                if !rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                    if rng.gen() {
                        apply_horizontal_tunnel(&mut tiles, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut tiles, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut tiles, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut tiles, prev_x, new_x, new_y);
                    }
                }

                rooms.push(new_room);
            }
        }

        Map { tiles, rooms }
    }
}

pub fn xy_idx(x: i16, y: i16) -> usize {
    (y as usize * 80) + x as usize
}

fn apply_room_to_map(room: &Rect, tiles: &mut [TileType]) {
    for y in room.y1..room.y2 {
        for x in room.x1..room.x2 {
            tiles[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(tiles: &mut [TileType], x1: i16, x2: i16, y: i16) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx < 80 * 24 {
            tiles[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(tiles: &mut [TileType], y1: i16, y2: i16, x: i16) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx < 80 * 24 {
            tiles[idx as usize] = TileType::Floor;
        }
    }
}

pub fn draw_map_system(mut term: ResMut<Terminal>, map: Res<Map>) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.tiles.iter() {
        match tile {
            TileType::Floor => {
                term.put_char_with_color(
                    x,
                    y,
                    '.',
                    Colors {
                        foreground: Some(Color::Grey),
                        background: None,
                    },
                );
            }
            TileType::Wall => {
                term.put_char_with_color(
                    x,
                    y,
                    '#',
                    Colors {
                        foreground: Some(Color::Green),
                        background: None,
                    },
                );
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
