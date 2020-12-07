use crate::{
    components::{Player, Position},
    map::{Map, TileType},
};
use bevy::prelude::*;
use std::cmp::{max, min};

fn try_move_player(
    delta_x: i16,
    delta_y: i16,
    map: &Res<Map>,
    query: &mut Query<With<Player, &mut Position>>,
) {
    for mut pos in query.iter_mut() {
        let new_x = min(map.width - 1, max(0, pos.x + delta_x));
        let new_y = min(map.height - 1, max(0, pos.y + delta_y));
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] == TileType::Floor {
            pos.x = new_x;
            pos.y = new_y;
        }
    }
}

pub fn player_input_system(
    keys: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut query: Query<With<Player, &mut Position>>,
) {
    if keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::H) {
        try_move_player(-1, 0, &map, &mut query);
    }
    if keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::L) {
        try_move_player(1, 0, &map, &mut query);
    }
    if keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::K) {
        try_move_player(0, -1, &map, &mut query);
    }
    if keys.pressed(KeyCode::Down) || keys.pressed(KeyCode::J) {
        try_move_player(0, 1, &map, &mut query);
    }
}
