use macroquad::prelude::*;

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum TileType {
    Empty(i32),
    Floor(i32),
    Wall(i32),
    PlayerSpawn(i32),
}

pub struct Tile {
    source: Rect,
    pos: Vec2,
    tile_type: TileType,
}

impl Tile {
    pub fn new(pos: Vec2, source: Rect, tile_type: TileType) -> Tile {
        Tile {
            source,
            pos,
            tile_type,
        }
    }

    pub fn empty(pos: Vec2) -> Tile {
        Tile {
            pos,
            source: Rect::new(0.0, 0.0, 0.0, 0.0),
            tile_type: TileType::Empty(0),
        }
    }

    pub fn source(&self) -> Rect {
        self.source
    }

    pub fn set_source(&mut self, rect: Rect) {
        self.source = rect;
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn reset(&mut self) {
        self.tile_type = TileType::Empty(0);
        self.source = Rect::new(0.0, 0.0, 0.0, 0.0);
    }

    pub fn tile_type(&self) -> TileType {
        self.tile_type
    }

    pub fn set_type(&mut self, tile_type: TileType) {
        self.tile_type = tile_type;
    }
}

pub fn get_tile(x: f32, y: f32, tiles: &Vec<Tile>) -> bool {
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
}

pub fn calculate_tile_pos(x: f32, y: f32) -> Vec2 {
    let result_x = (x as i32 / 32) * 32;
    let result_y = (y as i32 / 32) * 32;

    vec2(result_x as f32, result_y as f32)
}
