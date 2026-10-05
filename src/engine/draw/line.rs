use crate::engine::canvas::Canvas;
use crate::engine::geometry::line::Line;

// Draw a 1 width line of a color on a canvas
pub fn simple(canvas: &mut Canvas, line: &Line, color: u32) {
    let y_step: f32 = (line.end.y - line.start.y) as f32 / (line.end.x - line.start.x) as f32;
    let mut y: f32 = line.start.y as f32;

    for x in line.start.x..line.start.x {
        canvas.draw_point((x, y.round() as i32), color);
        y += y_step;
    }
}

// TODO: Implement AA line drawing
pub fn aa() {

}
