use std::rc::Rc;
use std::num::NonZeroU32;

use crate::render::camera::Camera;
use nivis::canvas::Canvas;
use nivis::draw;
use nivis::geometry::rectangle;
use nivis::color::Color;

use winit::window::Window;
use softbuffer::Surface;

pub fn redraw_request(
    canvas: &mut Canvas,
    camera: &mut Camera,
    surface: &mut Option<Surface<Rc<dyn Window>, Rc<dyn Window>>>,
    window: &Option<Rc<dyn Window>>,
) {
    let (width, height) = {
        let size = window.as_ref().unwrap().surface_size();
        (size.width, size.height)
    };

    let max_dim = (width >> 1).max(height >> 1);

    let rescale_time = std::time::Instant::now();

    camera.set_presentation_width(max_dim);
    camera.set_presentation_height(max_dim);
    canvas.resize_pixels(width, height, Color::new(0, 0, 0, 0));

    println!("time to resize: {}", rescale_time.elapsed().as_millis());

    camera.present_to_screen(canvas, (width, height));

    let present_time = std::time::Instant::now();

    let surface = surface.as_mut().unwrap();
    surface.resize(NonZeroU32::new(width).unwrap(), NonZeroU32::new(height).unwrap()).unwrap();

    let mut buffer = surface.buffer_mut().unwrap();
    buffer.copy_from_slice(&canvas.get_pixels_u32());

    buffer.present().unwrap();

    println!("time to present: {}", present_time.elapsed().as_millis());
}
