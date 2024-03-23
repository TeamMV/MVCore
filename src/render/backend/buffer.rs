use std::ffi::CString;
use std::sync::Arc;
use bitflags::bitflags;
use mvcore_proc_macro::graphics_item;
use crate::render::backend::command_buffer::CommandBuffer;
use crate::render::backend::device::Device;
use crate::render::backend::vulkan::buffer::VkBuffer;

pub(crate) struct MVBufferCreateInfo {
    pub(crate) instance_size: u64,
    pub(crate) instance_count: u32,
    pub(crate) usage: BufferUsage,
    pub(crate) memory_properties: MemoryProperties,
    pub(crate) minimum_alignment: u64,
    pub(crate) no_pool: bool,

    pub(crate) label: Option<String>
}

#[graphics_item(ref)]
pub(crate) enum Buffer {
    Vulkan(VkBuffer),
    #[cfg(target_os = "macos")]
    Metal,
    #[cfg(target_os = "windows")]
    DirectX,
}

impl Buffer {
    pub(crate) fn new(device: Device, create_info: MVBufferCreateInfo) -> Self {
        match device {
            Device::Vulkan(device) => Buffer::Vulkan(VkBuffer::new(device, create_info.into()).into()),
            #[cfg(target_os = "macos")]
            Device::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            Device::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn write(&mut self, data: &[u8], offset: u64, command_buffer: Option<CommandBuffer>) {
        match self {
            Buffer::Vulkan(buffer) => buffer.write_to_buffer(data, offset, command_buffer.map(CommandBuffer::into_vulkan)),
            #[cfg(target_os = "macos")]
            Buffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            Buffer::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn flush(&self, size: u64, offset: u64) {
        match self {
            Buffer::Vulkan(buffer) => buffer.flush(size, offset)),
            #[cfg(target_os = "macos")]
            Buffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            Buffer::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn get_descriptor_info(&self, size: u64, offset: u64) -> DescriptorBufferInfo {
        match self {
            Buffer::Vulkan(buffer) => DescriptorBufferInfo::Vulkan(buffer.get_descriptor_info(size, offset)),
            #[cfg(target_os = "macos")]
            Buffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            Buffer::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn copy_buffer(src: &Buffer, dst: &Buffer, size: u64, command_buffer: Option<CommandBuffer>) {
        match (src, dst) {
            (Buffer::Vulkan(src), Buffer::Vulkan(dst)) => VkBuffer::copy_buffer(src, dst, size, command_buffer.map(CommandBuffer::into_vulkan)),
            #[cfg(target_os = "macos")]
            (Buffer::Metal, Buffer::Metal) => unimplemented!(),
            #[cfg(target_os = "windows")]
            (Buffer::DirectX, Buffer::DirectX) => unimplemented!(),
            (_, _) => unreachable!(),
        }
    }

    pub(crate) fn map(&mut self, size: u64, offset: u64) {
        match self {
            Buffer::Vulkan(buffer) => buffer.map(size, offset),
            #[cfg(target_os = "macos")]
            Buffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            Buffer::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn unmap(&mut self) {
        match self {
            Buffer::Vulkan(buffer) => buffer.unmap(),
            #[cfg(target_os = "macos")]
            Buffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            Buffer::DirectX => unimplemented!(),
        }
    }
}

pub(crate) enum DescriptorBufferInfo {
    Vulkan(ash::vk::DescriptorBufferInfo),
    #[cfg(target_os = "macos")]
    Metal,
    #[cfg(target_os = "windows")]
    DirectX,
}

bitflags! {
    pub struct BufferUsage: u32 {
        const TRANSFER_SRC = 1;
        const TRANSFER_DST = 1 << 1;
        const UNIFORM_TEXEL_BUFFER = 1 << 2;
        const STORAGE_TEXEL_BUFFER = 1 << 3;
        const UNIFORM_BUFFER = 1 << 4;
        const STORAGE_BUFFER = 1 << 5;
        const INDEX_BUFFER = 1 << 6;
        const VERTEX_BUFFER = 1 << 7;
        const INDIRECT_BUFFER = 1 << 8;
        #[cfg(feature = "ray-tracing")]
        const SHADER_BINDING_TABLE_KHR = 1 << 10;
        const SHADER_DEVICE_ADDRESS = 1 << 17;
        #[cfg(feature = "ray-tracing")]
        const VACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR = 1 << 19;
        #[cfg(feature = "ray-tracing")]
        const ACCELERATION_STRUCTURE_STORAGE_KHR = 1 << 20;
    }
}

bitflags! {
    pub struct MemoryProperties: u8 {
        const DEVICE_LOCAL = 1;
        const HOST_VISIBLE = 1 << 1;
        const HOST_COHERENT = 1 << 2;
        const HOST_CACHED = 1 << 3;
        const LAZILY_ALLOCATED = 1 << 4;
        const PROTECTED = 1 << 5;
    }
}