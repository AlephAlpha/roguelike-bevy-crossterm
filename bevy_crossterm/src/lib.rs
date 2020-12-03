use bevy_app::{AppBuilder, Plugin};

mod converter;
mod runner;
mod terminal;

pub use crossterm;
pub use runner::crossterm_runner;
pub use terminal::Terminal;

#[derive(Debug, Default)]
pub struct CrosstermPlugin;

impl Plugin for CrosstermPlugin {
    fn build(&self, app: &mut AppBuilder) {
        if app.resources().get::<Terminal>().is_none() {
            app.init_resource::<Terminal>();
        }
        app.set_runner(crossterm_runner);
    }
}
