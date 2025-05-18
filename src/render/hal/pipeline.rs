use crate::render::{vertex::Vertex, Renderer};
#[cfg(doc)]
use crate::renderer::RenderPass;

use super::{resource::ShaderResource, shader::Shader};

/// Represents a graphics or compute pipeline. Used to describe rendering
/// process in a [`RenderPass`]
pub enum Pipeline {
    /// A render pipeline.
    Render(wgpu::RenderPipeline),
    /// A compute pipeline.
    Compute(wgpu::ComputePipeline),
}

/// Descriptor for creating a render pipeline.
pub struct RenderPipelineDescriptor<'a> {
    /// The shader used in the pipeline.
    pub shader: Shader,
    /// The shader resources (buffers, textures) used in the pipeline.
    pub bindings: &'a [&'a ShaderResource],
    /// The label for the pipeline. Displayed, when any error connected with
    /// the pipeline occures
    pub label: &'a str,
    /// Indicates whether the pipeline uses vertex buffers.
    pub use_vertices: bool,
    /// The surface formats used in the pipeline. Count and formats must
    /// match ones in render pass
    pub surface_formats: &'a [wgpu::TextureFormat],
}

/// Descriptor for creating a compute pipeline.
pub struct ComputePipelineDescriptor<'a> {
    /// The shader used in the pipeline.
    pub shader: Shader,
    /// The shader resources (buffers, textures) used in the pipeline.
    pub bindings: &'a [&'a ShaderResource],
    /// The label for the pipeline. Displayed, when any error connected with
    /// the pipeline occures
    pub label: &'a str,
}

impl Pipeline {
    /// Creates a new rendering pipeline using the provided descriptor.
    pub fn new_render(
        renderer: &Renderer,
        descriptor: &RenderPipelineDescriptor<'_>,
    ) -> Pipeline {
        let shader = match descriptor.shader {
            Shader::Wgsl(ref wgsl) => renderer.device.create_shader_module(wgsl.clone()),
            Shader::SpirV(ref spirv) => unsafe {
                renderer.device.create_shader_module_spirv(spirv)
            },
        };

        let layout = renderer.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(format!("{} Render Pipeline Layout", descriptor.label).as_str()),
            bind_group_layouts: &descriptor.bindings
                .to_vec()
                .iter()
                .map(|b| &b.bind_group_layout)
                .collect::<Vec<_>>(),
            push_constant_ranges: &[wgpu::PushConstantRange {
                stages: wgpu::ShaderStages::VERTEX_FRAGMENT,
                range: 0..128,
            }],
        });

        let buffers = if descriptor.use_vertices {
            vec![Vertex::vertex_buffer_layout()]
        } else {
            vec![]
        };

        let pipeline = renderer.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(format!("{} Render Pipeline", descriptor.label).as_str()),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", 
                buffers: &buffers,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &descriptor.surface_formats
                    .iter()
                    .map(|format| Some(wgpu::ColorTargetState {
                        format: *format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    }))
                    .collect::<Vec<_>>(),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, 
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, 
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1, 
                mask: !0, 
                alpha_to_coverage_enabled: false, 
            },
            multiview: None, 
            cache: None,
        });

        Pipeline::Render(pipeline)
    }

    /// Creates a new compute pipeline using the provided descriptor.
    pub fn new_compute(
        renderer: &Renderer,
        descriptor: &ComputePipelineDescriptor<'_>,
    ) -> Pipeline {
        let shader = match descriptor.shader {
            Shader::Wgsl(ref wgsl) => renderer.device.create_shader_module(wgsl.clone()),
            Shader::SpirV(ref spirv) => unsafe {
                renderer.device.create_shader_module_spirv(spirv)
            },
        };

        let layout = renderer.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(format!("{} Compute Pipeline Layout", descriptor.label).as_str()),
            bind_group_layouts: &descriptor.bindings
                .to_vec()
                .iter()
                .map(|b| &b.bind_group_layout)
                .collect::<Vec<_>>(),
            push_constant_ranges: &[wgpu::PushConstantRange {
                stages: wgpu::ShaderStages::COMPUTE,
                range: 0..128,
            }],
        });

        let pipeline = renderer.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(format!("{} Compute Pipeline", descriptor.label).as_str()),
            layout: Some(&layout),
            module: &shader,
            entry_point: "cs_main",
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

        Pipeline::Compute(pipeline)
    }
}