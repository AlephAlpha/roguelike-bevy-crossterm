use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_crossterm::{
    crossterm::style::{Color, Colors},
    CrosstermPlugin, Terminal,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
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
struct Player {}

#[derive(PartialEq, Copy, Clone, Debug)]
enum TileType {
    Wall,
    Floor,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<TileType>,
}

impl FromResources for Map {
    fn from_resources(resources: &Resources) -> Self {
        let mut rng = resources.get_mut::<StdRng>().unwrap();

        let mut tiles = vec![TileType::Floor; 80 * 24];

        // Make the boundaries walls
        for x in 0..80 {
            tiles[xy_idx(x, 0)] = TileType::Wall;
            tiles[xy_idx(x, 23)] = TileType::Wall;
        }
        for y in 0..24 {
            tiles[xy_idx(0, y)] = TileType::Wall;
            tiles[xy_idx(79, y)] = TileType::Wall;
        }

        for _i in 0..400 {
            let x = rng.gen_range(0, 80);
            let y = rng.gen_range(0, 24);
            let idx = xy_idx(x, y);
            if idx != xy_idx(40, 13) {
                tiles[idx] = TileType::Wall;
            }
        }

        Map { tiles }
    }
}

fn xy_idx(x: i16, y: i16) -> usize {
    (y as usize * 80) + x as usize
}

fn spawn_player(mut commands: Commands) {
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

fn clear_screen_system(mut term: ResMut<Terminal>) {
    term.cls();
}

fn render_system(mut term: ResMut<Terminal>, data: Query<(&Position, &Renderable)>) {
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

fn try_move_player(
    delta_x: i16,
    delta_y: i16,
    map: &Res<Map>,
    data: &mut Query<With<Player, &mut Position>>,
) {
    for mut pos in data.iter_mut() {
        let new_x = min(79, max(0, pos.x + delta_x));
        let new_y = min(23, max(0, pos.y + delta_y));
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] == TileType::Floor {
            pos.x = new_x;
            pos.y = new_y;
        }
    }
}

fn player_input_system(
    keys: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut data: Query<With<Player, &mut Position>>,
) {
    if keys.pressed(KeyCode::Left) {
        try_move_player(-1, 0, &map, &mut data);
    }
    if keys.pressed(KeyCode::Right) {
        try_move_player(1, 0, &map, &mut data);
    }
    if keys.pressed(KeyCode::Up) {
        try_move_player(0, -1, &map, &mut data);
    }
    if keys.pressed(KeyCode::Down) {
        try_move_player(0, 1, &map, &mut data);
    }
}

fn draw_map_system(mut term: ResMut<Terminal>, map: Res<Map>) {
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

fn main() {
    let mut app_builder = App::build();

    app_builder
        .add_resource(Terminal::with_title("Roguelike Tutorial"))
        .add_resource(StdRng::from_entropy());

    app_builder
        .init_resource::<Map>()
        .add_startup_system(spawn_player.system())
        .add_system(exit_on_esc_system.system())
        .add_system(player_input_system.system())
        .add_system(clear_screen_system.system())
        .add_system(draw_map_system.system())
        .add_system(render_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(CrosstermPlugin)
        .run();
}
