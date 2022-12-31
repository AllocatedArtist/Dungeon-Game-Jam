use macroquad::prelude::*;

#[macro_use]
extern crate serde_derive;

pub mod editor;
pub mod enemy;
pub mod player;
pub mod serialization;
pub mod tile;
pub mod utility;

pub fn setup_window() -> Conf {
    Conf {
        window_title: String::from("Dungeon Game"),
        fullscreen: false,
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}
