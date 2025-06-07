#[macro_export]
macro_rules! include_wgsl {
    ($token:tt) => {
        {
            #[$crate::render::include_wgsl_raw($token)]
            #[allow(non_snake_case)]
            mod shader {}

            $crate::render::hal::shader::Shader::Wgsl($crate::render::types::ShaderModuleDescriptor {
                label: Some($token),
                source: $crate::render::types::ShaderSource::Wgsl(shader::SOURCE.into()),
            })
        }
    };
}

#[macro_export]
macro_rules! include_spirv {
    ($token:expr) => {
        $crate::render::hal::pipeline::Shader::SpirV($crate::render::include_spirv_raw!($token))
    };
}

pub use include_wgsl;
pub use include_spirv;