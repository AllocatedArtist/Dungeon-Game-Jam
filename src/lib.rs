use macroquad::{
    hash, input,
    prelude::*,
    ui::{self, root_ui},
};

#[macro_use]
extern crate serde_derive;

use std::io::Write;
use std::{fs::File, io::ErrorKind};

use serde::ser::{Serialize, SerializeStruct, Serializer};

use serde_json::Result as JsonResult;
//use serde_json::Value;

pub fn setup_window() -> Conf {
    Conf {
        window_title: String::from("Dungeon Game"),
        fullscreen: false,
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

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
            if let TileType::PlayerSpawn(_) = tile.tile_type {
                self.pos = tile.pos;
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
                    if let TileType::Wall(_) = tile.tile_type {
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

fn create_new_map(padding: (f32, f32), map_width: i32, map_height: i32) -> Vec<Tile> {
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

pub enum EditorMode {
    None,
    Paint,
}

pub struct TileMapEditor {
    sample_x: f32,
    show_edit_window: bool,
    is_collision_paint: bool,
    sample_y: f32,
    filename: String,
    h_slice: f32,
    v_slice: f32,
    tile_scale: f32,
    map_width_slider: f32,
    spawn_set: bool,
    map_height_slider: f32,
    tilemap_source: Texture2D,
    pub editor_camera: EditorCamera,
    map_size: (i32, i32),
    pub tiles: Vec<Tile>,
    padding: (f32, f32),
    can_paint: bool,
    editor_mode: EditorMode,
}

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
    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn reset(&mut self) {
        self.tile_type = TileType::Empty(0);
        self.source = Rect::new(0.0, 0.0, 0.0, 0.0);
    }
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

impl TileMapEditor {
    pub fn new(
        sample_x: f32,
        sample_y: f32,
        h_slice: f32,
        v_slice: f32,
        tile_scale: f32,
        map_size: (i32, i32),
        padding: (f32, f32),
    ) -> TileMapEditor {
        TileMapEditor {
            spawn_set: false,
            sample_x,
            sample_y,
            is_collision_paint: false,
            show_edit_window: true,
            filename: String::new(),
            map_width_slider: 10.0,
            map_height_slider: 10.0,
            h_slice,
            editor_camera: EditorCamera::new(),
            v_slice,
            map_size,
            tilemap_source: Texture2D::empty(),
            tiles: vec![],
            tile_scale,
            padding,
            editor_mode: EditorMode::None,
            can_paint: false,
        }
    }
    pub fn mouse_to_grid(&self) -> Vec2 {
        let mouse_pos = self
            .editor_camera
            .camera
            .screen_to_world(Vec2::new(mouse_position().0, mouse_position().1));

        let result_x = (mouse_pos.x as i32 / 32) * 32;
        let result_y = (mouse_pos.y as i32 / 32) * 32;

        Vec2::new(
            result_x as f32 * self.padding.0,
            result_y as f32 * self.padding.1,
        )
    }

    pub fn set_texture(&mut self, texture: Texture2D) {
        self.tilemap_source = texture;
    }

    pub fn current_rect(&self) -> Rect {
        Rect::new(
            self.sample_x * self.tile_scale,
            self.sample_y * self.tile_scale,
            self.tile_scale,
            self.tile_scale,
        )
    }

    pub fn can_edit(&self) -> bool {
        self.can_paint
    }

    pub fn edit_tiles(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let pos = self.mouse_to_grid();

            let grid = self.current_rect();
            for tile in self.tiles.iter_mut() {
                if tile.pos == pos {
                    tile.tile_type = if !self.is_collision_paint {
                        tile.source = grid;
                        TileType::Floor(1)
                    } else {
                        TileType::Wall(2)
                    };
                }
            }
        }

        if is_key_pressed(KeyCode::C) {
            self.is_collision_paint = !self.is_collision_paint;
        }

        if is_mouse_button_down(MouseButton::Right) {
            let pos = self.mouse_to_grid();

            for tile in self.tiles.iter_mut() {
                if tile.pos == pos {
                    tile.tile_type = if !self.is_collision_paint {
                        TileType::Empty(0)
                    } else {
                        TileType::Floor(1)
                    };
                }
            }
        }

        if is_key_pressed(KeyCode::P) && is_key_down(KeyCode::LeftControl) && !self.spawn_set {
            let pos = self.mouse_to_grid();

            for tile in self.tiles.iter_mut() {
                if tile.pos == pos {
                    self.spawn_set = true;
                    tile.tile_type = TileType::PlayerSpawn(3);
                }
            }
        }

        if is_key_pressed(KeyCode::R) && is_key_down(KeyCode::LeftControl) && self.spawn_set {
            for tile in self.tiles.iter_mut() {
                if let TileType::PlayerSpawn(_) = tile.tile_type {
                    self.spawn_set = false;
                    tile.tile_type = TileType::Floor(1);
                }
            }
        }
    }

    pub fn show_editors(&mut self) {
        match self.editor_mode {
            EditorMode::None => {
                self.show_edit_window = true;
                self.is_collision_paint = false;
                self.can_paint = false;
            }
            EditorMode::Paint => {
                if self.can_edit() {
                    self.edit_tiles();
                }

                if is_key_pressed(KeyCode::E) && is_key_down(KeyCode::LeftControl) {
                    self.can_paint = !self.can_paint;
                    if self.can_paint == false {
                        self.is_collision_paint = false;
                    }
                }

                if is_key_pressed(KeyCode::Q) && is_key_down(KeyCode::LeftControl) {
                    self.show_edit_window = !self.show_edit_window;
                }

                if self.show_edit_window {
                    self.serialization_editor();
                    self.draw();
                }

                if self.can_edit() {
                    draw_text("Edit Mode", 0.0, 20.0, 16.0, RED);
                    if self.is_collision_paint {
                        draw_text("Collision Paint", 0.0, 40.0, 16.0, YELLOW);
                    }
                }

                if is_key_pressed(KeyCode::F3) {
                    self.is_collision_paint = false;
                    self.can_paint = false;
                }
            }
        }
    }

    pub fn draw(&mut self) {
        ui::widgets::Window::new(ui::hash!(), vec2(0.0, 0.0), vec2(300.0, 300.0))
            .label("Editor")
            .titlebar(true)
            .ui(&mut *ui::root_ui(), |ui| {
                ui.tree_node(ui::hash!(), "Spritesheet", |ui| {
                    ui.texture(
                        self.tilemap_source,
                        self.tilemap_source.width(),
                        self.tilemap_source.height(),
                    );
                });

                ui.tree_node(hash!(), "Sprite Slice", |ui| {
                    ui.slider(hash!(), "Rows", 0.0..100.0, &mut self.h_slice);
                    ui.slider(hash!(), "Columns", 0.0..100.0, &mut self.v_slice);
                    self.h_slice = self.h_slice.round();
                    self.v_slice = self.v_slice.round();
                    ui.slider(
                        hash!(),
                        "X Pos",
                        0.0..self.h_slice - 1.0,
                        &mut self.sample_x,
                    );
                    ui.slider(
                        hash!(),
                        "Y Pos",
                        0.0..self.v_slice - 1.0,
                        &mut self.sample_y,
                    );

                    self.sample_x = self.sample_x.round();
                    self.sample_y = self.sample_y.round();
                });

                ui.tree_node(hash!(), "Map Options", |ui| {
                    ui.slider(hash!(), "Map Width", 1.0..50.0, &mut self.map_width_slider);
                    ui.slider(
                        hash!(),
                        "Map Height",
                        1.0..50.0,
                        &mut self.map_height_slider,
                    );
                    self.map_width_slider = self.map_width_slider.round();
                    self.map_height_slider = self.map_height_slider.round();

                    if ui.button(Vec2::new(0.0, 290.0), "Update Map (Resets all tiles)") {
                        self.spawn_set = false;
                        self.map_size =
                            (self.map_width_slider as i32, self.map_height_slider as i32);
                        self.tiles = create_new_map(self.padding, self.map_size.0, self.map_size.1);
                    }
                });

                ui.checkbox(hash!(), "Edit Mode", &mut self.can_paint);

                let selected_texture;

                let sample_rect = Rect::new(
                    self.sample_x * self.tile_scale,
                    self.sample_y * self.tile_scale,
                    self.tile_scale,
                    self.tile_scale,
                );

                if sub_image_valid(&self.tilemap_source, sample_rect) {
                    selected_texture = self
                        .tilemap_source
                        .get_texture_data()
                        .sub_image(sample_rect);
                } else {
                    ui.label(Vec2::new(0.0, 230.0), "Invalid texture sample.");
                    selected_texture = Image::empty();
                }
                ui.texture(
                    Texture2D::from_image(&selected_texture),
                    self.tile_scale,
                    self.tile_scale,
                );
            });
    }

    pub fn switch_mode(&mut self, mode: EditorMode) {
        self.editor_mode = mode;
    }

    pub fn tilemap_source(&self) -> Texture2D {
        self.tilemap_source
    }

    pub fn serialization_editor(&mut self) {
        ui::widgets::Window::new(hash!(), vec2(100.0, 0.0), vec2(300.0, 300.0))
            .label("Save/Load")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.input_text(hash!(), "Filename", &mut self.filename);
                if ui.button(Vec2::new(25.0, 50.0), "Save") && !self.filename.is_empty() {
                    match save(&self.tiles, &self.filename) {
                        Ok(_) => eprintln!("{} saved successfully!", self.filename),
                        Err(_) => eprintln!("{} not saved.", self.filename),
                    }
                }
                if ui.button(Vec2::new(70.0, 50.0), "Load") && !self.filename.is_empty() {
                    self.tiles = load(&self.filename).unwrap();

                    for tile in self.tiles.iter() {
                        if let TileType::PlayerSpawn(_) = tile.tile_type {
                            self.spawn_set = false;
                        }
                    }
                }
            });
    }
}

pub struct EditorCamera {
    camera: Camera2D,
    speed: f32,
}

impl EditorCamera {
    pub fn new() -> EditorCamera {
        let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 600.0, 600.0));

        EditorCamera { camera, speed: 5.0 }
    }

    pub fn update_camera(&mut self) {
        let mut move_dir = Vec2::ZERO;

        if is_key_down(KeyCode::Right) {
            move_dir.x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            move_dir.y += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            move_dir.y -= 1.0;
        }

        self.camera.target += move_dir.normalize_or_zero() * self.speed * 50.0 * get_frame_time();
        self.camera.target = self.camera.target.round();

        /*

        zooming in bad

        if is_key_pressed(KeyCode::Z) {
            self.camera.zoom.x += 0.001;

            self.camera.zoom.y -= 0.001;
        }
        if is_key_pressed(KeyCode::X) && self.camera.zoom.y < -0.001 {
            self.camera.zoom.x -= 0.001;
            self.camera.zoom.y += 0.001;
        }
        */

        set_camera(&self.camera);
    }
}

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
            x: self.pos.x,
            y: self.pos.y,
        };

        let source = SerRec {
            x: self.source.x,
            y: self.source.y,
            w: self.source.w,
            h: self.source.h,
        };

        let mut tile = serializer.serialize_struct("Tile", 3)?;
        tile.serialize_field("pos", &pos)?;
        tile.serialize_field("source", &source)?;
        tile.serialize_field("tile_type", &self.tile_type)?;
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

