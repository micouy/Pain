use crate::{canvas::Canvas, widget::Widget};

pub mod circe;
pub mod penicilin;
pub mod rectangel;
pub mod linen;

pub use circe::Circe;
pub use penicilin::Penicilin;
pub use rectangel::Rectangel;
pub use linen::Linen;

pub trait Tool: Widget {
    fn handle_press(&mut self, mouse: (isize, isize), canvas: &mut Canvas);
    fn handle_hold(
        &mut self,
        prev_mouse: (isize, isize),
        curr_mouse: (isize, isize),
        canvas: &mut Canvas,
    );
    fn handle_release(&mut self, mouse: (isize, isize), canvas: &mut Canvas);
}

fn plot_line(
    (prev_x, prev_y): (isize, isize),
    (curr_x, curr_y): (isize, isize),
) -> Vec<(usize, usize)> {
    let d_x = curr_x - prev_x;
    let d_y = curr_y - prev_y;

    if d_x == 0 && d_y == 0 {
        vec![(curr_x as usize, curr_y as usize)]
    } else if d_x.abs() > d_y.abs() || d_y == 0 {
        let min_x = curr_x.min(prev_x);
        let max_x = curr_x.max(prev_x);
        let m = d_y as f32 / d_x as f32;

        (min_x..=max_x)
            .map(|pixel_x| {
                let pixel_y = prev_y as f32 + m * (pixel_x - prev_x) as f32;
                let pixel_y = pixel_y.round() as isize;

                (pixel_x as usize, pixel_y as usize)
            })
            .collect()
    } else {
        let min_y = curr_y.min(prev_y);
        let max_y = curr_y.max(prev_y);
        let m = d_x as f32 / d_y as f32;

        (min_y..=max_y)
            .map(|pixel_y| {
                let pixel_x = prev_x as f32 + m * (pixel_y - prev_y) as f32;
                let pixel_x = pixel_x.round() as isize;

                (pixel_x as usize, pixel_y as usize)
            })
            .collect()
    }
}
