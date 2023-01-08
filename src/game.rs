use dungeon_game::{editor::tilemap_editor::*, enemy::*, player::*, utility::*};
use macroquad::prelude::*;
use std::process;

use std::collections::VecDeque;

pub enum GameState {
    EditorMode,
    Play,
}

pub struct Game {
    game_state: GameState,
    game_camera: Camera2D,
    player_sword_sprite: Texture2D,
    debug_collision: bool,
    enemies: Vec<Enemy>,
    player: Player,
    enemy_sword_sprite: Texture2D,
    enemy_sprite_a: Texture2D,
    level_atlas: Texture2D,
    editor: TileMapEditor,
    _debug_path: VecDeque<Vec2>,
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

        let enemy_sword_sprite = create_texture("res/textures/enemy_sword.png")
            .await
            .unwrap_or_else(|err| {
                println!("{err}");
                process::exit(1);
            });

        let player_sword_sprite = create_texture("res/textures/player_sword.png")
            .await
            .unwrap_or_else(|err| {
                println!("{err}");
                process::exit(1);
            });

        let mut editor = TileMapEditor::new(0.0, 0.0, 1.0, 1.0, 16.0, (10, 10), (1.0, 1.0));
        editor.set_texture(level_atlas.clone());

        let player_cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 600.0, 600.0));

        Game {
            player_sword_sprite,
            game_state: GameState::EditorMode,
            debug_collision: true,
            enemies: Vec::new(),
            player,
            enemy_sprite_a,
            enemy_sword_sprite,
            level_atlas,
            game_camera: player_cam,
            editor,
            _debug_path: VecDeque::new(),
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
            spawn_enemy(&self.editor.tiles, &mut self.enemies, self.player.pos(), 5);
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
                if self.player.health() <= 0 {
                    self.player.set_health(3);
                    self.game_state = GameState::EditorMode;
                }

                self.game_camera.target = self.game_camera.target.lerp(self.player.pos(), 0.1);
                self.game_camera.target = self.game_camera.target.round();
                set_camera(&self.game_camera);

                draw_map(
                    &mut self.editor.tiles,
                    self.level_atlas,
                    self.debug_collision,
                );

                for enemy in self.enemies.iter_mut() {
                    if enemy.pos().distance(self.player.pos()) < 60.0 {
                        enemy.set_player_spotted(true);
                    }

                    if enemy.player_spotted() {
                        enemy.move_to(self.player.pos(), &self.editor.tiles);
                        enemy.damage_player(&mut self.player);
                    }

                    enemy.draw(self.enemy_sprite_a);
                    enemy.draw_weapon(self.enemy_sword_sprite, self.player.pos());
                }

                let enemy_list_cloned = self.enemies.clone().to_owned();
                for (index, enemy) in enemy_list_cloned.iter().enumerate() {
                    if enemy.health() <= 0 {
                        self.enemies.remove(index);
                    }
                }

                let enemy_count = self.enemies.len();
                let enemy_list_cloned = self.enemies.clone().to_owned();
                let mut enemies = self.enemies.iter_mut();

                for i in 0..enemy_count {
                    if let Some(enemy) = enemies.next() {
                        for (index, enemy2) in enemy_list_cloned.iter().enumerate() {
                            if i != index {
                                enemy.displace(enemy2);
                            }
                        }
                    }
                }

                self.player.draw();
                self.player
                    .draw_weapon(self.player_sword_sprite, self.game_camera);
                self.player.attack(&mut self.enemies, self.game_camera);
                self.player.draw_hearts();
                self.player.move_player(&self.editor.tiles);
            }
        }
    }
}
