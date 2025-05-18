pub enum Shader {
    /// A WGSL shader module descriptor.
    Wgsl(wgpu::ShaderModuleDescriptor<'static>),
    /// A SPIR-V shader module descriptor.
    SpirV(wgpu::ShaderModuleDescriptorSpirV<'static>),
}