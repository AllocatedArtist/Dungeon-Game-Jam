use crate::tile::*;
use macroquad::{input, prelude::*};

pub struct Player {
    size: Vec2,
    pos: Vec2,
    speed: f32,
    texture: Texture2D,
}

impl Player {
    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn new(x: f32, y: f32, w: f32, h: f32, speed: f32) -> Player {
        Player {
            size: Vec2::new(w, h),
            pos: Vec2::new(x, y),
            speed,
            texture: Texture2D::empty(),
        }
    }

    pub fn draw(&self) {
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

        let get_tile = |x: f32, y: f32| {
            let result_x = ((x + 15.0) as i32 / 32) * 32;
            let result_y = ((y + 15.0) as i32 / 32) * 32;

            for tile in tiles.iter() {
                if tile.pos() == vec2(result_x as f32, result_y as f32) {
                    if let TileType::Wall(_) = tile.tile_type() {
                        return true;
                    }
                }
            }

            false
        };

        let mut new_pos = self.pos + velocity.normalize_or_zero() * self.speed * get_frame_time();

        if velocity.x <= 0.0 {
            if get_tile(new_pos.x - 15.0, self.pos.y) {
                new_pos.x = self.pos.x;
                velocity.x = 0.0;
            }
        } else {
            if get_tile(new_pos.x + 15.0, self.pos.y) {
                new_pos.x = self.pos.x;
                velocity.x = 0.0;
            }
        }

        if velocity.y <= 0.0 {
            if get_tile(self.pos.x, new_pos.y - 15.0) {
                new_pos.y = self.pos.y;
                velocity.y = 0.0;
            }
        } else {
            if get_tile(self.pos.x, new_pos.y + 15.0) {
                new_pos.y = self.pos.y;
                velocity.y = 0.0;
            }
        }

        self.pos = new_pos;
    }
}
