use ash::vk;


pub struct VulkanIndexBuffer {
    pub count: usize,
    pub size: u64,
    pub buffer: vk::Buffer,
    pub buffer_memory: vk::DeviceMemory,
}