use crate::{
    buffer::GuardedBuffer,
    color::Color,
    widget::Widget,
    BORDER_WIDTH,
};

pub const CANVAS_WIDTH: u32 = 200;
pub const CANVAS_HEIGHT: u32 = 100;

pub struct Canvas {
    inner: [[Color; CANVAS_WIDTH as usize]; CANVAS_HEIGHT as usize],
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            inner: [[Color::white(); CANVAS_WIDTH as usize];
                CANVAS_HEIGHT as usize],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let pixel = y
            .checked_sub(BORDER_WIDTH as usize)
            .and_then(|y| x.checked_sub(BORDER_WIDTH as usize).map(|x| (x, y)))
            .and_then(|(x, y)| {
                self.inner.get_mut(y).and_then(|row| row.get_mut(x))
            });
        if let Some(pixel) = pixel {
            *pixel = color;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        y.checked_sub(BORDER_WIDTH as usize)
            .and_then(|y| x.checked_sub(BORDER_WIDTH as usize).map(|x| (x, y)))
            .and_then(|(x, y)| self.inner.get(y).and_then(|row| row.get(x)))
            .copied()
    }
}

impl Widget for Canvas {
    fn display(&self, buffer: &mut GuardedBuffer<'_, '_>) {
        let offset_x = BORDER_WIDTH as usize;
        let offset_y = BORDER_WIDTH as usize;

        for (y, row) in self.inner.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                let buffer_x = x + offset_x;
                let buffer_y = y + offset_y;

                buffer.put_pixel(buffer_x, buffer_y, *pixel);
            }
        }
    }
}
