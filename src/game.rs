use dungeon_game::{editor::tilemap_editor::*, player::player::*, utility::*};
use macroquad::prelude::*;
use std::process;

pub enum GameState {
    EditorMode,
    Play,
}

pub struct Game {
    game_state: GameState,
    game_camera: Camera2D,
    debug_collision: bool,
    enemies: Vec<Vec2>,
    player: Player,
    enemy_sprite_a: Texture2D,
    level_atlas: Texture2D,
    editor: TileMapEditor,
}

impl Game {
    pub async fn build() -> Game {
        let mut player = Player::new(20.0, 20.0, 32.0, 32.0, 100.0);

        player.set_texture(
            create_texture("res/textures/player.png")
                .await
                .unwrap_or_else(|err| {
                    println!("{err}");
                    process::exit(1);
                }),
        );

        let enemy_sprite_a = create_texture("res/textures/sword_enemy.png")
            .await
            .unwrap_or_else(|err| {
                println!("{err}");
                process::exit(1);
            });

        let level_atlas = create_texture("res/textures/tilemap_packed.png")
            .await
            .unwrap_or_else(|err| {
                println!("{err}");
                process::exit(1);
            });

        let mut editor = TileMapEditor::new(0.0, 0.0, 1.0, 1.0, 16.0, (10, 10), (1.0, 1.0));
        editor.set_texture(level_atlas.clone());

        let player_cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 600.0, 600.0));

        Game {
            game_state: GameState::EditorMode,
            debug_collision: true,
            enemies: Vec::new(),
            player,
            enemy_sprite_a,
            level_atlas,
            game_camera: player_cam,
            editor,
        }
    }

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Tab) && is_key_down(KeyCode::LeftControl) {
            if let GameState::EditorMode = self.game_state {
                self.game_state = GameState::Play;
            } else {
                self.game_state = GameState::EditorMode;
            }

            self.enemies.clear();
            self.player.spawn_player(&self.editor.tiles);
            spawn_enemy(&self.editor.tiles, &mut self.enemies, self.player.pos(), 1);
        }
    }

    pub fn handle_states(&mut self) {
        match self.game_state {
            GameState::EditorMode => {
                self.editor.editor_camera.update_camera();

                draw_map(
                    &mut self.editor.tiles,
                    self.level_atlas,
                    self.debug_collision,
                );

                if is_key_pressed(KeyCode::F1) {
                    self.editor.switch_mode(EditorMode::None);
                }
                if is_key_pressed(KeyCode::F2) {
                    self.editor.switch_mode(EditorMode::Paint);
                }

                if is_key_pressed(KeyCode::F4) {
                    self.debug_collision = !self.debug_collision;
                }

                set_default_camera();
                self.editor.show_editors();
            }
            GameState::Play => {
                self.game_camera.target = self.game_camera.target.lerp(self.player.pos(), 0.1);
                self.game_camera.target = self.game_camera.target.round();
                set_camera(&self.game_camera);

                draw_map(
                    &mut self.editor.tiles,
                    self.level_atlas,
                    self.debug_collision,
                );

                for enemy in &self.enemies {
                    draw_texture_ex(
                        self.enemy_sprite_a,
                        enemy.x,
                        enemy.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(32.0, 32.0)),
                            ..Default::default()
                        },
                    )
                }

                self.player.draw();
                self.player.move_player(&self.editor.tiles);
            }
        }
    }
}
