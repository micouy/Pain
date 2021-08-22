#![feature(box_syntax)]
#![deny(clippy::all)]
#![forbid(unsafe_code)]

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod buffer;
mod canvas;
mod color;
mod tools;
mod utils;
mod widget;

use buffer::Buffer;
use canvas::{Canvas, CANVAS_HEIGHT, CANVAS_WIDTH};
use tools::{Circe, Penicilin, Rectangel, Tool};
use widget::Widget;

const BORDER_WIDTH: u32 = 1;
const COLOR_PICKER_SIZE: u32 = 5;
const WIDTH: u32 = CANVAS_WIDTH;
const HEIGHT: u32 = CANVAS_HEIGHT + 3 * BORDER_WIDTH + COLOR_PICKER_SIZE;

const PIXEL_SCALE: f64 = 4.0;

struct App {
    canvas: Canvas,
    tool: Box<dyn Tool>,
}

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
            }

            window.request_redraw();
        }
    });
}

impl App {
    fn new() -> Self {
        Self {
            canvas: Canvas::new(),
            tool: box Rectangel::new(),
        }
    }

    fn handle_press(&mut self, mouse: (isize, isize)) {
        self.tool.handle_press(mouse, &mut self.canvas)
    }

    fn handle_hold(&mut self, prev_mouse: (isize, isize), curr_mouse: (isize, isize)) {
        self.tool
            .handle_hold(prev_mouse, curr_mouse, &mut self.canvas)
    }

    fn handle_release(&mut self, mouse: (isize, isize)) {
        self.tool.handle_release(mouse, &mut self.canvas)
    }

    fn switch_tool(&mut self, tool: Box<dyn Tool>) {
        self.tool = tool;
    }

    fn draw(&self, frame: &mut [u8]) {
        let mut buffer = Buffer::new(frame);
        let mut canvas_buffer = buffer.lend(box |x: usize, y: usize| {
            let offset_x = x.checked_sub(BORDER_WIDTH as usize);
            let offset_y = y.checked_sub(BORDER_WIDTH as usize);

            let (offset_x, offset_y) =
                if let (Some(offset_x), Some(offset_y)) = (offset_x, offset_y) {
                    (offset_x, offset_y)
                } else {
                    return false;
                };

            (0..(CANVAS_WIDTH as usize)).contains(&offset_x)
                && (0..(CANVAS_HEIGHT as usize)).contains(&offset_y)
        });

        self.canvas.display(&mut canvas_buffer);
        self.tool.display(&mut canvas_buffer);
    }
}
