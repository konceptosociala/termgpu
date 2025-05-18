use bytemuck::Pod;

use crate::render::Renderer;

use super::{buffer::{Buffer, BufferResourceDescriptor}, texture::{Texture, TextureResourceDescriptor, TextureResourceUsage}};

pub struct ShaderResourceBuilder<'a> {
    label: Option<String>,
    bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry>,
    bind_group_entries: Vec<wgpu::BindGroupEntry<'a>>,
}

impl<'a> ShaderResourceBuilder<'a> {
    /// Set a label for the shader resource. It is displayed
    /// when some error occures
    pub fn set_label(&mut self, label: impl Into<String>) -> &mut Self {
        self.label = Some(label.into());
        self
    }

    /// Add a buffer to the shader resource with the given descriptor.
    pub fn add_buffer<T: Pod>(
        &mut self,
        buffer: &'a Buffer<T>,
        descriptor: &BufferResourceDescriptor,
    ) -> &mut Self {
        self.bind_group_layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding: self.bind_group_layout_entries.len() as u32,
            visibility: descriptor.visibility,
            ty: wgpu::BindingType::Buffer {
                ty: descriptor.buffer_type,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });

        self.bind_group_entries.push(wgpu::BindGroupEntry {
            binding: self.bind_group_entries.len() as u32,
            resource: buffer.inner().as_entire_binding(),
        });

        self
    }

    /// Add a texture to the shader resource with the given descriptor.
    pub fn add_texture(
        &mut self,
        texture: &'a Texture,
        descriptor: &TextureResourceDescriptor,
    ) -> &mut Self {
        let view_dimension = match texture.descriptor().dimension {
            wgpu::TextureDimension::D1 => wgpu::TextureViewDimension::D1,
            wgpu::TextureDimension::D2 => wgpu::TextureViewDimension::D2,
            wgpu::TextureDimension::D3 => wgpu::TextureViewDimension::D3,
        };

        let bind_group_layout_entries = descriptor.usage
            .iter()
            .enumerate()
            .filter_map(|(i, usage)| {
                match usage {
                    TextureResourceUsage::TEXTURE => {
                        Some(wgpu::BindGroupLayoutEntry {
                            binding: (self.bind_group_layout_entries.len() + i) as u32,
                            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Texture {
                                sample_type: descriptor.sample_type.unwrap_or_else(|| {
                                    panic!("Must specify sample type for texture with TextureResourceUsage::TEXTURE");
                                }),
                                view_dimension,
                                multisampled: false,
                            },
                            count: None,
                        })
                    },
                    TextureResourceUsage::SAMPLER => {
                        Some(wgpu::BindGroupLayoutEntry {
                            binding: (self.bind_group_layout_entries.len() + i) as u32,
                            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Sampler(
                                descriptor.sampler_binding_type
                                    .expect("Must specify sampler binding type for TextureResourceUsage::SAMPLER")
                            ),
                            count: None,
                        })
                    },
                    TextureResourceUsage::STORAGE => {
                        Some(wgpu::BindGroupLayoutEntry {
                            binding: (self.bind_group_layout_entries.len() + i) as u32,
                            visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::WriteOnly,
                                format: texture.descriptor().format,
                                view_dimension,
                            },
                            count: None,
                        })
                    },
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        let bind_group_entries = descriptor.usage
            .iter()
            .enumerate()
            .filter_map(|(i, usage)| {
                match usage {
                    TextureResourceUsage::STORAGE | TextureResourceUsage::TEXTURE => {
                        Some(wgpu::BindGroupEntry {
                            binding: (self.bind_group_entries.len() + i) as u32,
                            resource: wgpu::BindingResource::TextureView(texture.view())
                        },)
                    },
                    TextureResourceUsage::SAMPLER => {
                        Some(wgpu::BindGroupEntry {
                            binding: (self.bind_group_entries.len() + i) as u32,
                            resource: wgpu::BindingResource::Sampler(texture.sampler()),
                        })
                    },
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        self.bind_group_layout_entries.extend(bind_group_layout_entries);
        self.bind_group_entries.extend(bind_group_entries);

        self
    }

    /// Build the shader resource from added bindings
    pub fn build(&self, renderer: &Renderer) -> ShaderResource {
        let bind_group_layout = renderer.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: self.label
                .as_ref()
                .map(|label| format!("{label} Bind Group Layout"))
                .as_deref(),
            entries: &self.bind_group_layout_entries,
        });

        let bind_group = renderer.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: self.label
                .as_ref()
                .map(|label| format!("{label} Bind Group"))
                .as_deref(),
            layout: &bind_group_layout,
            entries: &self.bind_group_entries,
        });

        ShaderResource { bind_group_layout, bind_group }
    }
}

/// Represents a shader resource that includes bind group layouts 
/// and bind groups. It is used in render and compute [`Pipeline`]s
/// to push any data to shaders
#[derive(Debug)]
pub struct ShaderResource {
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) bind_group: wgpu::BindGroup,
}

impl ShaderResource {
    /// Initialize shader resource builder
    pub fn builder<'a>() -> ShaderResourceBuilder<'a> {
        ShaderResourceBuilder {
            bind_group_layout_entries: vec![],
            bind_group_entries: vec![],
            label: None,
        }
    }
}