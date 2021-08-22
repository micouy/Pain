use super::Tool;
use crate::{
    buffer::{Guard, GuardedBuffer},
    canvas::Canvas,
    color::Color,
    widget::Widget,
};

pub struct Penicilin {}

impl Widget for Penicilin {
    fn display(&self, buffer: &mut GuardedBuffer<'_>) {
        // nothing to do...
    }
}

impl Tool for Penicilin {
    fn handle_press(&mut self, (mouse_x, mouse_y): (isize, isize), canvas: &mut Canvas) {
        canvas.set_pixel(
            mouse_x as usize,
            mouse_y as usize,
            Color::new(0xff, 0x00, 0x00),
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
            .for_each(|(x, y)| canvas.set_pixel(x, y, Color::new(0xff, 0x00, 0x00)));
    }

    fn handle_release(&mut self, mouse: (isize, isize), canvas: &mut Canvas) {
        // nothing to do...
    }
}
