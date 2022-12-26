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

    let tilemap = create_texture("res/textures/tilemap_packed.png")
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

    let mut enemies: Vec<Vec2> = Vec::new();

    let enemy_sprite = create_texture("res/textures/sword_enemy.png")
        .await
        .unwrap_or_else(|err| {
            println!("{err}");
            process::exit(1);
        });

    enemy_sprite.set_filter(FilterMode::Nearest);

    loop {
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::Tab) && is_key_down(KeyCode::LeftControl) {
            use_editor = !use_editor;

            enemies.clear();

            player.spawn_player(&editor.tiles);

            spawn_enemy(&editor.tiles, &mut enemies, player.pos(), 1);
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

        if !use_editor {
            for enemy in &enemies {
                draw_texture_ex(
                    enemy_sprite,
                    enemy.x,
                    enemy.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(32.0, 32.0)),
                        ..Default::default()
                    },
                )
            }

            player.draw();
            player.move_player(&editor.tiles);
        }

        set_default_camera();

        if use_editor {
            editor.show_editors();
        }

        next_frame().await
    }
}

fn spawn_enemy(tiles: &Vec<Tile>, enemies: &mut Vec<Vec2>, player_pos: Vec2, enemy_count: usize) {
    const FACTOR: f32 = 32.0 * 5.0;

    for tile in tiles.iter() {
        if let TileType::Floor(_) = tile.tile_type() {
            if rand::gen_range(0.0, 5.0) < 1.0 {
                if tile.pos().x > player_pos.x + FACTOR
                    || tile.pos().x < player_pos.x - FACTOR
                    || tile.pos().y > player_pos.y + FACTOR
                    || tile.pos().y < player_pos.y - FACTOR
                {
                    if enemies.len() < enemy_count {
                        enemies.push(tile.pos());
                    }
                }
            }
        }
    }
}
