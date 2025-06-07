use std::{
    io::{Stdout, stdout},
    time::Duration,
};

use crate::{event::input::KeyboardInput, fatal, utils::Size};
use crossterm::{cursor::MoveTo, event as ctevent, style::{Color, Stylize}, terminal::{Clear, ClearType}, ExecutableCommand};
use image::{ImageBuffer, Rgba};

pub mod prelude {
    pub use super::Terminal;
}
pub struct Terminal {
    cols: u16,
    rows: u16,
    stdout: Stdout,
}

impl Terminal {
    /// Creates a new terminal with the specified number of columns and rows.
    pub fn new(cols: u16, rows: u16) -> Terminal {
        Terminal {
            cols,
            rows,
            stdout: stdout(),
        }
    }

    pub fn enable(&mut self) {
        crossterm::terminal::enable_raw_mode()
            .unwrap_or_else(|e| fatal!("Failed to enable raw mode: {e}"));
        
        crossterm::execute!(
            self.stdout,
            crossterm::terminal::EnterAlternateScreen,
            crossterm::cursor::Hide,
        )
        .unwrap_or_else(|e| fatal!("Failed to clear terminal: {e}"));
    }

    pub fn disable(&mut self) {
        crossterm::terminal::disable_raw_mode()
            .unwrap_or_else(|e| fatal!("Failed to disable raw mode: {e}"));
        
        crossterm::execute!(
            self.stdout,
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::cursor::Show,
        )
        .unwrap_or_else(|e| fatal!("Failed to restore terminal: {e}"));
    }

    pub fn size(&self) -> Size {
        Size::Terminal(self.cols, self.rows)
    }

    pub fn clear(&mut self) {
        self.stdout
            .execute(Clear(ClearType::All)).unwrap_or_else(|e| fatal!("Failed to clear terminal: {e}"))
            .execute(MoveTo(0, 0)).unwrap_or_else(|e| fatal!("Failed to move cursor: {e}"));
    }

    pub fn print_image(&mut self, buf: &[u8], width: u32, height: u32) {
        let image = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, buf).unwrap();
        let mut s = String::new();
        
        for y in 0..(image.height() / 2) {
            for x in 0..image.width() {
                let top = image.get_pixel(x, y * 2);
                let top_color = Color::Rgb {
                    r: top.0[0],
                    g: top.0[1],
                    b: top.0[2],
                };

                let bottom = image.get_pixel(x, y * 2 + 1);
                let bottom_color = Color::Rgb {
                    r: bottom.0[0],
                    g: bottom.0[1],
                    b: bottom.0[2],
                };

                s.push_str(&"▀".with(top_color).on(bottom_color).to_string());
            }
        }

        self.stdout
            .execute(Clear(ClearType::All)).unwrap_or_else(|e| fatal!("Failed to clear terminal: {e}"))
            .execute(MoveTo(0, 0)).unwrap_or_else(|e| fatal!("Failed to move cursor: {e}"));

        print!("{s}");
    }

    pub fn print_at(&mut self, text: &str, x: u16, y: u16) {
        if x < self.cols && y < self.rows {
            crossterm::execute!(
                self.stdout,
                crossterm::cursor::MoveTo(x, y),
                crossterm::style::Print(text)
            )
            .unwrap_or_else(|e| fatal!("Failed to print at position ({x}, {y}): {e}"));
        } else {
            fatal!(
                "Position out of bounds: ({}, {}) for terminal size ({}, {})",
                x,
                y,
                self.cols,
                self.rows
            );
        }
    }

    pub fn resized(&self) -> Option<Size> {
        crossterm::terminal::size()
            .ok()
            .filter(|&(cols, rows)| cols != self.cols || rows != self.rows)
            .map(|(cols, rows)| Size::new_terminal(cols, rows))
    }

    pub fn input(&self, timeout: Duration) -> Option<KeyboardInput> {
        if ctevent::poll(timeout).ok()? {
            if let ctevent::Event::Key(ctevent::KeyEvent { code, kind, .. }) =
                ctevent::read().ok()?
            {
                return Some(KeyboardInput { code, kind });
            }
        }

        None
    }

    /// Resizes the terminal with the current terminal size.
    ///
    /// # Panics
    /// If the terminal size cannot be retrieved, this function will panic.
    pub fn resize_with_current(&mut self) {
        let size = crossterm::terminal::size()
            .unwrap_or_else(|e| fatal!("Failed to get terminal size: {}", e));

        self.cols = size.0;
        self.rows = size.1;
    }

    /// Resizes the terminal to the specified size.
    pub fn resize_with(&mut self, size: Size) {
        let Size::Terminal(cols, rows) = size.to_terminal() else { unreachable!("Expected terminal size") };

        self.cols = cols;
        self.rows = rows;
    }
}
