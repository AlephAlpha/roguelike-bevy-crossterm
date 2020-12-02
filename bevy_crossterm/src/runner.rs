use crate::converter::convert_key_code;
use bevy_app::{App, AppExit, EventReader, Events};
use bevy_input::{keyboard::KeyboardInput, ElementState};
use crossterm::{
    event::{self, Event},
    Result,
};
use std::time::Duration;

pub fn crossterm_runner(mut app: App) {
    app.initialize();
    event_loop(&mut app).unwrap();
}

fn event_loop(app: &mut App) -> Result<()> {
    const TIMEOUT: Duration = Duration::from_millis(50);
    let mut app_exit_event_reader = EventReader::<AppExit>::default();

    app.update();

    loop {
        if event::poll(TIMEOUT)? {
            let event = event::read()?;

            if let Event::Key(key_event) = event {
                let mut keyboard_input_events =
                    app.resources.get_mut::<Events<KeyboardInput>>().unwrap();
                for key_code in convert_key_code(key_event) {
                    let input_event = KeyboardInput {
                        scan_code: 0,
                        state: ElementState::Pressed,
                        key_code: Some(key_code),
                    };
                    keyboard_input_events.send(input_event);
                }
            }
        }

        if let Some(app_exit_events) = app.resources.get_mut::<Events<AppExit>>() {
            if app_exit_event_reader.latest(&app_exit_events).is_some() {
                break;
            }
        }

        app.update();
    }
    Ok(())
}
