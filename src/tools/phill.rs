use crate::{
    buffer::GuardedBuffer,
    canvas::Canvas,
    color::Color,
    widget::Widget,
};

use super::Tool;

use std::collections::HashSet;

pub struct Phill {
    outline_color: Color,
}

impl Phill {
    pub fn new() -> Self {
        Self {
            outline_color: Color::black(),
        }
    }
}

impl Widget for Phill {
    fn display(&self, _buffer: &mut GuardedBuffer<'_, '_>) {}
}

impl Tool for Phill {
    fn handle_press(&mut self, (x, y): (isize, isize), canvas: &mut Canvas) {
        let mut checked = HashSet::new();
        let mut queue = vec![(x, y)];
        let color = match canvas.get_pixel(x as usize, y as usize) {
            Some(color) => color,
            _ => return,
        };
        let neighbours = |x: isize, y: isize| {
            [
                (x, y + 1),
                (x, (y - 1).max(0)),
                (x + 1, y),
                ((x - 1).max(0), y),
            ]
        };

        while let Some((x, y)) = queue.pop() {
            checked.insert((x, y));

            match canvas.get_pixel(x as usize, y as usize) {
                Some(pixel) if pixel == color => {
                    canvas.set_pixel(
                        x as usize,
                        y as usize,
                        self.outline_color,
                    );

                    queue.extend(
                        neighbours(x, y)
                            .iter()
                            .filter(|coords| !checked.contains(coords)),
                    );
                }
                _ => {}
            }
        }
    }

    fn handle_hold(
        &mut self,
        _prev_mouse: (isize, isize),
        _curr_mouse: (isize, isize),
        _canvas: &mut Canvas,
    ) {
    }

    fn handle_release(&mut self, _mouse: (isize, isize), _canvas: &mut Canvas) {
    }

    fn set_outline_color(&mut self, outline_color: Color) {
        self.outline_color = outline_color;
    }
}
