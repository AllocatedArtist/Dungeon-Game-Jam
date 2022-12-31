use crate::tile::*;
use macroquad::prelude::*;

use std::collections::VecDeque;

pub struct Enemy {
    move_pos: Vec2,
    prev_goal: Vec2,
    path: VecDeque<Vec2>,
    pos: Vec2,
}

impl Enemy {
    pub fn path(&self) -> VecDeque<Vec2> {
        self.path.clone()
    }

    pub fn new(pos: Vec2) -> Enemy {
        Enemy {
            move_pos: Vec2::ZERO,
            pos,
            prev_goal: Vec2::ZERO,
            path: VecDeque::new(),
        }
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn draw(&self, texture: Texture2D) {
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
    }

    pub fn move_to(&mut self, new_pos: Vec2, tiles: &Vec<Tile>) {
        self.prev_goal = new_pos;
        self.path = breadth_first_search(
            calculate_tile_pos(self.pos.x, self.pos.y),
            calculate_tile_pos(new_pos.x, new_pos.y),
            tiles,
        );
        self.move_pos = self.path.pop_front().unwrap_or(new_pos);

        if self.pos.distance(new_pos) > 50.0 {
            if self.pos.distance(self.move_pos) < 2.0 {
                self.move_pos = self.path.pop_front().unwrap_or(Vec2::ZERO);
            }

            let mut velocity = (self.move_pos - self.pos).normalize_or_zero();

            const SPEED: f32 = 80.0;

            let mut new_pos = self.pos + velocity * SPEED * get_frame_time();

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
