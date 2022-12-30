use macroquad::prelude::*;

pub struct EditorCamera {
    camera: Camera2D,
    speed: f32,
}

impl EditorCamera {
    pub fn new() -> EditorCamera {
        let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 600.0, 600.0));

        EditorCamera { camera, speed: 5.0 }
    }

    pub fn camera(&self) -> Camera2D {
        self.camera
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
