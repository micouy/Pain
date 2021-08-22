use super::Tool;
use crate::{
    buffer::{Guard, GuardedBuffer},
    canvas::Canvas,
    color::Color,
    utils,
    widget::Widget,
};

pub struct Rectangel {
    origin: (isize, isize),
    mouse: (isize, isize),
    down: bool,
}

impl Rectangel {
    pub fn new() -> Self {
        Self {
            origin: (0, 0),
            mouse: (0, 0),
            down: false,
        }
    }
}

impl Widget for Rectangel {
    fn display(&self, buffer: &mut GuardedBuffer<'_>) {
        if !self.down {
            return;
        }

        for x in utils::range_inclusive(self.origin.0, self.mouse.0) {
            buffer.put_pixel(
                x as usize,
                self.origin.1 as usize,
                Color::new(0x00, 0xff, 0x00),
            );
            buffer.put_pixel(
                x as usize,
                self.mouse.1 as usize,
                Color::new(0x00, 0xff, 0x00),
            );
        }

        for y in utils::range_inclusive(self.origin.1, self.mouse.1) {
            buffer.put_pixel(
                self.origin.0 as usize,
                y as usize,
                Color::new(0x00, 0xff, 0x00),
            );
            buffer.put_pixel(
                self.mouse.0 as usize,
                y as usize,
                Color::new(0x00, 0xff, 0x00),
            );
        }
    }
}

impl Tool for Rectangel {
    fn handle_press(&mut self, (mouse_x, mouse_y): (isize, isize), canvas: &mut Canvas) {
        self.down = true;
        self.origin = (mouse_x, mouse_y);
        self.mouse = self.origin;
    }

    fn handle_hold(
        &mut self,
        prev_mouse: (isize, isize),
        curr_mouse: (isize, isize),
        canvas: &mut Canvas,
    ) {
        self.mouse = curr_mouse;
    }

    fn handle_release(&mut self, mouse: (isize, isize), canvas: &mut Canvas) {
        self.mouse = mouse;
        self.down = false;

        for x in utils::range_inclusive(self.origin.0, self.mouse.0) {
            canvas.set_pixel(
                x as usize,
                self.origin.1 as usize,
                Color::new(0x00, 0xff, 0x00),
            );
            canvas.set_pixel(
                x as usize,
                self.mouse.1 as usize,
                Color::new(0x00, 0xff, 0x00),
            );
        }

        for y in utils::range_inclusive(self.origin.1, self.mouse.1) {
            canvas.set_pixel(
                self.origin.0 as usize,
                y as usize,
                Color::new(0x00, 0xff, 0x00),
            );
            canvas.set_pixel(
                self.mouse.0 as usize,
                y as usize,
                Color::new(0x00, 0xff, 0x00),
            );
        }
    }
}
