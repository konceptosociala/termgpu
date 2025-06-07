use rand::{distr::Alphanumeric, Rng};
use termgpu::render::{hal::pipeline::{Pipeline, RenderPipelineDescriptor}, DrawDescriptor, Renderer};
use std::{io::{self, Write}, thread, time::Duration};
use crossterm::{
    cursor::{self, Hide, MoveTo}, event::{poll, read, Event, KeyCode, KeyEvent}, style::{self, Color, Stylize}, terminal::{self, window_size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, QueueableCommand
};

macro_rules! load_image_to_string {
    ($s:ident: $img:expr) => {
        let img = image::load_from_memory(include_bytes!($img)).unwrap().to_rgb8();
        let mut $s = String::new();
        for y in 0..img.height() {
            for x in 0..img.width() {
                let pixel = img.get_pixel(x, y);
                let color = Color::Rgb {
                    r: pixel.0[0],
                    g: pixel.0[1],
                    b: pixel.0[2],
                };
                $s.push_str(&"█".with(color).to_string());
            }
        }
    };
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let size = window_size().unwrap();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?.execute(Hide)?;

    load_image_to_string!(s1: "../img1.png");
    load_image_to_string!(s2: "../img2.png");

    loop {
        if poll(Duration::ZERO)? {
            if let Event::Key(KeyEvent {
                code: KeyCode::Esc,
                ..
            }) = read()? {
                terminal::disable_raw_mode()?;
                stdout.execute(LeaveAlternateScreen)?;
                return Ok(());
            }
        }

        stdout.execute(Clear(ClearType::All))?.execute(MoveTo(0, 0))?;
        print!("{s1}");
        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(50));

        stdout.execute(Clear(ClearType::All))?.execute(MoveTo(0, 0))?;
        print!("{s2}");
        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(50));
    }
}