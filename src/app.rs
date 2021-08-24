use crate::{
    buffer::Buffer,
    canvas::{Canvas, CANVAS_HEIGHT, CANVAS_WIDTH},
    color_picker::{self, ColorPicker},
    tools::{Rectangel, Tool},
    widget::Widget,
    BORDER_WIDTH,
};

pub struct App {
    canvas: Canvas,
    tool: Box<dyn Tool>,
    color_picker: ColorPicker,
}

impl App {
    pub fn new() -> Self {
        Self {
            canvas: Canvas::new(),
            tool: box Rectangel::new(),
            color_picker: ColorPicker::new(),
        }
    }

    pub fn handle_press(&mut self, mouse: (isize, isize)) {
        self.tool.handle_press(mouse, &mut self.canvas);
        if let Some(color) = self.color_picker.pick_color(mouse) {
            self.tool.set_outline_color(color);
        }
    }

    pub fn handle_hold(&mut self, prev_mouse: (isize, isize), curr_mouse: (isize, isize)) {
        self.tool
            .handle_hold(prev_mouse, curr_mouse, &mut self.canvas);
    }

    pub fn handle_release(&mut self, mouse: (isize, isize)) {
        self.tool.handle_release(mouse, &mut self.canvas);
    }

    pub fn switch_tool(&mut self, tool: Box<dyn Tool>) {
        self.tool = tool;
    }

    pub fn draw(&self, frame: &mut [u8]) {
        let mut buffer = Buffer::new(frame);

        {
            let mut canvas_buffer = buffer.lend(box {
                let (x, y) = (BORDER_WIDTH, BORDER_WIDTH);
                let (width, height) = (CANVAS_WIDTH, CANVAS_HEIGHT);

                ((x as usize, y as usize), (width as usize, height as usize))
            });

            self.canvas.display(&mut canvas_buffer);
            self.tool.display(&mut canvas_buffer);
        }

        let mut picker_buffer = buffer.lend(box {
            let x = BORDER_WIDTH;
            let y = 2 * BORDER_WIDTH + CANVAS_HEIGHT;
            let width = CANVAS_WIDTH;
            let height = color_picker::BUTTON_SIZE;

            ((x as usize, y as usize), (width as usize, height as usize))
        });
        self.color_picker.display(&mut picker_buffer);
    }
}
