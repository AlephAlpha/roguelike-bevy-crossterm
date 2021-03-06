use bevy_crossterm::crossterm::style::Color;

#[derive(Clone, Debug)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Debug)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

#[derive(Clone, Debug)]
pub struct Player {}
