# termgpu 📟 ❤️ 🖼️

A GPU-accelerated terminal graphics engine for Rust, enabling real-time rendering of 3D graphics directly in your terminal. Built on top of [wgpu](https://github.com/gfx-rs/wgpu) and [crossterm](https://github.com/crossterm-rs/crossterm), termgpu brings modern rendering techniques to the command line.

## Features

- Render 3D graphics in the terminal using Unicode and ANSI colors
- Real-time input handling (keyboard, resize)
- Customizable rendering pipelines (vertex, fragment, compute shaders)
- Easy-to-use abstractions for meshes, transformations, and pipelines
- Extensible UI context for overlays and widgets `(WIP)`

## Example

Render a rotating triangle in your terminal:

```rust
use std::time::Duration;
use termgpu::prelude::*;

fn main() {
    let mut app = TermApp::new(Duration::from_millis(20));

    let mut triangle = Triangle::default();
    triangle.update(app.renderer_mut());

    let mut transform = Transform::identity();
    let mut direction = -1.0;

    let pipeline = Pipeline::new_render(app.renderer(), &RenderPipelineDescriptor {
        shader: include_wgsl!("basic.wgsl"),
        bindings: &[],
        label: "Basic pipeline",
        use_vertices: true,
        surface_formats: &[TextureFormat::Rgba8Unorm]
    });

    app.run(|event: Event| {
        match event {
            Event::Resize(size) => {
                // Handle resize if needed
            },
            Event::Input(input) => {
                if input.kind == KeyEventKind::Press {
                    match input.code {
                        KeyCode::Esc => exit(),
                        KeyCode::Char(' ') => direction *= -1.0,
                        _ => {}
                    }
                }
            },
            Event::Update => {
                // Animate rotation
                transform.rotation *= glm::quat_angle_axis(
                    0.05 * direction, 
                    &glm::Vec3::z()
                );
            },
            Event::Render(renderer) => {
                let canvas = renderer.canvas();
                let canvases: &[&dyn RenderSurface] = &[&canvas];
                let mut ctx = renderer.draw_ctx();

                {
                    let mut render_pass = ctx.render_pass(canvases, renderer.depth_texture());
                    render_pass.draw(renderer, DrawDescriptor {
                        drawable: Some(&triangle),
                        instance_data: Some(&transform), 
                        pipeline: &pipeline,
                        shader_resources: &[],
                    });
                }

                ctx.apply(canvas, renderer);
            },
            Event::DrawUi(_ctx) => {
                // Draw UI overlays here (WIP)
            },
        }
    });
}
```

See [examples/basic.rs](examples/basic.rs) for the full example.

# Getting Started

1. Add to your Cargo.toml:
```toml
[dependencies]
termgpu = { path = "." }
```
2. Create a WGSL shader:
See [examples/basic.wgsl](examples/basic.wgsl) for a minimal shader.
3. Run the example:
```bash
cargo run --example basic
```

# Roadmap

<input checked="" disabled="" type="checkbox"> Basic 3D rendering in terminal

<input checked="" disabled="" type="checkbox"> Input and resize handling

<input checked="" disabled="" type="checkbox"> Compute shader support

<input disabled="" type="checkbox"> UI widgets (labels, buttons, overlays)

<input disabled="" type="checkbox"> More mesh primitives and materials

# License

This project is licensed under the Unlicense License. See the [LICENSE](LICENSE) file for details.