use bytemuck::{Zeroable, Pod};

pub mod buffer;
pub mod macros;
pub mod pipeline;
pub mod resource;
pub mod shader;
pub mod texture;

/// A structure used for padding to align data to specific byte 
/// boundaries for convenience and safety in GPU memory operations.
#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq, Copy, Zeroable, Pod)]
pub struct Padding {
    _padding: u32,
}