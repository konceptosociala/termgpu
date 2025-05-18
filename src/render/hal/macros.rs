#[macro_export]
macro_rules! include_wgsl {
    ($token:tt) => {
        {
            #[$crate::renderer::include_wgsl_raw($token)]
            #[allow(non_snake_case)]
            mod shader {}

            $crate::renderer::hal::pipeline::Shader::Wgsl($crate::renderer::types::ShaderModuleDescriptor {
                label: Some($token),
                source: $crate::renderer::types::ShaderSource::Wgsl(shader::SOURCE.into()),
            })
        }
    };
}

#[macro_export]
macro_rules! include_spirv {
    ($token:expr) => {
        $crate::renderer::hal::pipeline::Shader::SpirV($crate::renderer::include_spirv_raw!($token))
    };
}

pub use include_wgsl;
pub use include_spirv;