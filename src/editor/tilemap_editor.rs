use crate::editor::editor_camera::*;
use crate::serialization::*;
use crate::tile::*;
use crate::utility::*;
use macroquad::{hash, prelude::*, ui, ui::root_ui};

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
            .camera()
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
                if tile.pos() == pos {
                    let tile_type = if !self.is_collision_paint {
                        tile.set_source(grid);
                        TileType::Floor(1)
                    } else {
                        TileType::Wall(2)
                    };

                    tile.set_type(tile_type);
                }
            }
        }

        if is_key_pressed(KeyCode::C) {
            self.is_collision_paint = !self.is_collision_paint;
        }

        if is_mouse_button_down(MouseButton::Right) {
            let pos = self.mouse_to_grid();

            for tile in self.tiles.iter_mut() {
                if tile.pos() == pos {
                    let tile_type = if !self.is_collision_paint {
                        TileType::Empty(0)
                    } else {
                        TileType::Floor(1)
                    };

                    tile.set_type(tile_type);
                }
            }
        }

        if is_key_pressed(KeyCode::P) && is_key_down(KeyCode::LeftControl) && !self.spawn_set {
            let pos = self.mouse_to_grid();

            for tile in self.tiles.iter_mut() {
                if tile.pos() == pos {
                    self.spawn_set = true;
                    tile.set_type(TileType::PlayerSpawn(3));
                }
            }
        }

        if is_key_pressed(KeyCode::R) && is_key_down(KeyCode::LeftControl) && self.spawn_set {
            for tile in self.tiles.iter_mut() {
                if let TileType::PlayerSpawn(_) = tile.tile_type() {
                    self.spawn_set = false;
                    tile.set_type(TileType::Floor(1));
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

                if sub_image_valid(&self.tilemap_source(), sample_rect) {
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
                        if let TileType::PlayerSpawn(_) = tile.tile_type() {
                            self.spawn_set = false;
                        }
                    }
                }
            });
    }
}
