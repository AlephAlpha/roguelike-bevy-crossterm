use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_crossterm::{
    crossterm::style::{Color, Colors},
    CrosstermPlugin, Terminal,
};
use std::cmp::{max, min};

#[derive(Clone, Debug)]
struct Position {
    x: i16,
    y: i16,
}

#[derive(Clone, Debug)]
struct Renderable {
    glyph: char,
    fg: Option<Color>,
    bg: Option<Color>,
}

#[derive(Clone, Debug)]
struct LeftMover {}

#[derive(Clone, Debug)]
struct Player {}

fn spawn_player(commands: &mut Commands) {
    commands.spawn((
        Position { x: 40, y: 13 },
        Renderable {
            glyph: '@',
            fg: Some(Color::Yellow),
            bg: None,
        },
        Player {},
    ));
}

fn spawn_smileys(commands: &mut Commands) {
    for i in 0..10 {
        commands.spawn((
            Position { x: i * 7, y: 10 },
            Renderable {
                glyph: '☺',
                fg: Some(Color::Red),
                bg: None,
            },
            LeftMover {},
        ));
    }
}

fn render_system(mut term: ResMut<Terminal>, data: Query<(&Position, &Renderable)>) {
    term.cls();
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

fn left_walker_system(mut data: Query<&mut Position, With<LeftMover>>) {
    for mut pos in data.iter_mut() {
        pos.x -= 1;
        if pos.x < 0 {
            pos.x = 79;
        }
    }
}

fn try_move_player(delta_x: i16, delta_y: i16, data: &mut Query<&mut Position, With<Player>>) {
    for mut pos in data.iter_mut() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

fn player_input_system(keys: Res<Input<KeyCode>>, mut data: Query<&mut Position, With<Player>>) {
    if keys.pressed(KeyCode::Left) {
        try_move_player(-1, 0, &mut data);
    }
    if keys.pressed(KeyCode::Right) {
        try_move_player(1, 0, &mut data);
    }
    if keys.pressed(KeyCode::Up) {
        try_move_player(0, -1, &mut data);
    }
    if keys.pressed(KeyCode::Down) {
        try_move_player(0, 1, &mut data);
    }
}

fn main() {
    App::build()
        .add_resource(Terminal::with_title("Roguelike Tutorial"))
        .add_startup_system(spawn_player.system())
        .add_startup_system(spawn_smileys.system())
        .add_system(exit_on_esc_system.system())
        .add_system(player_input_system.system())
        .add_system(left_walker_system.system())
        .add_system(render_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(CrosstermPlugin)
        .run();
}
