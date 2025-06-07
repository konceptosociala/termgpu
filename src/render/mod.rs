use std::mem::size_of;
use bytemuck::Pod;
use hal::{
    buffer::*, pipeline::*, resource::ShaderResource, texture::*
};
use serde::{Deserialize, Serialize};
use vertex::Vertex;
use nalgebra_glm as glm;

pub mod error;
pub mod hal;
pub mod vertex;

pub mod prelude {
    pub use super::{
        Renderer,
        DrawContext,
        ComputePass,
        RenderPass,
        DrawDescriptor,
        ComputeDescriptor,
        RenderSurface,
        Canvas,
        Drawable,
        InstanceData,
        TransformationType,
        Transformation,
    };
    pub use super::types::*;
    pub use super::vertex::*;
    pub use super::hal::{
        buffer::*,
        pipeline::*,
        resource::*,
        texture::*,
        shader::*,
        Padding,
    };
    pub use super::error::RenderError;
}

pub mod types {
    pub use wgpu::{
        BufferUsages,
        ShaderStages,
        BufferBindingType,
        FilterMode,
        TextureDimension,
        TextureUsages,
        TextureFormat,
        TextureSampleType,
        Extent3d,
        ShaderSource,
        TextureView,
        SamplerBindingType,
        ShaderModuleDescriptor,
        ShaderModuleDescriptorSpirV,
    };
}

pub use include_wgsl_oil::include_wgsl_oil as include_wgsl_raw;
pub use wgpu::include_spirv_raw;

use crate::{fatal, utils::Size};

pub struct Renderer {
    width: u32,
    height: u32,
    device: wgpu::Device,
    queue: wgpu::Queue,
    vertex_buffers: Vec<Buffer<Vertex>>,
    surface_texture: Option<Texture>,
    depth_texture: Option<Texture>,
}

impl Renderer {
    pub fn new(size: Size) -> anyhow::Result<Renderer> {
        let instance = Self::init_instance();
        let adapter = Self::init_adapter(instance);
        let (device, queue) = Self::init_device(&adapter)?;

        let Size::Renderer(width, height) = size.to_renderer() else { unreachable!() };

        let mut renderer = Renderer {
            width,
            height,
            device,
            queue,
            vertex_buffers: vec![],
            surface_texture: None,
            depth_texture: None,
        };

        renderer.surface_texture = Some(Texture::new(
            &renderer,
            TextureDescriptor {
                width: renderer.width,
                height: renderer.height,
                filter: wgpu::FilterMode::Linear,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT 
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC,
                depth: None,
                mip_level_count: 1,
                label: "Surface texture".to_string(),
            }
        ));

        renderer.depth_texture = Some(Texture::new(
            &renderer, 
            TextureDescriptor {
                width: renderer.width,
                height: renderer.height,
                filter: wgpu::FilterMode::Linear,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                depth: None,
                mip_level_count: 1,
                label: "Depth texture".to_string(),
            },
        ));

        Ok(renderer)
    }

