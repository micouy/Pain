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
mod widget;
mod tools;

use buffer::Buffer;
use canvas::{Canvas, CANVAS_HEIGHT, CANVAS_WIDTH};
use widget::Widget;
use tools::Pencil;

const BORDER_WIDTH: u32 = 1;
const COLOR_PICKER_SIZE: u32 = 5;
const WIDTH: u32 = CANVAS_WIDTH;
const HEIGHT: u32 = CANVAS_HEIGHT + 3 * BORDER_WIDTH + COLOR_PICKER_SIZE;

const PIXEL_SCALE: f64 = 4.0;

struct App {
    canvas: Canvas,
    tool: Pencil,
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

            if input.mouse_pressed(0) || input.mouse_held(0) {
                app.handle_click(prev_mouse_cell, mouse_cell);
            }

            window.request_redraw();
        }
    });
}

impl App {
    fn new() -> Self {
        Self {
            canvas: Canvas::new(),
            tool: Pencil {},
        }
    }

    fn handle_click(&mut self, prev_mouse: (isize, isize), curr_mouse: (isize, isize)) {
        self.tool.handle_click(prev_mouse, curr_mouse, &mut self.canvas)
    }

    fn draw(&self, frame: &mut [u8]) {
        let mut buffer = Buffer::new(frame);
        let mut canvas_buffer = buffer.lend(|x, y| {
            let offset_x = x - BORDER_WIDTH as usize;
            let offset_y = y - BORDER_WIDTH as usize;
            (0..(CANVAS_WIDTH as usize)).contains(&offset_x)
                && (0..(CANVAS_HEIGHT as usize)).contains(&offset_y)
        });

        self.canvas.display(&mut canvas_buffer);
    }
}
