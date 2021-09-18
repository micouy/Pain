use crate::{
    buffer::GuardedBuffer,
    canvas::Canvas,
    color::Color,
    widget::Widget,
};

use super::Tool;

pub struct Penicilin {
    outline_color: Color,
}

impl Penicilin {
    pub fn new() -> Self {
        Self {
            outline_color: Color::black(),
        }
    }
}

impl Widget for Penicilin {
    fn display(&self, _buffer: &mut GuardedBuffer<'_, '_>) {}
}

impl Tool for Penicilin {
    fn handle_press(
        &mut self,
        (mouse_x, mouse_y): (isize, isize),
        canvas: &mut Canvas,
    ) {
        canvas.set_pixel(
            mouse_x as usize,
            mouse_y as usize,
            self.outline_color,
        );
    }

    fn handle_hold(
        &mut self,
        prev_mouse: (isize, isize),
        curr_mouse: (isize, isize),
        canvas: &mut Canvas,
    ) {
        super::plot_line(prev_mouse, curr_mouse)
            .into_iter()
            .for_each(|(x, y)| canvas.set_pixel(x, y, self.outline_color));
    }

    fn handle_release(&mut self, _mouse: (isize, isize), _canvas: &mut Canvas) {
    }

    fn set_outline_color(&mut self, outline_color: Color) {
        self.outline_color = outline_color;
    }
}
