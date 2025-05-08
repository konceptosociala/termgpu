use std::io::stdout;

use crossterm::{execute, style::{Color, SetForegroundColor}, terminal::{Clear, ClearType}};

pub fn clear() {
    let mut stdout = stdout();

    loop {
        execute!(stdout, 
            Clear(ClearType::All),
            SetForegroundColor(Color:)
        ).unwrap();
    }
}

pub fn set_double_pixel(upper: Pixel, lower: Pixel) {

}

pub struct Pixel {

}