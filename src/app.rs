use crate::{
    buffer::Buffer,
    canvas::{Canvas, CANVAS_HEIGHT, CANVAS_WIDTH},
    tools::{Rectangel, Tool},
    widget::Widget,
    BORDER_WIDTH,
};
pub struct App {
    canvas: Canvas,
    tool: Box<dyn Tool>,
}

impl App {
    pub fn new() -> Self {
        Self {
            canvas: Canvas::new(),
            tool: box Rectangel::new(),
        }
    }

    pub fn handle_press(&mut self, mouse: (isize, isize)) {
        self.tool.handle_press(mouse, &mut self.canvas)
    }

    pub fn handle_hold(&mut self, prev_mouse: (isize, isize), curr_mouse: (isize, isize)) {
        self.tool
            .handle_hold(prev_mouse, curr_mouse, &mut self.canvas)
    }

    pub fn handle_release(&mut self, mouse: (isize, isize)) {
        self.tool.handle_release(mouse, &mut self.canvas)
    }

    pub fn switch_tool(&mut self, tool: Box<dyn Tool>) {
        self.tool = tool;
    }

    pub fn draw(&self, frame: &mut [u8]) {
        let mut buffer = Buffer::new(frame);
        let mut canvas_buffer = buffer.lend(box |x: usize, y: usize| {
            let offset_x = x.checked_sub(BORDER_WIDTH as usize);
            let offset_y = y.checked_sub(BORDER_WIDTH as usize);

            let (offset_x, offset_y) =
                if let (Some(offset_x), Some(offset_y)) = (offset_x, offset_y) {
                    (offset_x, offset_y)
                } else {
                    return false;
                };

            (0..(CANVAS_WIDTH as usize)).contains(&offset_x)
                && (0..(CANVAS_HEIGHT as usize)).contains(&offset_y)
        });

        self.canvas.display(&mut canvas_buffer);
        self.tool.display(&mut canvas_buffer);
    }
}
