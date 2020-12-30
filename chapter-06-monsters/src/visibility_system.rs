use crate::{
    components::{Player, Position, Viewshed},
    map::Map,
};
use bevy::prelude::*;
use roguelike_algorithms::fov::shadow_casting_fov;

pub fn visibility_system(
    mut map: ResMut<Map>,
    mut query: Query<(Option<&Player>, &mut Viewshed, &Position)>,
) {
    for (player, mut viewshed, pos) in query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;
            let mut visible_tiles = Vec::new();
            shadow_casting_fov(&*map, (pos.x, pos.y), viewshed.range, |point| {
                visible_tiles.push(point)
            });
            viewshed.visible_tiles = visible_tiles;

            if player.is_some() {
                for t in map.visible_tiles.iter_mut() {
                    *t = false;
                }
                for &(x, y) in viewshed.visible_tiles.iter() {
                    let idx = map.xy_idx(x, y);
                    map.revealed_tiles[idx] = true;
                    map.visible_tiles[idx] = true;
                }
            }
        }
    }
}
