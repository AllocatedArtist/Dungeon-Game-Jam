use crate::player::*;
use crate::tile::*;

use macroquad::prelude::*;

use std::collections::VecDeque;

#[derive(Clone)]
pub struct Enemy {
    attack_timer: f32,
    invulnerable: bool,
    health: i32,
    idle: bool,
    is_attacking: bool,
    attack_anim_start: bool,
    health_timer: f32,
    speed: f32,
    attack_spot: Vec2,
    player_spotted: bool,
    move_pos: Vec2,
    prev_goal: Vec2,
    path: VecDeque<Vec2>,
    sword_pos: Vec2,
    pos: Vec2,
}

impl Enemy {
    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub fn path(&self) -> VecDeque<Vec2> {
        self.path.clone()
    }

    pub fn player_spotted(&self) -> bool {
        self.player_spotted
    }

    pub fn set_player_spotted(&mut self, spotted: bool) {
        self.player_spotted = spotted
    }

    pub fn new(pos: Vec2) -> Enemy {
        Enemy {
            health_timer: 0.0,
            health: 3,
            invulnerable: false,
            idle: true,
            attack_timer: 2.0,
            attack_anim_start: false,
            is_attacking: false,
            attack_spot: Vec2::ZERO,
            sword_pos: Vec2::ZERO,
            speed: rand::gen_range(90.0, 105.0),
            player_spotted: false,
            move_pos: Vec2::ZERO,
            pos,
            prev_goal: Vec2::ZERO,
            path: VecDeque::new(),
        }
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn draw(&mut self, texture: Texture2D) {
        if !self.invulnerable {
            draw_texture_ex(
                texture,
                self.pos.x,
                self.pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(32.0, 32.0)),
                    ..Default::default()
                },
            );
        } else {
            self.health_timer += get_frame_time();
            if self.health_timer > 1.5 {
                self.health_timer = 0.0;
                self.invulnerable = false;
            }

            draw_texture_ex(
                texture,
                self.pos.x,
                self.pos.y,
                RED,
                DrawTextureParams {
                    dest_size: Some(vec2(32.0, 32.0)),
                    ..Default::default()
                },
            );
        }
    }

    pub fn damage_player(&self, player: &mut Player) {
        if self.sword_pos.distance(player.pos()) < 19.0 && !self.idle {
            player.take_damage();
        }
    }

    pub fn take_damage(&mut self) {
        if !self.invulnerable {
            self.health -= 1;
            self.health = self.health.max(0);

            self.idle = true;
            self.invulnerable = true;
        }
    }

    pub fn draw_weapon(&mut self, texture: Texture2D, player_pos: Vec2) {
        if self.player_spotted {
            let dir = (player_pos - self.pos).normalize_or_zero();

            let angle = player_pos.angle_between(self.sword_pos);

            let flip_y = player_pos.y > self.pos.y;

            if self.pos.distance(player_pos) < 50.0 && !self.idle {
                if !self.is_attacking {
                    self.is_attacking = true;
                    self.attack_spot = self.sword_pos + dir * 40.0;
                }

                if self.sword_pos.distance(self.attack_spot) > 5.0
                    && self.attack_anim_start == false
                    && self.is_attacking
                {
                    self.sword_pos = self.sword_pos.lerp(self.attack_spot, 0.06);
                } else {
                    self.attack_anim_start = true;
                    self.sword_pos = self.sword_pos.lerp(self.pos + dir * 20.0, 0.1);
                    if self.sword_pos.distance(self.pos + dir * 20.0) > 5.0 {
                        self.attack_anim_start = false;
                        self.is_attacking = false;
                        self.idle = true;
                    }
                }

                if self.sword_pos.distance(self.pos) > 50.0 {
                    self.is_attacking = false;
                    self.attack_anim_start = false;
                    self.sword_pos = self.pos + dir * 20.0;
                    self.idle = true;
                }
            } else {
                if self.idle {
                    self.attack_timer += get_frame_time();
                    if self.attack_timer > 2.5 {
                        self.attack_timer = 0.0;
                        self.idle = false;
                    }
                }
                self.is_attacking = false;
                self.sword_pos = self.pos + dir * 20.0;
            }

            if !self.idle {
                draw_texture_ex(
                    texture,
                    self.sword_pos.x,
                    self.sword_pos.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(32.0, 32.0)),
                        rotation: angle,
                        pivot: Some(self.sword_pos),
                        flip_y,
                        ..Default::default()
                    },
                );
            } else {
                draw_texture_ex(
                    texture,
                    self.sword_pos.x,
                    self.sword_pos.y,
                    RED,
                    DrawTextureParams {
                        dest_size: Some(vec2(32.0, 32.0)),
                        rotation: angle,
                        pivot: Some(self.sword_pos),
                        flip_y,
                        ..Default::default()
                    },
                );
            }
        } else {
            draw_texture_ex(
                texture,
                self.pos.x - 12.0,
                self.pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(32.0, 32.0)),
                    ..Default::default()
                },
            );
        }
    }

    pub fn displace(&mut self, other: &Enemy) {
        if other.pos.distance(self.pos) < 20.0 {
            let dir = (other.pos() - self.pos()).normalize_or_zero();
            self.pos += -dir * 10.0 * get_frame_time();
        }
    }

    pub fn move_to(&mut self, new_pos: Vec2, tiles: &Vec<Tile>) {
        if self.pos.distance(new_pos) > 100.0 {
            self.prev_goal = new_pos;

            self.path = breadth_first_search(
                calculate_tile_pos(self.pos.x, self.pos.y),
                calculate_tile_pos(new_pos.x, new_pos.y),
                tiles,
            );

            self.move_pos = self.path.pop_front().unwrap_or(new_pos);
            self.move_pos = self.move_pos.round();
        } else {
            self.path.clear();
            self.move_pos = new_pos;
        }

        if self.pos.distance(new_pos) > 50.0 {
            if self.pos.distance(self.move_pos) < 2.0 {
                self.move_pos = self.path.pop_front().unwrap_or(new_pos);
                self.move_pos = self.move_pos.round();
            }

            let mut velocity = (self.move_pos - self.pos).normalize_or_zero();

            let mut new_pos = self.pos + velocity * self.speed * get_frame_time();

            if velocity.x <= 0.0 {
                if get_tile(new_pos.x - 15.0, self.pos.y, tiles) {
                    new_pos.x = self.pos.x;
                    velocity.x = 0.0;
                }
            } else {
                if get_tile(new_pos.x + 15.0, self.pos.y, tiles) {
                    new_pos.x = self.pos.x;
                    velocity.x = 0.0;
                }
            }

            if velocity.y <= 0.0 {
                if get_tile(self.pos.x, new_pos.y - 15.0, tiles) {
                    new_pos.y = self.pos.y;
                    velocity.y = 0.0;
                }
            } else {
                if get_tile(self.pos.x, new_pos.y + 15.0, tiles) {
                    new_pos.y = self.pos.y;
                    velocity.y = 0.0;
                }
            }

            self.pos = new_pos;
        }
    }
}

