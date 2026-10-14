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

    canvas.scale_to(width, height);
    canvas.fill(Color::new(0, 0, 0, 0xFF));

    camera.present_to_screen(canvas, (width, height));

    let surface = surface.as_mut().unwrap();
    surface.resize(NonZeroU32::new(width).unwrap(), NonZeroU32::new(height).unwrap()).unwrap();

    let mut buffer = surface.buffer_mut().unwrap();
    buffer.copy_from_slice(&canvas.get_pixels_u32());

    buffer.present().unwrap();
}
