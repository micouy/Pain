use crate::{color::Color, HEIGHT, WIDTH};

pub struct Buffer<'a> {
    pixels: &'a mut [u8],
}

impl<'a> Buffer<'a> {
    pub fn new(pixels: &'a mut [u8]) -> Self {
        Self { pixels }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, Color { r, g, b }: Color) {
        if let Some(ix) = Self::calc_pixel_ix(x, y) {
            self.pixels[ix..(ix + 4)].copy_from_slice(&[r, g, b, 0xff]);
        }
    }

    pub fn lend<G>(&'a mut self, guard: G) -> GuardedBuffer<'a, G> where G: Guard {
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

pub trait Guard: FnMut(usize, usize) -> bool {}

impl<T> Guard for T where T: FnMut(usize, usize) -> bool {}

pub struct GuardedBuffer<'a, G> where G: Guard {
    buffer: &'a mut Buffer<'a>,
    guard: G,
}

impl<'a, G> GuardedBuffer<'a, G> where G: Guard {
    pub fn new(buffer: &'a mut Buffer<'a>, guard: G) -> Self {
        Self { buffer, guard }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, color: Color) {
        if (self.guard)(x, y) {
            self.buffer.put_pixel(x, y, color);
        }
    }
}
