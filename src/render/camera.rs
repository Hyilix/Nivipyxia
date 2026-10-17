// The camera of the application

use nivis::canvas;
use nivis::geometry::rectangle::{self, Rectangle};
use nivis::geometry::point::Point;
use nivis::draw;
use nivis::color::Color;

use std::cmp;

pub struct Camera {
    presentation_area: rectangle::Rectangle,
    area: rectangle::Rectangle,
    zoom: f32,
    canvas: canvas::Canvas,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(0, 0, 1, 1, 1f32)
    }
}

// Boiler plate stuff
impl Camera {
    pub fn new(px: i32, py: i32, sx: u32, sy: u32, zoom: f32) -> Self {
        Self {
            presentation_area: rectangle::Rectangle::new(0, 0, sx, sy),
            area: rectangle::Rectangle::new(px, py, sx, sy),
            zoom,
            canvas: canvas::Canvas::default(),
        }
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn add_zoom(&mut self, val: f32) {
        self.zoom += val;
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.area.get_x(), self.area.get_y())
    }

    pub fn get_corner(&self) -> Point {
        self.area.corner
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.area.width, self.area.height)
    }

    pub fn get_x(&self) -> i32 {
        self.area.get_x()
    }

    pub fn get_y(&self) -> i32 {
        self.area.get_y()
    }

    pub fn get_width(&self) -> u32 {
        self.area.width
    }

    pub fn get_height(&self) -> u32 {
        self.area.height
    }

    pub fn set_width(&mut self, new: u32) {
        self.area.width = new;
    }

    pub fn set_height(&mut self, new: u32) {
        self.area.height = new;
    }

    pub fn set_presentation_width(&mut self, new_w: u32) {
        self.presentation_area.width = new_w;
    }

    pub fn set_presentation_height(&mut self, new_h: u32) {
        self.presentation_area.height = new_h;
    }

    pub fn set_x(&mut self, new: i32) {
        self.area.corner.x = new;
    }

    pub fn set_y(&mut self, new: i32) {
        self.area.corner.y = new;
    }

    pub fn get_canvas(&self) -> &canvas::Canvas {
        &self.canvas
    }

    pub fn get_canvas_mut(&mut self) -> &mut canvas::Canvas {
        &mut self.canvas
    }
}

// Camera methods
impl Camera {
    pub fn get_camera_drawing_size(&self, screen_size: (u32, u32)) -> (u32, u32) {
        let mut result: (u32, u32) = (
            (self.presentation_area.get_width() as f32 / self.zoom).ceil() as u32,
            (self.presentation_area.get_height() as f32 / self.zoom).ceil() as u32
        );

        if result.0 >= self.canvas.get_width() {
            result.0 = self.canvas.get_width();
        }

        if result.1 >= self.canvas.get_height() {
            result.1 = self.canvas.get_height();
        }

        result.0 = (result.0 as f32 * self.zoom) as u32;
        result.1 = (result.1 as f32 * self.zoom) as u32;

        result
    }

    pub fn present_to_screen(
        &mut self,
        screen: &mut canvas::Canvas,
        screen_size: (u32, u32),
    ) {
        // Corner position of the camera centered on the center of screen
        let pos_cam : (i32, i32) = (
            (screen_size.0 / 2 - self.presentation_area.get_width() / 2) as i32,
            (screen_size.1 / 2 - self.presentation_area.get_height() / 2) as i32,
        );

        // self.area.scale_from_center(self.zoom);
        self.area.width = (self.presentation_area.get_width() as f32 / self.zoom).ceil() as u32;
        self.area.height = (self.presentation_area.get_height() as f32 / self.zoom).ceil() as u32;

        // let mut canvas_zone = self.canvas.get_zone(&self.area);
        // canvas_zone.scale_by(self.zoom);
        // canvas_zone.scale_to(
        //     self.presentation_area.get_width(),
        //     self.presentation_area.get_height()
        // );

        let zone_w = (self.area.width as f32 * self.zoom).min(self.canvas.get_width() as f32 * self.zoom);
        let zone_h = (self.area.height as f32 * self.zoom).min(self.canvas.get_height() as f32 * self.zoom);

        let pos_canvas = (
            (screen_size.0 / 2 - zone_w as u32 / 2) as i32,
            (screen_size.1 / 2 - zone_h as u32 / 2) as i32
        );

        // Corner position of the canvas centered on the center of screen
        // let pos_canvas : (i32, i32) = (
        //     (screen_size.0 / 2 - canvas_zone.get_width() / 2) as i32,
        //     (screen_size.1 / 2 - canvas_zone.get_height() / 2) as i32,
        // );

        let pos: (i32, i32) = (
            cmp::max(pos_cam.0, pos_canvas.0),
            cmp::max(pos_cam.1, pos_canvas.1)
        );

        // screen.blit(
        //     &canvas_zone,
        //     Point::pair_to_point(pos)
        // );

        println!("area {:?} at pos {:?}", self.area, pos);
        // println!("screen size: {:?}", (screen.get_width(), screen.get_height()));

        screen.blit_area_zoomed(&self.canvas, &self.area, Point::pair_to_point(pos), self.zoom);

        let presentation_rect: rectangle::Rectangle = 
            rectangle::Rectangle::new(pos_cam.0 - 1, pos_cam.1 - 1, self.presentation_area.get_width() + 1, self.presentation_area.get_height() + 1);

        draw::rectangle::outlined(screen, &presentation_rect, Color::new(0xFF, 0, 0, 0));

        let outline_rect: rectangle::Rectangle = 
            rectangle::Rectangle::new(pos.0 - 1, pos.1 - 1, zone_w as u32 + 1, zone_h as u32 + 1);

        draw::rectangle::outlined(screen, &outline_rect, Color::new(0, 0xFF, 0, 0));
    }

    // Move the camera position with the specified vector
    pub fn move_with(&mut self, vec: Point) {
        let mut new_pos: Point = Point::pair_to_point(
            (self.area.corner.x - vec.x,
             self.area.corner.y - vec.y)
        );

        // Clamp
        if new_pos.x < 0 {
            new_pos.x = 0;
        }

        if new_pos.x + self.area.get_width() as i32 >= self.canvas.get_width() as i32 {
            new_pos.x -= new_pos.x + self.area.get_width() as i32 - self.canvas.get_width() as i32;
        }

        if new_pos.y < 0 {
            new_pos.y = 0;
        }

        if new_pos.y + self.area.get_height() as i32 >= self.canvas.get_height() as i32 {
            new_pos.y -= new_pos.y + self.area.get_height() as i32 - self.canvas.get_height() as i32;
        }

        self.area.corner = new_pos;
    }
}
