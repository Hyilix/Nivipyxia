// Module responsible for handling the window and rendering to it
use std::rc::Rc;
use std::collections::HashSet;

use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, EventLoop, ControlFlow};
use winit::event;
use event::WindowEvent;
use winit::window::{Window, WindowId, WindowAttributes};
use winit::keyboard::{PhysicalKey, KeyCode};
use softbuffer::{Context, Surface};

use nivis::canvas;
use nivis::geometry;
use nivis::draw::{self, rectangle};
use nivis::color::Color;
use geometry::{triangle::Triangle, point::Point, rectangle::Rectangle};

use crate::events::{redraw_request, mouse_pressed};

use crate::mouse;
use crate::render::camera;

#[derive(Default)]
pub struct App {
    window: Option<Rc<dyn Window>>,
    context: Option<Context<Rc<dyn Window>>>,
    surface: Option<Surface<Rc<dyn Window>, Rc<dyn Window>>>,
    canvas: canvas::Canvas,
    camera: camera::Camera,
    mouse: mouse::Mouse,
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
            println!("window size {:?}", size);
            (size.width, size.height)
        };

        self.canvas = canvas::Canvas::new(width, height, Color::new(0, 0, 0, 0xFF));
        self.camera = camera::Camera::new(0, 0, width / 2, width / 2, 32f32);

        self.mouse = mouse::Mouse::default();

        // NOTE: Arbitrary values for camera testing
        // self.camera = camera::Camera::new(width / 2 - width / 16, height / 2 - height / 16, width / 8, height / 8, 1f32);

        // let line_test = geometry::line::Line::new(0, 4, 7, 1);
        // let rect_test = geometry::rectangle::Rectangle::new(0, 0, 5, 7);
        // let circle_test = geometry::circle::Circle::new(8, 8, 4);
        // let trig_test = Triangle::new(
        //     Point::pair_to_point((0, 0)),
        //     Point::pair_to_point((5, 15)),
        //     Point::pair_to_point((12, 3))
        // );

        // NOTE: Camera testing
        self.camera.get_canvas_mut().fill(Color::new(0xAA, 0xAA, 0xAA, 0));

        self.camera.get_canvas_mut().checkerboard(Color::new(0xDD, 0xDD, 0xDD, 0));

        // draw::line::simple(self.camera.get_canvas_mut(), &line_test, 0x00FF0000);
        // self.camera.get_canvas_mut().fill(0x00AAAAAA);
        // draw::circle::filled(self.camera.get_canvas_mut(), &circle_test, 0x00FF0000);
        // draw::triangle::filled(self.camera.get_canvas_mut(), &trig_test, Color::new(0xFF, 0, 0, 0));
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
            // TODO: simplify redraw request function
            WindowEvent::RedrawRequested => {
                redraw_request::redraw_request(&mut self.canvas, &mut self.camera, &mut self.surface, &self.window);
            },
            WindowEvent::KeyboardInput {event, .. } => {
                println!("Keyboard event: {:?}", event);
                if !event.state.is_pressed() {
                    return;
                }

                // TODO: Make the vector add with input to move once, so it can move both dimensions
                if let PhysicalKey::Code(code) = event.physical_key {
                    println!("code: {:?}", code);

                    // Get the direction vector to move the camera with
                    let mut move_vec = Point::pair_to_point((0, 0));
                    let moved = match code {
                        winit::keyboard::KeyCode::ArrowUp => {
                            println!("Move UP!");
                            move_vec += Point::new(0, 1);
                            true
                        },
                        winit::keyboard::KeyCode::ArrowDown => {
                            println!("Move DOWN!");
                            move_vec += Point::new(0, -1);
                            true
                        },
                        winit::keyboard::KeyCode::ArrowLeft => {
                            println!("Move LEFT!");
                            move_vec += Point::new(1, 0);
                            true
                        },
                        winit::keyboard::KeyCode::ArrowRight => {
                            println!("Move RIGHT!");
                            move_vec += Point::new(-1, 0);
                            true
                        },
                        _ => false
                    };

                    // Update camera position and update screen
                    if moved {
                        self.camera.move_with(move_vec);
                        self.window.as_ref().unwrap().request_redraw();
                    }
                }
            },
            WindowEvent::PointerButton {state, position, ..} => {
                println!("Mouse state is : {:?} at position {} {}.", state, position.x, position.y);
                self.mouse.update_position(position);
                self.mouse.set_pressed(event::ElementState::is_pressed(state));

                let win_size: (u32, u32) = {
                    let size = self.window.as_ref().unwrap().surface_size();
                    (size.width, size.height)
                };

                // Get information about the displayed canvas
                let can_size = self.camera.get_camera_drawing_size(win_size);
                let can_rect = Rectangle::new(
                    (win_size.0 / 2 - can_size.0 / 2) as i32,
                    (win_size.1 / 2 - can_size.1 / 2) as i32,
                    can_size.0,
                    can_size.1
                );

                // Mouse not in canvas
                if !can_rect.has_point(Point::pair_to_point((position.x as i32, position.y as i32))) {
                    return;
                }

                let draw_pos: Point = Point::new(
                    (position.x as i32 - can_rect.corner.x) / self.camera.get_zoom() as i32,
                    (position.y as i32 - can_rect.corner.y) / self.camera.get_zoom() as i32
                );

                println!("can_rect = {:?} ; draw_pos = {:?}", can_rect, draw_pos);

                let point_add = self.camera.get_corner();

                self.camera.get_canvas_mut().draw_point(draw_pos + point_add, Color::new(0xFF, 0, 0, 0));

                // Update screen
                self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::MouseWheel {delta, ..} => {
                println!("delta: {:?}", delta);

                if let event::MouseScrollDelta::LineDelta(_, val) = delta {
                    // Change zoom
                    self.camera.add_zoom(val);
                    self.window.as_ref().unwrap().request_redraw();
                }
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
