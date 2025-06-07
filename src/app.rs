use std::{sync::{atomic::{AtomicBool, Ordering}, OnceLock}, time::Duration};

use game_loop::game_loop;

use crate::{event::Event, fatal, prelude::Buffer, render::Renderer, terminal::Terminal, utils::Size};

pub mod prelude {
    pub use super::TermApp;
    pub use super::{exit, should_exit};
}

static EXIT: OnceLock<AtomicBool> = OnceLock::new();

pub fn exit() {
    EXIT.get_or_init(|| AtomicBool::new(false)).store(true, Ordering::SeqCst);
}

pub fn should_exit() -> bool {
    EXIT.get_or_init(|| AtomicBool::new(false)).load(Ordering::SeqCst)
}

pub struct TermApp {
    renderer: Renderer,
    terminal: Terminal,
    intermediate_buffer: Buffer<u8>,
    timeout: Duration,
}

impl TermApp {
    pub fn new(timeout: Duration) -> TermApp {
        let (cols, rows) = crossterm::terminal::size()
            .unwrap_or_else(|e| fatal!("Failed to get terminal size: {e}"));

        let renderer = Renderer::new(Size::new_terminal(cols, rows))
            .unwrap_or_else(|e| fatal!("Failed to initialize renderer: {e}"));

        let Size::Renderer(width, height) = renderer.size().to_renderer() else { unreachable!() };
        let unpadded_bytes_per_row = width * 4;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT; // 256
        let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;

        let intermediate_buffer = Buffer::new(
                &renderer, 
                (padded_bytes_per_row as usize) * (height as usize), 
                wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::MAP_READ,
            );

        TermApp {
            renderer,
            intermediate_buffer,
            terminal: Terminal::new(cols, rows),
            timeout,
        }
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn terminal(&self) -> &Terminal {
        &self.terminal
    }

    pub fn terminal_mut(&mut self) -> &mut Terminal {
        &mut self.terminal
    }

    pub fn size(&self) -> Size {
        self.renderer().size()
    }

    pub fn run<F>(mut self, handler: F)
    where
        F: FnMut(Event),
    {
        use std::cell::RefCell;
        let handler = RefCell::new(handler);

        // Enable terminal
        self.terminal.enable();

        game_loop(
            &mut self, 240, 0.1, 
            |update_game| {
                if should_exit() {
                    update_game.exit();
                }

                // 1. Input
                if let Some(input) = update_game.game.terminal.input(Duration::from_millis(10)) {
                    handler.borrow_mut()(Event::Input(input));
                }

                // 2. Check resize
                if let Some(size) = update_game.game.terminal.resized() {
                    let size = size.to_renderer();
                    let Size::Renderer(width, height) = size else { unreachable!() };

                    let unpadded_bytes_per_row = width * 4;
                    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT; // 256
                    let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;

                    update_game.game.renderer.resize_with(size);
                    update_game.game.intermediate_buffer.resize(
                        &update_game.game.renderer,
                        (padded_bytes_per_row as usize) * (height as usize),
                    );
                    handler.borrow_mut()(Event::Resize(size));
                }

                // 3. Update
                handler.borrow_mut()(Event::Update);
            },
            |render_game| {
                let renderer = &mut render_game.game.renderer;

                // 4. Render
                handler.borrow_mut()(Event::Render(renderer));

                // 4.1. Draw rendered texture to terminal
                // 4.1.1. Copy texture to intermediate buffer
                let mut draw_ctx = renderer.draw_ctx();
                let canvas = renderer.canvas();
                let width = canvas.texture().descriptor().width;
                let height = canvas.texture().descriptor().height;

                draw_ctx.copy_texture_to_buffer(
                    canvas.texture(),
                    &render_game.game.intermediate_buffer,
                );

                draw_ctx.apply(canvas, renderer);

                let data = pollster::block_on(render_game.game.intermediate_buffer.read_bytes(renderer));
                render_game.game.intermediate_buffer.unmap();

                // 4.1.2. Save the texture to a vec of bytes
                let unpadded_bytes_per_row = width * 4;
                let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT; // 256
                let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;

                let mut raw_data = Vec::with_capacity((width * height * 4) as usize);
                for row in 0..height {
                    let start = (row * padded_bytes_per_row) as usize;
                    let end = start + unpadded_bytes_per_row as usize;
                    raw_data.extend_from_slice(&data[start..end]);
                }

                // 4.1.3. Print the image to the terminal
                render_game.game.terminal.clear();
                render_game.game.terminal.print_image(&raw_data, width, height);

                // 5. Draw UI
                // TODO: Implement UI context

                std::thread::sleep(render_game.game.timeout);
            },
        );

        self.terminal.disable();
    }
}