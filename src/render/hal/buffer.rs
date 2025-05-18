//! Buffer module contains types related to buffers and buffer
//! resources, which transfer data from CPU to shaders on GPU

use std::marker::PhantomData;
use std::mem::{size_of, size_of_val};

use bytemuck::Pod;
use derive_getters::Getters;
use pretty_type_name::pretty_type_name;
use thiserror::Error;

use crate::render::error::RenderError;
use crate::render::Renderer;
use crate::render::types::*;

/// Buffer identification number, given by renderer when
/// creating vertex buffer
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BufferId(pub(crate) usize);

/// Error getting vertex buffer instance from renderer
/// using invalid [`BufferId`]
#[derive(Debug, Error)]
#[error("Invalid buffer id {}", self.0.0)]
pub struct InvalidBufferId(pub BufferId);

/// A generic resizable buffer used for storing data on the GPU.
/// 
/// * `T` - The type of data stored in the buffer. Must implement the `Pod` trait.
#[derive(Debug, Getters)]
pub struct Buffer<T> {
    inner: wgpu::Buffer,
    capacity: usize,
    #[getter(skip)]
    _phantom_data: PhantomData<T>,
}

impl<T: Pod> Buffer<T> {
    /// Creates a new buffer with the given capacity and usage. 
    /// 
    /// The capacity of the buffer is the number of elements of type `T`.
    pub fn new(renderer: &Renderer, capacity: usize, usage: BufferUsages) -> Buffer<T> {
        Buffer {
            inner: Buffer::<T>::new_inner(&renderer.device, capacity * size_of::<T>(), usage),
            capacity,
            _phantom_data: PhantomData,
        }
    }

    /// Fills the buffer with the given data, ensuring the data 
    /// fits within the buffer capacity. 
    /// 
    /// Offset is the number of elements of type `T`, not bytes. 
    /// 
    /// Returns `BufferOverflow` error, if the data length exceeds 
    /// the buffer capacity.
    pub fn fill_exact(
        &self, 
        renderer: &Renderer, 
        offset: u64,
        data: &[T],
    ) -> Result<(), RenderError> {
        if data.len() > self.capacity {
            return Err(RenderError::BufferOverflow(data.len()));
        }

        if !data.is_empty() {
            renderer.queue.write_buffer(&self.inner, offset * size_of::<T>() as u64, bytemuck::cast_slice(data));
        }

        Ok(())
    }

    /// Fills the buffer with the given data, resizing the buffer if necessary.
    ///
    /// Offset is the number of elements of type `T`, not bytes. 
    pub fn fill(
        &mut self, 
        renderer: &Renderer, 
        offset: u64,
        data: &[T],
    ) {
        let bytes_to_write = size_of_val(data);
        if bytes_to_write > self.capacity * size_of::<T>() {
            self.resize(renderer, data.len());
        }

        self.fill_exact(renderer, offset, data).unwrap();
    }

    /// Resize the buffer to the given capacity, which
    /// is the number of elements of type `T`
    pub fn resize(&mut self, renderer: &Renderer, capacity: usize) {
        self.inner = Buffer::<T>::new_inner(&renderer.device, capacity * size_of::<T>(), self.inner.usage());
        self.capacity = capacity;
    }

    fn new_inner(device: &wgpu::Device, capacity: usize, usage: wgpu::BufferUsages) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(format!("Buffer ({:?}, {})", usage, pretty_type_name::<T>()).as_str()),
            size: capacity as u64,
            usage: usage | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }
}

#[cfg(doc)]
use crate::renderer::hal::pipeline::ShaderResource;

/// Used to bind generic buffer in [`ShaderResource`]
pub struct BufferResourceDescriptor {
    /// Indicates in which shader stages (Vertex, Fragment and/or Compute)
    /// current buffer is visible
    pub visibility: ShaderStages,
    /// Indicates buffer binding type:
    /// * uniform - read only, faster, small amount of data
    /// * storage - read/write, slower, bigger amount of data
    pub buffer_type: BufferBindingType,
}