use bitflags::bitflags;
use derive_getters::Getters;

use crate::render::{RenderSurface, Renderer};
use crate::render::types::*;

/// Describes a texture, including its size, format, usage, and filtering mode.
#[derive(Debug, Clone)]
pub struct TextureDescriptor {
    /// Width of the texture in pixels.
    pub width: u32,
    /// Height of the texture in pixels.
    pub height: u32,
    /// Optional depth value (for 3D textures).
    pub depth: Option<u32>,
    /// Filtering mode used for sampling the texture.
    pub filter: FilterMode,
    /// Dimensionality of the texture (1D, 2D, or 3D).
    pub dimension: TextureDimension,
    /// Usage flags specifying how the texture will be used.
    pub usage: TextureUsages,
    /// Format of the texture
    pub format: TextureFormat,
    /// Number of mip levels for the texture.
    pub mip_level_count: u32,
    /// A human-readable label for debugging purposes. Displayed, when
    /// error affiliated with the texture occures
    pub label: String,
}

bitflags! {
    /// Flags indicating how a texture resource will be used in shaders.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TextureResourceUsage: u8 {
        /// The texture will be used as a sampled texture.
        const TEXTURE = 1;
        /// The texture will be used with a sampler.
        const SAMPLER = 1 << 1;
        /// The texture will be used as a storage texture.
        const STORAGE = 1 << 2;
    }
}

/// Describes a texture resource, defining how it will be accessed in a shader.
pub struct TextureResourceDescriptor {
    /// The intended usage of the texture resource.
    pub usage: TextureResourceUsage,
    /// The expected sample type when used as a sampled texture.
    pub sample_type: Option<TextureSampleType>,
    /// The type of sampler binding when used as a sampler.
    pub sampler_binding_type: Option<SamplerBindingType>,
}

/// A structure representing a GPU texture, including its view and sampler.
#[derive(Debug, Getters)]
pub struct Texture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    descriptor: TextureDescriptor,
}

impl RenderSurface for Texture {
    fn view(&self) -> &TextureView {
        if !self.descriptor.usage.contains(wgpu::TextureUsages::RENDER_ATTACHMENT) {
            panic!("Texture, used as render surface, must have RENDER_ATTACHMENT usage");
        }

        &self.view
    }
}

impl Texture {
    /// Creates a new texture with the specified descriptor.
    pub fn new(
        renderer: &Renderer, 
        descriptor: TextureDescriptor,
    ) -> Texture {
        let size = wgpu::Extent3d {
            width: descriptor.width,
            height: descriptor.height,
            depth_or_array_layers: descriptor.depth.unwrap_or(1),
        };

        let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(format!("{} Texture", descriptor.label).as_str()),
            size,
            mip_level_count: descriptor.mip_level_count,
            sample_count: 1,
            dimension: descriptor.dimension,
            format: descriptor.format,
            usage: descriptor.usage,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = renderer.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some(format!("{} Texture Sampler", descriptor.label).as_str()),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: descriptor.filter,
            min_filter: descriptor.filter,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Texture { 
            texture, 
            view, 
            sampler,
            descriptor,
        }
    }

    /// Resizes the texture to match a new surface size.
    pub fn resize(&mut self, renderer: &Renderer, width: u32, height: u32) {
        let mut descr = self.descriptor.clone();
        descr.width = width;
        descr.height = height;
        *self = Texture::new(renderer, descr);
    }
}
