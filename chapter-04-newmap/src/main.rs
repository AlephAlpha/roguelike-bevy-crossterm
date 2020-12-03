use crate::{
    components::{Player, Position, Renderable},
    map::{draw_map, Map},
    player::player_input_system,
};
use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_crossterm::{
    crossterm::style::{Color, Colors},
    CrosstermPlugin, Terminal,
};
use rand::{rngs::StdRng, SeedableRng};

mod components;
mod map;
mod player;
mod rect;

fn spawn_player(mut commands: Commands, map: Res<Map>) {
    let (player_x, player_y) = map.rooms[0].center();

    commands.spawn((
        Position {
            x: player_x,
            y: player_y,
        },
        Renderable {
            glyph: '@',
            fg: Some(Color::Yellow),
            bg: None,
        },
        Player {},
    ));
}

fn render_system(mut term: ResMut<Terminal>, map: Res<Map>, data: Query<(&Position, &Renderable)>) {
    draw_map(&mut term, &map);

    for (pos, render) in data.iter() {
        term.put_char_with_color(
            pos.x as u16,
            pos.y as u16,
            render.glyph,
            Colors {
                foreground: render.fg,
                background: render.bg,
            },
        );
    }
}

fn main() {
    App::build()
        .add_resource(Terminal::with_title("Roguelike Tutorial"))
        .add_resource(StdRng::from_entropy())
        .init_resource::<Map>()
        .add_startup_system(spawn_player.system())
        .add_system(exit_on_esc_system.system())
        .add_system(player_input_system.system())
        .add_system(render_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(CrosstermPlugin)
        .run();
}
