use crate::engine::geometry::point::Point;

pub struct Rectangle {
    pub corner: Point,
    pub width: u32,
    pub height: u32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { corner: Point::pair_to_point((x, y)), width, height}
    }

    // Get the rectangle area
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // Get the rectangle perimeter
    pub fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }

    pub fn get_x(&self) -> i32 {
        self.corner.x
    }

    pub fn get_y(&self) -> i32 {
        self.corner.y
    }
}
