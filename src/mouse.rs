use winit::dpi;

use crate::render::camera::Camera;

use nivis::geometry::{rectangle::Rectangle, point::Point};

pub struct Mouse {
    current_pos: dpi::PhysicalPosition<f64>,
    last_pos: dpi::PhysicalPosition<f64>,
    pressed: bool,
}

impl Default for Mouse {
    fn default() -> Self {
        Self::new()
    }
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            current_pos: dpi::PhysicalPosition::new(0f64, 0f64),
            last_pos: dpi::PhysicalPosition::new(0f64, 0f64),
            pressed: false
        }
    }
}

impl Mouse {
    // Update the position of the mouse
    pub fn update_position(&mut self, pos: dpi::PhysicalPosition<f64>) {
        self.last_pos = self.current_pos;
        self.current_pos = pos;
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.pressed = pressed;
    }

    pub fn toggle_pressed(&mut self) {
        self.pressed = !self.pressed;
    }
}

impl Mouse {
    pub fn to_camera_position(
        &mut self,
        camera: &mut Camera,
        position: (i32, i32),
        win_size: (u32, u32),
    ) -> Option<Point> {
        // Get information about the displayed canvas
        let can_size = camera.get_camera_drawing_size(win_size);
        let can_rect = Rectangle::new(
            (win_size.0 / 2 - can_size.0 / 2) as i32,
            (win_size.1 / 2 - can_size.1 / 2) as i32,
            can_size.0,
            can_size.1
        );

        // Mouse not in canvas
        if !can_rect.has_point(Point::pair_to_point(position)) {
            return None;
        }

        let draw_pos: Point = Point::new(
            (position.0 as i32 - can_rect.corner.x) / camera.get_zoom() as i32,
            (position.1 as i32 - can_rect.corner.y) / camera.get_zoom() as i32
        );

        Some(draw_pos)
    }
}
