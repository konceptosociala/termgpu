use std::time::Duration;
use termgpu::prelude::*;

fn main() {
    let mut app = TermApp::new(Duration::from_millis(33));

    let mut triangle = Triangle::default();
    triangle.update(app.renderer_mut());

    let mut transform = Transform::identity();
    let mut direction = -1.0;
    let mut current_size = app.size();

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
                current_size = size;

                log::info!("Resized!");
            },
            Event::Input(input) => {
                if input.kind == KeyEventKind::Press {
                    match input.code {
                        KeyCode::Esc => exit(),
                        KeyCode::Char(' ') => {
                            direction *= -1.0;
                        },
                        _ => {}
                        
                    }
                }
            },
            Event::Update => {
                transform.rotation *= glm::quat_angle_axis(
                    0.005 * direction, 
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
                // ctx.label(0, 0, format!("Current size: {}x{}", current_size.width, current_size.height));
            },
        }
    });
}