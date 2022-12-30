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
