use macroquad::prelude::*;

pub mod game;

use crate::game::*;
use dungeon_game::setup_window;

#[macroquad::main(setup_window)]
async fn main() {
    let mut game = Game::build().await;

    loop {
        clear_background(LIGHTGRAY);

        game.update();
        game.handle_states();

        next_frame().await
    }
}
