use ash::vk;

use crate::renderer::resource_handles::ShaderLayoutHandle;


pub struct VulkanUniformBuffer {
    pub shader_layout: ShaderLayoutHandle,
    pub buffer: Vec<vk::Buffer>,
    pub buffer_memory: Vec<vk::DeviceMemory>,
    pub descriptor_sets: Vec<vk::DescriptorSet>,
}


impl VulkanUniformBuffer {

    pub fn update_descriptor_sets(&mut self, descriptor_sets: Vec<vk::DescriptorSet>) {
        self.descriptor_sets = descriptor_sets;
    }


    pub fn destroy(&mut self, device: &ash::Device) {
        unsafe {
            for buffer in &self.buffer {
                device.destroy_buffer(*buffer, None);
            }
            for buffer_memory in &self.buffer_memory {
                device.free_memory(*buffer_memory, None);
            }
        };
    }


}