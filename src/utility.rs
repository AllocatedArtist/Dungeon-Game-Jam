use crate::tile::tile::*;
use macroquad::prelude::*;

pub async fn create_texture(path: &str) -> Result<Texture2D, String> {
    let mut error_message = String::new();

    let texture = match load_texture(path).await {
        Ok(tx) => tx,
        Err(err) => {
            match err.kind {
                miniquad::fs::Error::IOError(msg) => {
                    error_message = format!("Error loading file: {}, error message: {}", path, msg)
                }
                miniquad::fs::Error::DownloadFailed => {
                    error_message = format!("Error loading file: {}, could not download.", path)
                }
                _ => (),
            };

            return Err(error_message);
        }
    };

    Ok(texture)
}

pub fn create_new_map(padding: (f32, f32), map_width: i32, map_height: i32) -> Vec<Tile> {
    let mut tiles = Vec::new();

    let x_padding = padding.0;
    let y_padding = padding.1;

    for y in 0..map_height {
        for x in 0..map_width {
            tiles.push(Tile::empty(Vec2::new(
                x as f32 * x_padding * 32.0,
                y as f32 * y_padding * 32.0,
            )));
        }
    }

    tiles
}

pub fn sub_image_valid(texture: &Texture2D, rect: Rect) -> bool {
    let width = rect.w as usize;
    let height = rect.h as usize;

    let texture_width = texture.width();
    let length = texture.get_texture_data().bytes.len();

    let x = rect.x as usize;
    let y = rect.y as usize;
    for y in y..y + height {
        for x in x..x + width {
            let n_1 = y * texture_width as usize * 4 + x * 4 + 0;
            let n_2 = y * texture_width as usize * 4 + x * 4 + 1;
            let n_3 = y * texture_width as usize * 4 + x * 4 + 2;
            let n_4 = y * texture_width as usize * 4 + x * 4 + 3;

            if n_1 >= length || n_2 >= length || n_3 >= length || n_4 >= length {
                return false;
            }
        }
    }

    true
}

pub fn draw_map(tiles: &mut Vec<Tile>, tilemap: Texture2D, debug_collider: bool) {
    for tile in tiles.iter() {
        match tile.tile_type() {
            TileType::Empty(_) => {
                draw_rectangle_lines(tile.pos().x, tile.pos().y, 32.0, 32.0, 1.0, RED);
            }
            TileType::Floor(_) => {
                draw_texture_ex(
                    tilemap,
                    tile.pos().x,
                    tile.pos().y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Option::Some(Vec2::new(32.0, 32.0)),
                        source: Option::Some(Rect::new(
                            tile.source().x,
                            tile.source().y,
                            tile.source().w,
                            tile.source().h,
                        )),
                        ..Default::default()
                    },
                );
            }
            TileType::Wall(_) => {
                if debug_collider {
                    draw_rectangle_lines(tile.pos().x, tile.pos().y, 32.0, 32.0, 1.0, BLUE);
                } else {
                    draw_texture_ex(
                        tilemap,
                        tile.pos().x,
                        tile.pos().y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Option::Some(Vec2::new(32.0, 32.0)),
                            source: Option::Some(Rect::new(
                                tile.source().x,
                                tile.source().y,
                                tile.source().w,
                                tile.source().h,
                            )),
                            ..Default::default()
                        },
                    );
                }
            }
            TileType::PlayerSpawn(_) => {
                if debug_collider {
                    draw_rectangle(tile.pos().x, tile.pos().y, 32.0, 32.0, BLACK);
                } else {
                    draw_texture_ex(
                        tilemap,
                        tile.pos().x,
                        tile.pos().y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Option::Some(Vec2::new(32.0, 32.0)),
                            source: Option::Some(Rect::new(
                                tile.source().x,
                                tile.source().y,
                                tile.source().w,
                                tile.source().h,
                            )),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
}