    pub fn canvas(&self) -> Canvas<'_> {
        Canvas { 
            texture: self.surface_texture.as_ref().unwrap() 
        }
    }

    pub fn depth_texture(&self) -> &Texture {
        self.depth_texture.as_ref().unwrap()
    }

    pub fn draw_ctx(&self) -> DrawContext {
        DrawContext {
            encoder: self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default()),
        }
    }

    pub fn resize(&mut self) {
        self.resize_with(Size::Renderer(self.width, self.height));
    }

    pub fn resize_with(&mut self, size: Size) {
        let Size::Renderer(width, height) = size.to_renderer() else { unreachable!() };

        if width == 0 || height == 0 { return }

        self.width = width;
        self.height = height;

        if let Some(surface_texture) = &self.surface_texture {
            let mut surface_descr = surface_texture.descriptor().clone();
            surface_descr.width = self.width;
            surface_descr.height = self.height;
            self.surface_texture = Some(Texture::new(self, surface_descr));
        }

        if let Some(depth_texture) = &self.depth_texture {
            let mut depth_descr = depth_texture.descriptor().clone();
            depth_descr.width = self.width;
            depth_descr.height = self.height;
            self.depth_texture = Some(Texture::new(self, depth_descr));
        }
    }

    pub fn create_vertex_buffer(&mut self, capacity: usize) -> BufferId {
        let id = self.vertex_buffers.len();

        self.vertex_buffers.push(Buffer::new(
            self,
            capacity,
            wgpu::BufferUsages::VERTEX,
        ));

        BufferId(id)
    }

    pub fn update_vertex_buffer(&mut self, id: BufferId, data: &[Vertex]) -> Result<(), InvalidBufferId> {
        if self.vertex_buffers
            .get(id.0)
            .ok_or(InvalidBufferId(id))?
            .fill_exact(self, 0, data).is_err() 
            {
                let mut buffer = self.vertex_buffers.swap_remove(id.0);
                buffer.fill(self, 0, data);
                self.vertex_buffers.insert(id.0, buffer);
            }

        Ok(())
    }

    pub fn size(&self) -> Size {
        Size::Renderer(self.width, self.height)
    }

    fn init_device(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue), wgpu::RequestDeviceError> {
        let mut required_limits = wgpu::Limits::downlevel_defaults();
        required_limits.max_texture_dimension_2d = 4096;
        required_limits.max_texture_dimension_3d = 2048;
        required_limits.max_push_constant_size = 128;
        
        pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::default()
                    | wgpu::Features::PUSH_CONSTANTS
                    | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES
                    | wgpu::Features::SPIRV_SHADER_PASSTHROUGH,
                required_limits,
                label: Some("Logical device"),
                memory_hints: Default::default(),
            },
            None,
        ))
    }

    fn init_adapter(instance: wgpu::Instance) -> wgpu::Adapter {
        pollster::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            }
        )).unwrap()
    }

    fn init_instance() -> wgpu::Instance {
        wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            flags: wgpu::InstanceFlags::from_build_config(),
            ..Default::default()
        })
    }
}

pub struct DrawContext {
    encoder: wgpu::CommandEncoder,
}

impl DrawContext {
    pub fn render_pass<'a>(
        &'a mut self,
        canvases: &'a [&'a dyn RenderSurface],
        depth_texture: &'a Texture,
    ) -> RenderPass<'a> {
        let pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &canvases
                .iter()
                .map(|canvas| {
                    Some(wgpu::RenderPassColorAttachment {
                        view: canvas.view(),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })
                })
                .collect::<Vec<_>>(),
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_texture.view(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        RenderPass { pass }
    }

    pub fn compute_pass(&mut self) -> ComputePass<'_> {
        let pass = self.encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute pass"),
            timestamp_writes: None,
        });

        ComputePass { pass }
    }

    pub fn clear_buffer<T>(&mut self, buffer: &Buffer<T>) {
        self.encoder.clear_buffer(buffer.inner(), 0, None);
    }

    pub fn copy_buffer<T>(
        &mut self,
        from: &Buffer<T>,
        from_offset: u64,
        to: &Buffer<T>,
        to_offset: u64,
        copy_size: u64,
    ) {
        self.encoder.copy_buffer_to_buffer(
            from.inner(), 
            from_offset * size_of::<T>() as u64, 
            to.inner(), 
            to_offset * size_of::<T>() as u64, 
            copy_size * size_of::<T>() as u64,
        );
    }

    pub fn copy_texture(&mut self, from: &Texture, to: &Texture) {
        self.encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: from.texture(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture {
                texture: to.texture(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: to.descriptor().width,
                height: to.descriptor().height,
                depth_or_array_layers: 1,
            }
        );
    }

    pub fn copy_texture_to_buffer<T>(
        &mut self,
        from: &Texture,
        to: &Buffer<T>,
    ) {
        let unpadded_bytes_per_row = from.descriptor().width * 4;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT; // 256
        let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;

        self.encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: from.texture(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: to.inner(),
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: None,
                },
            },
            wgpu::Extent3d {
                width: from.descriptor().width,
                height: from.descriptor().height,
                depth_or_array_layers: 1,
            },
        );
    }

    pub fn apply(self, _canvas: Canvas<'_>, renderer: &Renderer) {        
        renderer.queue.submit(std::iter::once(self.encoder.finish()));
    }
}

