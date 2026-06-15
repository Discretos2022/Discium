use ash::vk;


pub struct VulkanVertexBuffer {
    pub count: usize,
    pub size: u64,
    pub buffer: vk::Buffer,
    pub buffer_memory: vk::DeviceMemory,
}