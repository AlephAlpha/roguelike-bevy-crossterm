use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_crossterm::{CrosstermPlugin, Terminal};

fn hello_world_system(mut terminal: ResMut<Terminal>) {
    terminal.cls().unwrap();
    terminal.print(0, 0, "Hello Rust World").unwrap();
}

fn main() {
    App::build()
        .add_resource(Terminal::with_title("Roguelike Tutorial"))
        .add_system(hello_world_system.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(CrosstermPlugin)
        .run();
}