pub struct Path {
    a: Option<Vec2>,
    b: Vec2,
}

impl Path {
    pub fn new(a: Option<Vec2>, b: Vec2) -> Path {
        Path { a, b }
    }
}

pub fn breadth_first_search(start_pos: Vec2, goal: Vec2, tiles: &Vec<Tile>) -> VecDeque<Vec2> {
    if get_tile(goal.x, goal.y, tiles) {
        return VecDeque::new();
    }

    let mut frontier: VecDeque<Vec2> = VecDeque::new();
    frontier.push_front(start_pos);

    let mut came_from: Vec<Path> = Vec::new();
    came_from.push(Path::new(None, start_pos));

    while !frontier.is_empty() {
        let current = frontier.pop_back().unwrap_or(start_pos);

        if current == goal {
            break;
        }

        for next in get_neighbors(current, tiles).iter() {
            if !is_found_in_vec(&came_from, *next) {
                frontier.push_front(*next);
                came_from.push(Path::new(Some(current), *next));
            }
        }
    }

    let mut current = goal;
    let mut path: VecDeque<Vec2> = VecDeque::new();

    while current != start_pos {
        path.push_front(current);
        if let Some(a) = get_path(&came_from, current) {
            current = a;
        }
    }

    path
}

pub fn get_path(vector: &Vec<Path>, b: Vec2) -> Option<Vec2> {
    for v in vector.iter() {
        if v.b == b {
            return v.a;
        }
    }

    None
}

pub fn is_found_in_vec(vector: &Vec<Path>, element: Vec2) -> bool {
    for v in vector.iter() {
        if v.b == element {
            return true;
        }
    }

    false
}

pub fn get_neighbors(tile: Vec2, tiles: &Vec<Tile>) -> Vec<Vec2> {
    let up = get_tile(tile.x, tile.y - 32.0, tiles);
    let down = get_tile(tile.x, tile.y + 32.0, tiles);
    let right = get_tile(tile.x + 32.0, tile.y, tiles);
    let left = get_tile(tile.x - 32.0, tile.y, tiles);

    let mut neighbors = Vec::new();

    if !up {
        neighbors.push(calculate_tile_pos(tile.x, tile.y - 32.0));
    }
    if !down {
        neighbors.push(calculate_tile_pos(tile.x, tile.y + 32.0));
    }
    if !right {
        neighbors.push(calculate_tile_pos(tile.x + 32.0, tile.y));
    }
    if !left {
        neighbors.push(calculate_tile_pos(tile.x - 32.0, tile.y));
    }

    neighbors
}
