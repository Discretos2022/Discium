use ash::vk;


pub struct VulkanVertexBuffer {
    pub count: usize,
    pub size: u64,
    pub buffer: vk::Buffer,
    pub buffer_memory: vk::DeviceMemory,
}


impl VulkanVertexBuffer {

    pub fn destroy(&mut self, device: &ash::Device) {
        unsafe { 
            device.destroy_buffer(self.buffer, None);
            device.free_memory(self.buffer_memory, None);
        };
    }

}