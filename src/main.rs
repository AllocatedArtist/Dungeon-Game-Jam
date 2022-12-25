use macroquad::prelude::*;
use std::process;

use dungeon_game::*;

#[macroquad::main(setup_window)]
async fn main() {
    let mut player = Player::new(20.0, 20.0, 32.0, 32.0, 100.0);

    player.set_texture(
        create_texture("res/textures/player.png")
            .await
            .unwrap_or_else(|err| {
                println!("{err}");
                process::exit(1);
            }),
    );

    let tilemap = create_texture("res/levels/tilemap_packed.png")
        .await
        .unwrap_or_else(|err| {
            println!("{err}");
            process::exit(1);
        });

    tilemap.set_filter(FilterMode::Nearest);

    let mut editor = TileMapEditor::new(0.0, 0.0, 1.0, 1.0, 16.0, (10, 10), (1.0, 1.0));
    editor.set_texture(tilemap.clone());

    loop {
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::F1) {
            editor.switch_mode(EditorMode::None);
        }
        if is_key_pressed(KeyCode::F2) {
            editor.switch_mode(EditorMode::Paint);
        }
        if is_key_pressed(KeyCode::F3) {
            editor.switch_mode(EditorMode::Save);
        }

        editor.editor_camera.update_camera();

        player.draw();
        player.move_player();

        draw_map(&mut editor.tiles, tilemap);

        set_default_camera();

        editor.show_editors();

        next_frame().await
    }
}
