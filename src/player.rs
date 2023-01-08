use crate::enemy::*;
use crate::tile::*;
use macroquad::{input, prelude::*, ui::root_ui};

pub struct Player {
    block_timer: f32,
    attack_anim_start: bool,
    block: bool,
    health: i32,
    size: Vec2,
    pos: Vec2,
    speed: f32,
    texture: Texture2D,
    sword_pos: Vec2,
    invulnerable: bool,
    damage_timer: f32,
    can_attack: bool,
    strike_pos: Vec2,
}

impl Player {
    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn set_health(&mut self, health: i32) {
        self.health = health;
    }

    pub fn new(x: f32, y: f32, w: f32, h: f32, speed: f32) -> Player {
        Player {
            attack_anim_start: false,
            can_attack: true,
            block_timer: 0.0,
            block: false,
            sword_pos: Vec2::ZERO,
            strike_pos: Vec2::ZERO,
            damage_timer: 0.0,
            invulnerable: false,
            health: 3,
            size: Vec2::new(w, h),
            pos: Vec2::new(x, y),
            speed,
            texture: Texture2D::empty(),
        }
    }

    pub fn attack(&mut self, enemies: &mut Vec<Enemy>, camera: Camera2D) {
        let mouse_pos = camera.screen_to_world(vec2(mouse_position().0, mouse_position().1));
        let dir = (mouse_pos - self.pos).normalize();
        if self.can_attack && is_mouse_button_pressed(MouseButton::Left) && !self.block {
            self.can_attack = false;
            self.strike_pos = self.sword_pos + dir * 20.0;
        }

        if !self.can_attack {
            for enemy in enemies.iter_mut() {
                if enemy.pos().distance(self.sword_pos) < 19.0 {
                    enemy.take_damage();
                }
            }
        }
    }

    pub fn draw_weapon(&mut self, texture: Texture2D, camera: Camera2D) {
        let mouse_pos = camera.screen_to_world(vec2(mouse_position().0, mouse_position().1));
        let dir = (mouse_pos - self.pos).normalize();

        let flip_y = mouse_pos.y > self.pos.y;

        if !self.can_attack {
            if !self.attack_anim_start {
                self.sword_pos = self.sword_pos.lerp(self.strike_pos, 0.1);
            } else {
                self.sword_pos = self.sword_pos.lerp(self.pos + dir * 23.0, 0.5);
                if self.sword_pos.distance(self.pos + dir * 23.0) < 3.0 {
                    self.attack_anim_start = false;
                    self.can_attack = true;
                }
            }

            if self.sword_pos.distance(self.strike_pos) < 5.0 {
                self.attack_anim_start = true;
            }
        } else {
            self.sword_pos = self.pos + dir * 23.0;
        }

        draw_texture_ex(
            texture,
            self.sword_pos.x,
            self.sword_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                pivot: Some(self.sword_pos),
                flip_y,
                ..Default::default()
            },
        );
    }

    pub fn draw(&mut self) {
        if self.invulnerable {
            self.damage_timer += get_frame_time();

            if self.damage_timer > 1.5 {
                self.damage_timer = 0.0;
                self.invulnerable = false;
            }

            draw_texture_ex(
                self.texture,
                self.pos.x,
                self.pos.y,
                RED,
                DrawTextureParams {
                    dest_size: Option::Some(self.size),
                    ..Default::default()
                },
            );
        } else {
            draw_texture_ex(
                self.texture,
                self.pos.x,
                self.pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Option::Some(self.size),
                    ..Default::default()
                },
            );
        }

        if self.block {
            self.block_timer += get_frame_time();
            if self.block_timer > 2.0 {
                self.block = false;
                self.block_timer = 3.0;
            }

            draw_circle_lines(self.pos.x + 16.0, self.pos.y + 16.0, 16.0, 3.0, BLUE);
        } else {
            if self.block_timer > 0.0 {
                root_ui().label(
                    vec2(10.0, 30.0),
                    format!("Shield in: {} seconds", self.block_timer as i32 + 1).as_str(),
                );
                self.block_timer -= get_frame_time();
            }

            self.block_timer = self.block_timer.max(0.0);
        }
    }

    pub fn take_damage(&mut self) {
        if !self.invulnerable && !self.block {
            self.invulnerable = true;
            self.health -= 1;
            self.health = self.health.max(0);
        }
    }

    pub fn draw_hearts(&self) {
        root_ui().label(vec2(10.0, 7.0), "Health:");

        if self.health > 0 {
            for x in 0..self.health {
                root_ui().canvas().rect(
                    Rect::new(65.0 + (x as f32 * 15.0), 10.0, 10.0, 10.0),
                    None,
                    Some(RED),
                );
            }
        }
    }

    pub fn spawn_player(&mut self, tiles: &Vec<Tile>) {
        for tile in tiles.iter() {
            if let TileType::PlayerSpawn(_) = tile.tile_type() {
                self.pos = tile.pos();
                break;
            }
        }
    }

    pub fn set_texture(&mut self, texture: Texture2D) {
        self.texture = texture;
        self.texture.set_filter(FilterMode::Nearest);
    }

    pub fn move_player(&mut self, tiles: &Vec<Tile>) {
        let mut velocity = Vec2::ZERO;
        if is_key_down(input::KeyCode::W) {
            velocity.y -= 1.0;
        }
        if is_key_down(input::KeyCode::S) {
            velocity.y += 1.0;
        }
        if is_key_down(input::KeyCode::A) {
            velocity.x -= 1.0;
        }
        if is_key_down(input::KeyCode::D) {
            velocity.x += 1.0;
        }

        if is_mouse_button_pressed(MouseButton::Right)
            && !self.block
            && self.block_timer <= 0.0
            && self.can_attack
        {
            self.block = true;
        }

        let mut new_pos = self.pos + velocity.normalize_or_zero() * self.speed * get_frame_time();

        if velocity.x <= 0.0 {
            if get_tile(new_pos.x - 15.0, self.pos.y, &tiles) {
                new_pos.x = self.pos.x;
                velocity.x = 0.0;
            }
        } else {
            if get_tile(new_pos.x + 15.0, self.pos.y, &tiles) {
                new_pos.x = self.pos.x;
                velocity.x = 0.0;
            }
        }

        if velocity.y <= 0.0 {
            if get_tile(self.pos.x, new_pos.y - 15.0, &tiles) {
                new_pos.y = self.pos.y;
                velocity.y = 0.0;
            }
        } else {
            if get_tile(self.pos.x, new_pos.y + 15.0, &tiles) {
                new_pos.y = self.pos.y;
                velocity.y = 0.0;
            }
        }

        self.pos = new_pos;
    }
}