pub fn draw_map(tiles: &mut Vec<Tile>, tilemap: Texture2D, debug_collider: bool) {
    for tile in tiles.iter() {
        match tile.tile_type {
            TileType::Empty(_) => {
                draw_rectangle_lines(tile.pos.x, tile.pos.y, 32.0, 32.0, 1.0, RED);
            }
            TileType::Floor(_) => {
                draw_texture_ex(
                    tilemap,
                    tile.pos.x,
                    tile.pos.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Option::Some(Vec2::new(32.0, 32.0)),
                        source: Option::Some(Rect::new(
                            tile.source.x,
                            tile.source.y,
                            tile.source.w,
                            tile.source.h,
                        )),
                        ..Default::default()
                    },
                );
            }
            TileType::Wall(_) => {
                if debug_collider {
                    draw_rectangle_lines(tile.pos.x, tile.pos.y, 32.0, 32.0, 1.0, BLUE);
                } else {
                    draw_texture_ex(
                        tilemap,
                        tile.pos.x,
                        tile.pos.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Option::Some(Vec2::new(32.0, 32.0)),
                            source: Option::Some(Rect::new(
                                tile.source.x,
                                tile.source.y,
                                tile.source.w,
                                tile.source.h,
                            )),
                            ..Default::default()
                        },
                    );
                }
            }
            TileType::PlayerSpawn(_) => {
                if debug_collider {
                    draw_rectangle(tile.pos.x, tile.pos.y, 32.0, 32.0, BLACK);
                } else {
                    draw_texture_ex(
                        tilemap,
                        tile.pos.x,
                        tile.pos.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Option::Some(Vec2::new(32.0, 32.0)),
                            source: Option::Some(Rect::new(
                                tile.source.x,
                                tile.source.y,
                                tile.source.w,
                                tile.source.h,
                            )),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
}
