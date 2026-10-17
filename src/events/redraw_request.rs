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

    camera.set_presentation_width(width / 2);
    camera.set_presentation_height(width / 2);

    // let sca_now = std::time::Instant::now();
    canvas.scale_to(width, height);
    canvas.fill(Color::new(0, 0, 0, 0xFF));
    // println!("Time for (scale_to): {}", sca_now.elapsed().as_millis());

    // let now = std::time::Instant::now();
    camera.present_to_screen(canvas, (width, height));
    // println!("Time for (present_to_screen): {}", now.elapsed().as_millis());

    // let copy = std::time::Instant::now();

    let surface = surface.as_mut().unwrap();
    surface.resize(NonZeroU32::new(width).unwrap(), NonZeroU32::new(height).unwrap()).unwrap();

    let mut buffer = surface.buffer_mut().unwrap();
    buffer.copy_from_slice(&canvas.get_pixels_u32());
    // println!("Time for copy to buffer: {}", copy.elapsed().as_millis());

    buffer.present().unwrap();
}
