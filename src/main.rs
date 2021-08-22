#![feature(box_syntax)]
#![deny(clippy::all)]
#![forbid(unsafe_code)]

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod app;
mod buffer;
mod canvas;
mod color;
mod tools;
mod utils;
mod widget;

use app::App;
use canvas::{CANVAS_HEIGHT, CANVAS_WIDTH};
use tools::{Circe, Penicilin, Rectangel, Linen};

const BORDER_WIDTH: u32 = 1;
const COLOR_PICKER_SIZE: u32 = 5;
const WIDTH: u32 = CANVAS_WIDTH;
const HEIGHT: u32 = CANVAS_HEIGHT + 3 * BORDER_WIDTH + COLOR_PICKER_SIZE;

const PIXEL_SCALE: f64 = 4.0;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64 * PIXEL_SCALE, HEIGHT as f64 * PIXEL_SCALE);
        WindowBuilder::new()
            .with_title("pain... t")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut app = App::new();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            app.draw(pixels.get_frame());

            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            let (mouse_cell, prev_mouse_cell) = input
                .mouse()
                .map(|(mx, my)| {
                    let (dx, dy) = input.mouse_diff();
                    let prev_x = mx - dx;
                    let prev_y = my - dy;

                    let (mx_i, my_i) = pixels
                        .window_pos_to_pixel((mx, my))
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

                    let (px_i, py_i) = pixels
                        .window_pos_to_pixel((prev_x, prev_y))
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

                    (
                        (mx_i as isize, my_i as isize),
                        (px_i as isize, py_i as isize),
                    )
                })
                .unwrap_or_default();

            if input.mouse_pressed(0) {
                app.handle_press(mouse_cell);
            } else if input.mouse_held(0) {
                app.handle_hold(prev_mouse_cell, mouse_cell);
            } else if input.mouse_released(0) {
                app.handle_release(mouse_cell);
            }

            if input.key_pressed(VirtualKeyCode::Key1) {
                app.switch_tool(box Penicilin {});
            } else if input.key_pressed(VirtualKeyCode::Key2) {
                app.switch_tool(box Rectangel::new());
            } else if input.key_pressed(VirtualKeyCode::Key3) {
                app.switch_tool(box Circe::new());
            } else if input.key_pressed(VirtualKeyCode::Key4) {
                app.switch_tool(box Linen::new());
            }

            window.request_redraw();
        }
    });
}
