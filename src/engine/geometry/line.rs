use crate::engine::geometry;
use geometry::point::Point;

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Default for Line {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl Line {
    pub fn new(x_start: i32, y_start: i32, x_end: i32, y_end: i32) -> Self {
        Self { start: Point::pair_to_point((x_start, y_start)),
               end: Point::pair_to_point((x_end, y_end)) }
    }

    // Get the magnitude of the line
    pub fn magnitude(&self) -> f32 {
        geometry::p2p_distance(
            self.start.x,
            self.start.y,
            self.end.x,
            self.end.y
        )
    }
}
