#![deny(clippy::all)]
#![forbid(unsafe_code)]

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const CANVAS_WIDTH: u32 = 200;
const CANVAS_HEIGHT: u32 = 100;
const BORDER_WIDTH: u32 = 1;
const COLOR_PICKER_SIZE: u32 = 5;
const WIDTH: u32 = CANVAS_WIDTH;
const HEIGHT: u32 = CANVAS_HEIGHT + 3 * BORDER_WIDTH + COLOR_PICKER_SIZE;

const PIXEL_SCALE: f64 = 4.0;

struct Canvas {
    inner: [[Color; CANVAS_WIDTH as usize]; CANVAS_HEIGHT as usize],
    x: usize,
    y: usize,
}

impl Canvas {
    fn new() -> Self {
        Self {
            inner: [[Color::white(); CANVAS_WIDTH as usize]; CANVAS_HEIGHT as usize],
            x: BORDER_WIDTH as usize,
            y: BORDER_WIDTH as usize,
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.inner[y][x] = color;
    }

    fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.inner[y][x]
    }
}

trait Drawable {
    fn draw(&self, buffer: &mut Buffer<'_>);
}

impl Drawable for Canvas {
    fn draw(&self, buffer: &mut Buffer<'_>) {
        for (y, row) in self.inner.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                let buffer_x = x + self.x;
                let buffer_y = y + self.y;

                buffer.put_pixel(buffer_x, buffer_y, *pixel);
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn white() -> Self {
        Self { r: 0xff, g: 0xff, b: 0xff }
    }

    fn black() -> Self {
        Self { r: 0x00, g: 0x00, b: 0x00 }
    }
}

struct Buffer<'a> {
    pixels: &'a mut [u8],
}

impl<'a> Buffer<'a> {
    fn new(pixels: &'a mut [u8]) -> Self {
        Self { pixels }
    }

    fn put_pixel(&mut self, x: usize, y: usize, Color { r, g, b }: Color) {
        if let Some(ix) = Self::calc_pixel_ix(x, y) {
            self.pixels[ix..(ix + 4)].copy_from_slice(&[r, g, b, 0xff]);
        }
    }

    fn clear(&mut self) {
        for pixel in self.pixels.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
        }
    }

    fn calc_pixel_ix(x: usize, y: usize) -> Option<usize> {
        if (0..WIDTH as usize).contains(&x) && (0..HEIGHT as usize).contains(&y) {
            Some((x + y * WIDTH as usize) * 4)
        } else {
            None
        }
    }
}

struct World {
    canvas: Canvas,
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
    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());

            if pixels
                .render()
                .is_err()
            {
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

            let (mx, my) = input
                .mouse()
                .map(|(mx, my)| {
                    let (mx_i, my_i) = pixels
                        .window_pos_to_pixel((mx, my))
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

                        (mx_i as usize, my_i as usize)
                })
                .unwrap_or_default();

            if input.mouse_pressed(0) {
                world.handle_click(mx, my);
            }

            window.request_redraw();
        }
    });
}

impl World {
    fn new() -> Self {
        Self {
            canvas: Canvas::new(),
        }
    }

    fn handle_click(&mut self, x: usize, y: usize) {
        self.canvas.set_pixel(x, y, Color::black());
    }

    fn draw(&self, frame: &mut [u8]) {
        let mut buffer = Buffer::new(frame);

        self.canvas.draw(&mut buffer);
    }
}

