use termgpu::{event::Event, render::Renderer};

fn main() {
    let app = TermApp::new();

    let triangle = Triangle::default();
    let mut transform = Transform::identity();
    let mut direction = -1.0;
    let mut current_size = app.current_size();

    app.run(|event: Event| {
        match event {
            Event::Resize(width, height) => {
                current_size = (width, height);
            },
            Event::Input(input) => {
                if input.key == KeyCode::Space && input.state == KeyState::Pressed {
                    direction *= -1.0;
                }
            },
            Event::Update => {
                transform.rotate(0.01 * direction);
            },
            Event::Render(renderer) => {
                let canvas = renderer.canvas();
                let mut ctx = renderer.draw_ctx();

                {
                    let mut render_pass = ctx.render_pass(&[&canvas], renderer.depth_texture());

                    render_pass.draw(&renderer, DrawDescriptor {
                        drawable: Some(triangle),
                        instance_data: Some(transform), 
                        pipeline: &pipeline,
                        shader_resources: &[&shader_resource],
                    });
                }

                ctx.apply(canvas, &renderer);
            },
            Event::DrawUi(ctx) => {
                ctx.label(0, 0, format!("Current size: {}x{}", current_size.width, current_size.height));
            },
        }
    });
}