use crate::render::backend::buffer::Buffer;
use crate::render::backend::device::{CommandPool, Device};
use crate::render::backend::vulkan::buffer::VkBuffer;
use crate::render::backend::vulkan::command_buffer::VkCommandBuffer;
use mvcore_proc_macro::graphics_item;

pub(crate) enum CommandBufferLevel {
    Primary,
    Secondary,
}

pub(crate) struct MVCommandBufferCreateInfo {
    pub(crate) level: CommandBufferLevel,
    pub(crate) pool: CommandPool,

    pub(crate) label: Option<String>,
}

#[graphics_item(ref)]
pub(crate) enum CommandBuffer {
    Vulkan(VkCommandBuffer),
    #[cfg(target_os = "macos")]
    Metal,
    #[cfg(target_os = "windows")]
    DirectX,
}

impl CommandBuffer {
    pub(crate) fn new(device: Device, create_info: MVCommandBufferCreateInfo) -> Self {
        match device {
            Device::Vulkan(device) => {
                CommandBuffer::Vulkan(VkCommandBuffer::new(device, create_info.into()))
            }
            #[cfg(target_os = "macos")]
            Device::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            Device::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn begin(&self) {
        match self {
            CommandBuffer::Vulkan(cmd) => cmd.begin(),
            #[cfg(target_os = "macos")]
            CommandBuffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            CommandBuffer::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn end(&self) {
        match self {
            CommandBuffer::Vulkan(cmd) => cmd.end(),
            #[cfg(target_os = "macos")]
            CommandBuffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            CommandBuffer::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn write_buffer(&self, buffer: &mut Buffer, data: &[u8], offset: u64) {
        match self {
            CommandBuffer::Vulkan(cmd) => {
                buffer
                    .as_vulkan_mut()
                    .write_to_buffer(data, offset, Some(cmd.get_handle()))
            }
            #[cfg(target_os = "macos")]
            CommandBuffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            CommandBuffer::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn copy_buffers(
        &self,
        src: &Buffer,
        dst: &mut Buffer,
        size: u64,
        src_offset: u64,
        dst_offset: u64,
    ) {
        match self {
            CommandBuffer::Vulkan(cmd) => VkBuffer::copy_buffer(
                src.as_vulkan(),
                dst.as_vulkan_mut(),
                size,
                src_offset,
                dst_offset,
                Some(cmd.get_handle()),
            ),
            #[cfg(target_os = "macos")]
            CommandBuffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            CommandBuffer::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn draw(&self, vertex_count: u32, first_vertex: u32) {
        match self {
            CommandBuffer::Vulkan(cmd) => cmd.draw(vertex_count, 1, first_vertex, 0),
            #[cfg(target_os = "macos")]
            CommandBuffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            CommandBuffer::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn draw_instanced(
        &self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        match self {
            CommandBuffer::Vulkan(cmd) => {
                cmd.draw(vertex_count, instance_count, first_vertex, first_instance)
            }
            #[cfg(target_os = "macos")]
            CommandBuffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            CommandBuffer::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn draw_indexed(&self, index_count: u32, first_index: u32) {
        match self {
            CommandBuffer::Vulkan(cmd) => cmd.draw_indexed(index_count, 1, first_index, 0),
            #[cfg(target_os = "macos")]
            CommandBuffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            CommandBuffer::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn draw_indexed_instanced(
        &self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        first_instance: u32,
    ) {
        match self {
            CommandBuffer::Vulkan(cmd) => {
                cmd.draw_indexed(index_count, instance_count, first_index, first_instance)
            }
            #[cfg(target_os = "macos")]
            CommandBuffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            CommandBuffer::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn bind_vertex_buffer(&self, buffer: &Buffer) {
        match self {
            CommandBuffer::Vulkan(cmd) => cmd.bind_vertex_buffer(buffer.as_vulkan()),
            #[cfg(target_os = "macos")]
            CommandBuffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            CommandBuffer::DirectX => unimplemented!(),
        }
    }

    pub(crate) fn bind_index_buffer(&self, buffer: &Buffer) {
        match self {
            CommandBuffer::Vulkan(cmd) => cmd.bind_index_buffer(buffer.as_vulkan()),
            #[cfg(target_os = "macos")]
            CommandBuffer::Metal => unimplemented!(),
            #[cfg(target_os = "windows")]
            CommandBuffer::DirectX => unimplemented!(),
        }
    }
}
