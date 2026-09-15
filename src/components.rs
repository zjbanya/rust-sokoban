use std::{collections::HashMap, fmt::Display, time::Duration};

use ggez::{
    Context,
    audio::{self, SoundSource},
};

use crate::events::Event;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
// ANCHOR: box_colour
pub enum BoxColor {
    Red,
    Blue,
}
// ANCHOR_END: box_colour

#[derive(Default)]
pub enum GameplayState {
    #[default]
    Playing,
    Won,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

pub enum Renderablekind {
    Static,
    Animated,
}

pub struct Renderable {
    paths: Vec<String>,
}

impl Renderable {
    pub fn new_static(path: &str) -> Self {
        Self {
            paths: vec![path.to_string()],
        }
    }

    pub fn new_animated(paths: Vec<&str>) -> Self {
        Self {
            paths: paths.iter().map(|p| p.to_string()).collect(),
        }
    }

    pub fn kind(&self) -> Renderablekind {
        match self.paths.len() {
            0 => panic!("invalid renderable"),
            1 => Renderablekind::Static,
            _ => Renderablekind::Animated,
        }
    }

    pub fn path(&self, path_index: usize) -> String {
        // 如果请求的路径索引超出了实际拥有的路径数量，
        // 我们只需将该索引对路径总数取模，
        // 即可得到一个在有效范围内的索引。
        self.paths[path_index % self.paths.len()].clone()
    }
}

pub struct Wall {}

pub struct Player {}

// ANCHOR: box
pub struct Box {
    pub color: BoxColor,
}

pub struct BoxSpot {
    pub color: BoxColor,
}
// ANCHOR_END: box

pub struct Movable;

pub struct Immovable;

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}

impl Display for GameplayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GameplayState::Playing => "playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}

// ANCHOR: box_colour_display
impl Display for BoxColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BoxColor::Red => "red",
            BoxColor::Blue => "blue",
        })?;
        Ok(())
    }
}
// ANCHOR_END: box_colour_display

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

#[derive(Default)]
pub struct AudioStore {
    pub sounds: HashMap<String, std::boxed::Box<audio::Source>>,
}

impl AudioStore {
    pub fn play_sound(&mut self, context: &mut Context, sound: &str) {
        if let Some(source) = self.sounds.get_mut(sound) {
            if source.play_detached(context).is_ok() {
                println!("Playing sound: {}", sound);
            }
        }
    }
}
