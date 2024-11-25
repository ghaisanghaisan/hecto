use super::terminal::{Size, Terminal};
use std::io::Error;

pub mod buffer;
use buffer::Buffer;

#[derive(Default)]
pub struct View {
    pub buffer: Buffer,
}

impl View {
    pub fn load(&mut self, file_name: &String) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
        }
    }
    pub fn render_welcome() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Terminal::clear_line()?;

            if current_row == height / 3 {
                Self::draw_welcome_msg()?;
            } else {
                Terminal::print("~")?;
            }

            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }
    pub fn render_buffer(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Terminal::clear_line()?;

            if let Some(line) = self.buffer.lines.get(current_row) {
                Terminal::print(line)?;
            } else {
                Terminal::print("~")?;
            }

            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }
    pub fn render(&self) -> Result<(), Error> {
        if self.buffer.is_empty() {
            Self::render_welcome()?;
        } else {
            self.render_buffer()?;
        }
        Ok(())
    }

    fn draw_welcome_msg() -> Result<(), Error> {
        let Size { width, .. } = Terminal::size()?;
        let welcome_msg = "Welcome to Hecto v1.0.0";
        let len = welcome_msg.len();

        let padding = " ".repeat((width / 2).saturating_sub(len / 2));

        let line = format!("~{padding}{welcome_msg}");

        Terminal::print(&line)?;
        Ok(())
    }
}
