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

    let mut debug_collision = true;

    let mut use_editor = true;

    let mut player_cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 600.0, 600.0));

    loop {
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::Tab) && is_key_down(KeyCode::LeftControl) {
            player.spawn_player(&editor.tiles);
            use_editor = !use_editor;
        }

        if use_editor {
            editor.editor_camera.update_camera();

            if is_key_pressed(KeyCode::F1) {
                editor.switch_mode(EditorMode::None);
            }
            if is_key_pressed(KeyCode::F2) {
                editor.switch_mode(EditorMode::Paint);
            }

            if is_key_pressed(KeyCode::F4) {
                debug_collision = !debug_collision;
            }
        } else {
            player_cam.target = player_cam.target.lerp(player.pos(), 0.1);
            player_cam.target = player_cam.target.round();
            set_camera(&player_cam);
        }

        draw_map(&mut editor.tiles, tilemap, debug_collision);

        player.draw();
        player.move_player(&editor.tiles);

        set_default_camera();

        if use_editor {
            editor.show_editors();
        }

        next_frame().await
    }
}
