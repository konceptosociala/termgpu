use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    /// Triggered when the swap chain is lost and needs to be recreated.
    #[error("The swap chain has been lost and needs to be recreated")]
    Lost,
    ///Triggered when there is no memory left for a new frame.
    #[error("There is no more memory left to allocate a new frame")]
    OutOfMemory,
    /// Contains a string description of a surface error.
    #[error("Error acquiring current texture")]
    SurfaceError(String),
    ///Contains a list of missing or improperly set-up components.
    #[error("Render components are not set up: {0:?}")]
    SetupError(Vec<&'static str>),
    /// Triggered when a buffer exceeds its capacity.
    #[error("Buffer with capacity `{0}` is overflowed")]
    BufferOverflow(usize),
}

impl From<wgpu::SurfaceError> for RenderError {
    fn from(value: wgpu::SurfaceError) -> Self {
        match value {
            wgpu::SurfaceError::Lost => RenderError::Lost,
            wgpu::SurfaceError::OutOfMemory => RenderError::OutOfMemory,
            _ => RenderError::SurfaceError(value.to_string()),
        }
    }
}