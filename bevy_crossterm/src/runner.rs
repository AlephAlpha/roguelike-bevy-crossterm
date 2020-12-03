use crate::converter::convert_key_code;
use bevy_app::{App, AppExit, EventReader, Events};
use bevy_input::{keyboard::KeyboardInput, ElementState};
use crossterm::{
    event::{self, Event},
    Result,
};
use std::{mem, time::Duration};

const TIMEOUT: Duration = Duration::from_millis(50);

pub fn crossterm_runner(mut app: App) {
    app.initialize();
    event_loop(&mut app).unwrap();
}

fn event_loop(app: &mut App) -> Result<()> {
    let mut app_exit_event_reader = EventReader::<AppExit>::default();
    let mut old_key_codes = Vec::new();
    let mut new_key_codes = Vec::new();

    app.update();

    loop {
        if event::poll(TIMEOUT)? {
            let event = event::read()?;

            if let Event::Key(key_event) = event {
                new_key_codes = convert_key_code(key_event);
            }

            old_key_codes.retain(|key_code| {
                if let Some(i) = new_key_codes.iter().position(|k| k == key_code) {
                    new_key_codes.swap_remove(i);
                    false
                } else {
                    true
                }
            });
        }

        {
            let mut keyboard_input_events =
                app.resources.get_mut::<Events<KeyboardInput>>().unwrap();
            for &key_code in old_key_codes.iter() {
                let input_event = KeyboardInput {
                    scan_code: 0,
                    state: ElementState::Released,
                    key_code: Some(key_code),
                };
                keyboard_input_events.send(input_event);
            }
            for &key_code in new_key_codes.iter() {
                let input_event = KeyboardInput {
                    scan_code: 0,
                    state: ElementState::Pressed,
                    key_code: Some(key_code),
                };
                keyboard_input_events.send(input_event);
            }
        }

        mem::swap(&mut new_key_codes, &mut old_key_codes);
        new_key_codes.clear();

        if let Some(app_exit_events) = app.resources.get_mut::<Events<AppExit>>() {
            if app_exit_event_reader.latest(&app_exit_events).is_some() {
                break;
            }
        }

        app.update();
    }
    Ok(())
}
