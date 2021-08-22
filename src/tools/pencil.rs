use crate::{canvas::Canvas, color::Color};

pub struct Pencil {}

impl Pencil {
    pub fn handle_click(&self, prev_mouse: (isize, isize), curr_mouse: (isize, isize), canvas: &mut Canvas) {
        super::plot_line(prev_mouse, curr_mouse)
            .into_iter()
            .for_each(|(x, y)| canvas.set_pixel(x, y, Color::new(0xff, 0x00, 0x00)));
    }
}
