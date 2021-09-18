use crate::{
    buffer::GuardedBuffer,
    canvas::CANVAS_HEIGHT,
    color::Color,
    widget::Widget,
    BORDER_WIDTH,
};

pub const BUTTON_SIZE: u32 = 5;

pub struct ColorPicker {
    buttons: Vec<ColorButton>,
}

struct ColorButton {
    color: Color,
    x: isize,
    y: isize,
}

impl ColorPicker {
    pub fn new() -> Self {
        let y = (CANVAS_HEIGHT + 2 * BORDER_WIDTH) as isize;
        let buttons = vec![
            (0xff, 0x00, 0x00).into(),
            (0x00, 0x00, 0x00).into(),
            (0x00, 0xff, 0x00).into(),
            (0x00, 0x00, 0xff).into(),
            (0xff, 0xff, 0xff).into(),
        ]
        .iter()
        .copied()
        .enumerate()
        .map(|(i, color)| {
            let x = (BUTTON_SIZE as isize + BORDER_WIDTH as isize) * i as isize
                + BORDER_WIDTH as isize;

            ColorButton { color, x, y }
        })
        .collect();

        Self { buttons }
    }

    pub fn pick_color(&mut self, mouse: (isize, isize)) -> Option<Color> {
        for button in &self.buttons {
            if button.clicked(mouse) {
                return Some(button.color);
            }
        }

        None
    }
}

impl Widget for ColorPicker {
    fn display(&self, buffer: &mut GuardedBuffer<'_, '_>) {
        for button in &self.buttons {
            button.display(buffer);
        }
    }
}

impl Widget for ColorButton {
    fn display(&self, buffer: &mut GuardedBuffer<'_, '_>) {
        for x in 0..(BUTTON_SIZE as isize) {
            for y in 0..(BUTTON_SIZE as isize) {
                buffer.put_pixel(
                    (x + self.x) as usize,
                    (y + self.y) as usize,
                    self.color,
                );
            }
        }
    }
}

impl ColorButton {
    fn clicked(&self, (mouse_x, mouse_y): (isize, isize)) -> bool {
        (0..(BUTTON_SIZE as isize)).contains(&(mouse_x - self.x))
            && (0..(BUTTON_SIZE as isize)).contains(&(mouse_y - self.y))
    }
}
