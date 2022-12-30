use macroquad::prelude::*;

use std::io::Write;
use std::{fs::File, io::ErrorKind};

use serde::ser::{Serialize, SerializeStruct, Serializer};

use serde_json::Result as JsonResult;

use crate::tile::tile::*;

impl Serialize for TileType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TileType::Empty(num) => {
                serializer.serialize_newtype_variant("TileType", 0, "Empty", &num)
            }
            TileType::Floor(num) => {
                serializer.serialize_newtype_variant("TileType", 1, "Floor", &num)
            }
            TileType::Wall(num) => {
                serializer.serialize_newtype_variant("TileType", 2, "Wall", &num)
            }
            TileType::PlayerSpawn(num) => {
                serializer.serialize_newtype_variant("TileType", 3, "PlayerSpawn", &num)
            }
        }
    }
}

#[derive(Deserialize, Debug)]
struct SerVec2 {
    x: f32,
    y: f32,
}

#[derive(Deserialize, Debug)]
struct SerRec {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Serialize for SerVec2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut vec_2 = serializer.serialize_struct("SerVec2", 2)?;
        vec_2.serialize_field("x", &self.x)?;
        vec_2.serialize_field("y", &self.y)?;
        vec_2.end()
    }
}

impl Serialize for SerRec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut rec = serializer.serialize_struct("SerRec", 4)?;
        rec.serialize_field("x", &self.x)?;
        rec.serialize_field("y", &self.y)?;
        rec.serialize_field("w", &self.w)?;
        rec.serialize_field("h", &self.h)?;
        rec.end()
    }
}

impl Serialize for Tile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let pos = SerVec2 {
            x: self.pos().x,
            y: self.pos().y,
        };

        let source = SerRec {
            x: self.source().x,
            y: self.source().y,
            w: self.source().w,
            h: self.source().h,
        };

        let mut tile = serializer.serialize_struct("Tile", 3)?;
        tile.serialize_field("pos", &pos)?;
        tile.serialize_field("source", &source)?;
        tile.serialize_field("tile_type", &self.tile_type())?;
        tile.end()
    }
}

pub fn save(tiles: &Vec<Tile>, path: &str) -> JsonResult<()> {
    let j = serde_json::to_string_pretty(tiles)?;

    let extended_path = format!("{}.json", path);

    let mut output = File::create(extended_path).unwrap();
    write!(output, "{}", j).unwrap();

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct LoadedTile {
    pos: SerVec2,
    source: SerRec,
    tile_type: TileType,
}

pub fn load(path: &str) -> JsonResult<Vec<Tile>> {
    let extended_path = format!("res/levels/{}.json", path);

    let contents = match std::fs::read_to_string(extended_path) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                eprintln!("{} not found", path);
                return Ok(Vec::new());
            }

            _ => {
                eprintln!("{}", err.to_string());
                return Ok(Vec::new());
            }
        },
    };

    let tiles: Vec<LoadedTile> = match serde_json::from_str(&contents) {
        Ok(data) => data,
        Err(err) => {
            if err.is_data() {
                eprintln!("{} data not valid", path);
            } else if err.is_io() {
                eprintln!("Could not read {}", path);
            } else if err.is_syntax() {
                eprintln!("{} syntax not valid", path);
            }

            eprintln!("{}", err.to_string());

            Vec::new()
        }
    };

    let mut converted_tiles = Vec::new();

    for tile in tiles.iter() {
        converted_tiles.push(Tile::new(
            vec2(tile.pos.x, tile.pos.y),
            Rect::new(tile.source.x, tile.source.y, tile.source.w, tile.source.h),
            tile.tile_type,
        ));
    }

    Ok(converted_tiles)
}
