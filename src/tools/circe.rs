use crate::{buffer::GuardedBuffer, canvas::Canvas, color::Color, widget::Widget};

use super::Tool;

use std::f32::consts::FRAC_1_SQRT_2;

pub struct Circe {
    origin: (isize, isize),
    radius: f32,
    down: bool,
    outline_color: Color,
}

impl Circe {
    pub fn new() -> Self {
        Self {
            origin: (0, 0),
            radius: 0.0,
            down: false,
            outline_color: Color::black(),
        }
    }
}

impl Widget for Circe {
    fn display(&self, buffer: &mut GuardedBuffer<'_, '_>) {
        if !self.down || self.radius == 0.0 {
            return;
        }

        let max_x = (FRAC_1_SQRT_2 * self.radius).ceil() as isize;

        for pixel_x in 0..=max_x {
            let pixel_y = (self.radius * ((1.0 - (pixel_x as f32 / self.radius).powf(2.0)).sqrt()))
                .round() as isize;

            for (m_x, m_y) in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
                buffer.put_pixel(
                    (self.origin.0 + (m_x * pixel_x)) as usize,
                    (self.origin.1 + (m_y * pixel_y)) as usize,
                    self.outline_color,
                );
                buffer.put_pixel(
                    (self.origin.0 + (m_x * pixel_y)) as usize,
                    (self.origin.1 + (m_y * pixel_x)) as usize,
                    self.outline_color,
                );
            }
        }
    }
}

impl Tool for Circe {
    fn handle_press(&mut self, mouse: (isize, isize), _canvas: &mut Canvas) {
        self.down = true;
        self.origin = mouse;
        self.radius = 0.0;
    }

    fn handle_hold(
        &mut self,
        _prev_mouse: (isize, isize),
        curr_mouse: (isize, isize),
        _canvas: &mut Canvas,
    ) {
        let d_x = (curr_mouse.0 - self.origin.0) as f32;
        let d_y = (curr_mouse.1 - self.origin.1) as f32;

        self.radius = (d_x.powf(2.0) + d_y.powf(2.0)).sqrt();
    }

    fn handle_release(&mut self, mouse: (isize, isize), canvas: &mut Canvas) {
        let d_x = (mouse.0 - self.origin.0) as f32;
        let d_y = (mouse.1 - self.origin.1) as f32;

        self.radius = (d_x.powf(2.0) + d_y.powf(2.0)).sqrt();

        if self.radius == 0.0 {
            return;
        }
        self.down = false;

        let max_x = (FRAC_1_SQRT_2 * self.radius.abs()).ceil() as isize;

        for pixel_x in 0..=max_x {
            let pixel_y = (self.radius.abs()
                * ((1.0 - (pixel_x as f32 / self.radius.abs()).powf(2.0)).sqrt()))
            .round() as isize;

            for (m_x, m_y) in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
                canvas.set_pixel(
                    (self.origin.0 + (m_x * pixel_x)) as usize,
                    (self.origin.1 + (m_y * pixel_y)) as usize,
                    self.outline_color,
                );
                canvas.set_pixel(
                    (self.origin.0 + (m_x * pixel_y)) as usize,
                    (self.origin.1 + (m_y * pixel_x)) as usize,
                    self.outline_color,
                );
            }
        }
    }

    fn set_outline_color(&mut self, outline_color: Color) {
        self.outline_color = outline_color;
    }
}
