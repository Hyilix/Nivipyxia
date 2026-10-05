use crate::engine::canvas::Canvas;
use crate::engine::geometry;
use geometry::circle::Circle;

// Draw a filled circle onto a canvas with a color
pub fn filled(canvas: &mut Canvas, circle: &Circle, color: u32) {
    let center = circle.center;

    let y_start: i32 = center.y - circle.radius as i32;
    let y_end: i32 = center.y + circle.radius as i32;
    let x_start: i32 = center.x - circle.radius as i32;
    let x_end: i32 = center.x + circle.radius as i32;

    for y in y_start..y_end + 1 {
        for x in x_start..x_end + 1 {
            let dist = geometry::p2p_distance(x, y, center.x, center.y);

            println!("distance from ({x} {y}) to ({} {}) is {dist}", center.x, center.y);
            if dist.round() <= circle.radius as f32 {
                canvas.draw_point((x, y), color);
            }
        }
    }
}

// Draw a circle outline onto a canvas with a color
pub fn outlined(canvas: &mut Canvas, circle: &Circle, color: u32) {
    let center = circle.center;

    let y_start: i32 = center.y - circle.radius as i32;
    let y_end: i32 = center.y + circle.radius as i32;
    let x_start: i32 = center.x - circle.radius as i32;
    let x_end: i32 = center.x + circle.radius as i32;

    for y in y_start..y_end + 1 {
        for x in x_start..x_end + 1 {
            let dist = geometry::p2p_distance(x, y, center.x, center.y);

            println!("distance from ({x} {y}) to ({} {}) is {dist}", center.x, center.y);
            if dist.round() == circle.radius as f32 {
                canvas.draw_point((x, y), color);
            }
        }
    }
}
