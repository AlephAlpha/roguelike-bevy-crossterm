use crate::{
    components::{Position, Viewshed},
    map::Map,
};
use bevy::prelude::*;
use roguelike_algorithms::fov::shadow_casting_fov;

pub fn visibility_system(map: Res<Map>, mut query: Query<(&mut Viewshed, &Position)>) {
    for (mut viewshed, pos) in query.iter_mut() {
        let mut visible_tiles = Vec::new();
        shadow_casting_fov(
            &*map,
            (pos.x, pos.y),
            viewshed.range * viewshed.range,
            |point| visible_tiles.push(point),
        );
        viewshed.visible_tiles = visible_tiles;
    }
}
