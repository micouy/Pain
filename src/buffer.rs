use crate::{color::Color, HEIGHT, WIDTH};

use std::ops::Range;

pub struct Buffer<'p> {
    pixels: &'p mut [u8],
}

impl<'p> Buffer<'p> {
    pub fn new(pixels: &'p mut [u8]) -> Self {
        Self { pixels }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, Color { r, g, b }: Color) {
        if let Some(ix) = Self::calc_pixel_ix(x, y) {
            self.pixels[ix..(ix + 4)].copy_from_slice(&[r, g, b, 0xff]);
        }
    }

    pub fn lend<'b>(&'b mut self, guard: Box<dyn Guard>) -> GuardedBuffer<'b, 'p> {
        GuardedBuffer::new(self, guard)
    }

    fn clear(&mut self, Color { r, g, b }: Color) {
        for pixel in self.pixels.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[r, g, b, 0xff]);
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

pub trait Guard {
    fn contains_pixel(&self, x: usize, y: usize) -> bool;
}

impl<T> Guard for T
where
    T: Fn(usize, usize) -> bool,
{
    fn contains_pixel(&self, x: usize, y: usize) -> bool {
        (self)(x, y)
    }
}

impl Guard for (Range<usize>, Range<usize>) {
    fn contains_pixel(&self, x: usize, y: usize) -> bool {
        let range_x = &self.0;
        let range_y = &self.1;

        range_x.contains(&x) && range_y.contains(&y)
    }
}

impl Guard for ((usize, usize), (usize, usize)) {
    fn contains_pixel(&self, x: usize, y: usize) -> bool {
        let ((left, top), (width, height)) = *self;
        let range_x = left..(left + width);
        let range_y = top..(top + height);

        range_x.contains(&x) && range_y.contains(&y)
    }
}

pub struct GuardedBuffer<'b, 'p> {
    buffer: &'b mut Buffer<'p>,
    guard: Box<dyn Guard>,
}

impl<'b, 'p> GuardedBuffer<'b, 'p> {
    pub fn new(buffer: &'b mut Buffer<'p>, guard: Box<dyn Guard>) -> Self {
        Self { buffer, guard }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, color: Color) {
        if self.guard.contains_pixel(x, y) {
            self.buffer.put_pixel(x, y, color);
        }
    }
}
