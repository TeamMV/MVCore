use crate::render::backend::device::Device;
use crate::render::backend::vulkan::shader::VkShader;
use bitflags::bitflags;
use mvcore_proc_macro::graphics_item;
use std::sync::Arc;

pub struct MVShaderCreateInfo {
    pub stage: ShaderStage,
    pub code: Vec<u32>,

    pub label: Option<String>,
}

bitflags! {
     pub struct ShaderStage: u32 {
        const Vertex = 1;
        const TesselationControl = 1 << 1;
        const TesselationEvaluation = 1 << 2;
        const Geometry = 1 << 3;
        const Fragment = 1 << 4;
        const Compute = 1 << 5;
        #[cfg(feature = "ray-tracing")]
        const RayGen = 1 << 8;
        #[cfg(feature = "ray-tracing")]
        const AnyHit = 1 << 9;
        #[cfg(feature = "ray-tracing")]
        const ClosestHit = 1 << 10;
        #[cfg(feature = "ray-tracing")]
        const Miss = 1 << 11;
        #[cfg(feature = "ray-tracing")]
        const Intersection = 1 << 12;
        #[cfg(feature = "ray-tracing")]
        const Callable = 1 << 13;
    }
}

#[graphics_item(clone)]
#[derive(Clone)]
pub enum Shader {
    Vulkan(Arc<VkShader>),
    #[cfg(target_os = "macos")]
    Metal,
    #[cfg(target_os = "windows")]
    DirectX,
}

impl Shader {
    pub fn new(device: Device, create_info: MVShaderCreateInfo) -> Shader {
        match device {
            Device::Vulkan(device) => {
                Shader::Vulkan(VkShader::new(device, create_info.into()).into())
            }
            #[cfg(target_os = "macos")]
            Device::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            Device::DirectX => unimplemented!(),
        }
    }
}
