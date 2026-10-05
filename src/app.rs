// Module responsible for handling the window and rendering to it
use std::rc::Rc;
use std::num::NonZeroU32;

use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, EventLoop, ControlFlow};
use winit::event::WindowEvent;
use winit::window::{Window, WindowId, WindowAttributes};
use softbuffer::{Context, Surface};

use crate::engine::canvas;
use crate::render::camera;

use crate::engine::geometry;
use geometry::{triangle::Triangle, point::Point};
use crate::engine::draw;

#[derive(Default)]
pub struct App {
    window: Option<Rc<dyn Window>>,
    context: Option<Context<Rc<dyn Window>>>,
    surface: Option<Surface<Rc<dyn Window>, Rc<dyn Window>>>,

    canvas: canvas::Canvas,
    camera: camera::Camera,
}

impl ApplicationHandler for App {
    fn can_create_surfaces(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
    ) {
        let window: Rc<dyn Window> = Rc::from(event_loop.create_window(WindowAttributes::default()).unwrap());
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();

        self.context = Some(context);
        self.surface = Some(surface);
        self.window = Some(window);

        self.window.as_ref().unwrap().request_redraw();

        // NOTE: Canvas testing
        // self.canvas = canvas::Canvas::default();
        // self.canvas = canvas::Canvas::new(5, 5, 0xAA000000);
        // self.canvas.fill(0x00777777);
        // self.canvas.dither(0x00FFFFFF, 2);
        // self.canvas.scale_to(120, 120);

        let (width, height) = {
            let size = self.window.as_ref().unwrap().surface_size();
            (size.width, size.height)
        };

        self.canvas = canvas::Canvas::new(width, height, 0xFF000000);
        // NOTE: Arbitrary values for camera testing
        // self.camera = camera::Camera::new(width / 2 - width / 16, height / 2 - height / 16, width / 8, height / 8, 1f32);
        self.camera = camera::Camera::new(0, 0, width, height, 32f32);

        let line_test = geometry::line::Line::new(0, 4, 7, 1);
        let rect_test = geometry::rectangle::Rectangle::new(0, 0, 5, 7);
        let circle_test = geometry::circle::Circle::new(8, 8, 4);
        let trig_test = Triangle::new(
            Point::pair_to_point((0, 0)),
            Point::pair_to_point((5, 15)),
            Point::pair_to_point((12, 3))
        );

        // NOTE: Camera testing
        self.camera.get_canvas_mut().fill(0x00AAAAAA);

        self.camera.get_canvas_mut().checkerboard(0x00DDDDDD);

        // draw::line::simple(self.camera.get_canvas_mut(), &line_test, 0x00FF0000);
        // draw::rectangle::filled(self.camera.get_canvas_mut(), &rect_test, 0x00FF0000);
        // draw::circle::filled(self.camera.get_canvas_mut(), &circle_test, 0x00FF0000);
        draw::triangle::filled(self.camera.get_canvas_mut(), &trig_test, 0x00FF0000);
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window Closed via Request");
                event_loop.exit();
            },
            WindowEvent::Focused(state) => {
                println!("Window focus is now {state}");
            },
            WindowEvent::RedrawRequested => {
                let (width, height) = {
                    let size = self.window.as_ref().unwrap().surface_size();
                    (size.width, size.height)
                };

                self.canvas.scale_to(width, height);
                self.camera.present_to_screen(&mut self.canvas, (width, height));

                let surface = self.surface.as_mut().unwrap();
                surface.resize(NonZeroU32::new(width).unwrap(), NonZeroU32::new(height).unwrap()).unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                buffer.copy_from_slice(self.canvas.get_pixels());

                // for pixel in buffer.iter_mut() {
                //     *pixel = 0x00000000;
                // }

                buffer.present().unwrap();

                // self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::KeyboardInput {event, .. } => {
                println!("Keyboard event: {:?}", event);
            },
            _ => (),
        }
    }
}

pub fn run_application() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new().unwrap();

    // event_loop.set_control_flow(ControlFlow::Wait);

    event_loop.run_app(App::default())?;

    Ok(())
}
