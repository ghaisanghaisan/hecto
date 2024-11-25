use super::terminal::{Position, Size, Terminal};
use std::io::Error;

pub mod buffer;
use buffer::Buffer;

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::move_cursor_to(Position { row: at, col: 0 })?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;

        Ok(())
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }

        let Size { height, width } = self.size;

        if height == 0 || width == 0 {
            return Ok(());
        }

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };

                Self::render_line(current_row, truncated_line)?;
            } else if current_row == height / 3 && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_msg()?)?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }

        self.needs_redraw = false;
        Ok(())
    }

    fn build_welcome_msg() -> Result<String, Error> {
        let Size { width, .. } = Terminal::size()?;
        let welcome_msg = "Welcome to Hecto v1.0.0";
        let len = welcome_msg.len();

        let padding = " ".repeat((width / 2).saturating_sub(len / 2));

        Ok(format!("~{padding}{welcome_msg}"))
    }
}
