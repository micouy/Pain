use super::Tool;
use crate::{canvas::Canvas, color::Color, widget::Widget};

use std::collections::HashSet;

pub struct Phill {}

impl Phill {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for Phill {}

impl Tool for Phill {
    fn handle_press(&mut self, (x, y): (isize, isize), canvas: &mut Canvas) {
        let fill = Color::new(0xff, 0x00, 0xff);
        let mut checked = HashSet::new();
        let mut queue = vec![(x, y)];
        let color = canvas.get_pixel(x as usize, y as usize).unwrap();
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
                    canvas.set_pixel(x as usize, y as usize, fill);

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
}
