use super::Tool;
use crate::{buffer::GuardedBuffer, canvas::Canvas, color::Color, widget::Widget};

pub struct Linen {
    origin: (isize, isize),
    mouse: (isize, isize),
    down: bool,
}

impl Linen {
    pub fn new() -> Self {
        Self {
            origin: (0, 0),
            mouse: (0, 0),
            down: false,
        }
    }
}

impl Widget for Linen {
    fn display(&self, buffer: &mut GuardedBuffer<'_>) {
        if !self.down {
            return;
        }

        super::plot_line(self.origin, self.mouse)
            .into_iter()
            .for_each(|(x, y)| buffer.put_pixel(x, y, Color::new(0xff, 0xff, 0x00)));
    }
}

impl Tool for Linen {
    fn handle_press(&mut self, (mouse_x, mouse_y): (isize, isize), _canvas: &mut Canvas) {
        self.down = true;
        self.origin = (mouse_x, mouse_y);
        self.mouse = self.origin;
    }

    fn handle_hold(
        &mut self,
        _prev_mouse: (isize, isize),
        curr_mouse: (isize, isize),
        _canvas: &mut Canvas,
    ) {
        self.mouse = curr_mouse;
    }

    fn handle_release(&mut self, mouse: (isize, isize), canvas: &mut Canvas) {
        self.mouse = mouse;
        self.down = false;

        super::plot_line(self.origin, self.mouse)
            .into_iter()
            .for_each(|(x, y)| canvas.set_pixel(x, y, Color::new(0xff, 0xff, 0x00)));
    }
}