pub struct ComputePass<'a> {
    pass: wgpu::ComputePass<'a>,
}

pub struct ComputeDescriptor<'a, 'b, T> {
    pub instance_data: Option<&'b dyn InstanceData<UniformData = T>>,
    pub pipeline: &'a Pipeline,
    pub shader_resources: &'b [&'a ShaderResource],
    pub workgroups: (u32, u32, u32),
}

impl<'a> ComputePass<'a> {
    pub fn compute<T: Pod>(&mut self, descriptor: ComputeDescriptor<'a, '_, T>) {
        if let Pipeline::Compute(p) = descriptor.pipeline {
            self.pass.set_pipeline(p);
        } else {
            fatal!("Cannot use render pipeline in compute() command");
        }

        for (i, binding) in descriptor.shader_resources.iter().enumerate() {
            self.pass.set_bind_group(i as u32, &binding.bind_group, &[]);
        }

        if let Some(instance_data) = descriptor.instance_data {
            self.pass.set_push_constants(
                0,
                bytemuck::cast_slice(&[instance_data.uniform_data()]),
            );
        }

        self.pass.dispatch_workgroups(
            descriptor.workgroups.0, 
            descriptor.workgroups.1, 
            descriptor.workgroups.2,
        );
    }
}

pub struct RenderPass<'a> {
    pass: wgpu::RenderPass<'a>
}

pub struct DrawDescriptor<'a, 'b, T> {
    pub drawable: Option<&'b dyn Drawable>,
    pub instance_data: Option<&'b dyn InstanceData<UniformData = T>>,
    pub pipeline: &'a Pipeline,
    pub shader_resources: &'b [&'a ShaderResource],
}

impl<'a> RenderPass<'a> {
    pub fn draw<T: Pod>(&mut self, renderer: &'a Renderer, descriptor: DrawDescriptor<'a, '_, T>) {
        if let Pipeline::Render(p) = descriptor.pipeline {
            self.pass.set_pipeline(p);
        } else {
            fatal!("Cannot use compute pipeline in draw() command");
        }

        for (i, binding) in descriptor.shader_resources.iter().enumerate() {
            self.pass.set_bind_group(i as u32, &binding.bind_group, &[]);
        }

        if let Some(instance_data) = descriptor.instance_data {
            self.pass.set_push_constants(
                wgpu::ShaderStages::VERTEX_FRAGMENT,
                0,
                bytemuck::cast_slice(&[instance_data.uniform_data()]),
            );
        }
        
        if let Some(drawable) = descriptor.drawable {
            self.pass.set_vertex_buffer(0, renderer.vertex_buffers[drawable.vertex_buffer().0].inner().slice(..)); 
            self.pass.draw(0..*renderer.vertex_buffers[drawable.vertex_buffer().0].capacity() as u32, 0..1);
        } else {
            self.pass.draw(0..6, 0..1);
        }
    }
}

pub trait RenderSurface {
    fn view(&self) -> &types::TextureView;
}

pub struct Canvas<'canvas> {
    texture: &'canvas Texture,
}

impl Canvas<'_> {
    pub fn texture(&self) -> &Texture {
        self.texture
    }
}

impl RenderSurface for Canvas<'_> {
    fn view(&self) -> &wgpu::TextureView {
        self.texture.view()
    }
}

pub trait Drawable {
    fn update(&mut self, renderer: &mut Renderer);

    fn vertex_buffer(&self) -> BufferId;
}

pub trait InstanceData {
    type UniformData: Pod;

    fn uniform_data(&self) -> Self::UniformData;
}

impl<I: Pod> InstanceData for I {
    type UniformData = I;

    fn uniform_data(&self) -> Self::UniformData {
        *self
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug, Hash, PartialEq)]
pub enum TransformationType {
    #[default]
    FirstPerson,
    LookAt,
}


pub trait Transformation {
    fn from_transform(
        transformation_type: TransformationType,
        translation: glm::Vec3, 
        rotation: glm::Quat, 
        pivot: glm::Vec3,
    ) -> Self;
}