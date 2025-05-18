use rand::{distr::Alphanumeric, Rng};
use termgpu::render::{hal::pipeline::{Pipeline, RenderPipelineDescriptor}, DrawDescriptor, Renderer};
use std::{io::{self, Write}, thread, time::Duration};
use crossterm::{
    cursor::{self, Hide, MoveTo}, event::{poll, read, Event, KeyCode, KeyEvent}, style::{self, Color, Stylize}, terminal::{self, window_size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, QueueableCommand
};

fn main() -> io::Result<()> {
    let mut renderer = Renderer::new(0, 0).unwrap();

    let canvas = renderer.canvas();
    let mut ctx = renderer.draw_ctx();

    {
        let canvases: &[&dyn termgpu::render::RenderSurface] = &[&canvas];
        let mut render_pass = ctx.render_pass(canvases, renderer.depth_texture());

        render_pass.draw::<()>(&renderer, DrawDescriptor {
            drawable: None,
            instance_data: None, 
            pipeline: &Pipeline::new_render(&renderer, todo!()),
            shader_resources: todo!(),
        });
    }

    ctx.apply(canvas, &renderer);

    let mut stdout = io::stdout();

    let size = window_size().unwrap();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?.execute(Hide)?;

    loop {
        stdout.execute(Clear(ClearType::All))?.execute(MoveTo(0, 0))?;

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

        let mut s = String::new();

        for _ in 0..size.columns*size.rows {
            let color = Color::Rgb { r: rand::random(), g: rand::random(), b: rand::random() };
            s.push_str(&"█".with(color).to_string());
        }

        println!("{}", s);

        std::thread::sleep(Duration::from_millis(16));
    }
}