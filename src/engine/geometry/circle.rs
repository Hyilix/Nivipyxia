use crate::engine::geometry::point::Point;

pub struct Circle {
    pub center: Point,
    pub radius: u32,
}

impl Default for Circle {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl Circle {
    pub fn new(x: i32, y: i32, radius: u32) -> Self {
        Self { center: Point::pair_to_point((x, y)), radius}
    }
}
