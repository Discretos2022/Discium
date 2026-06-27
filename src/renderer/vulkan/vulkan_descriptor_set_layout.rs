use ash::vk;

use crate::renderer::resources::descriptor_binding::DescriptorBinding;
use crate::renderer::vulkan::vulkan_utils::VulkanUtils;


pub struct VulkanDescriptorSetLayout {

    pub descriptor_set_layouts: Vec<vk::DescriptorSetLayout>

}


impl VulkanDescriptorSetLayout {

    pub fn create(device: &ash::Device, bindings: Vec<DescriptorBinding>) -> Self {

        let mut layout_binding = Vec::new();

        for b in bindings {
            layout_binding.push(
                vk::DescriptorSetLayoutBinding::default()
                    .binding(b.binding)
                    .descriptor_type(VulkanUtils::descriptor_type_to_vk_descriptor_type(b.descriptor_type))
                    .descriptor_count(1)
                    .stage_flags(VulkanUtils::shader_type_to_vk_shader_stage(b.shader_type))
                    //.immutable_samplers();
            )
        }

        let layout_info = vk::DescriptorSetLayoutCreateInfo::default()
            .bindings(&layout_binding);

        let descriptor_set_layouts = unsafe { device.create_descriptor_set_layout(&layout_info, None).expect("Descriptor Layout Creation Failed !") };

        return Self {
            descriptor_set_layouts: vec![descriptor_set_layouts],
        }

    }


    pub fn destroy(&mut self, device: &ash::Device) {
        unsafe { 
            for desc in &self.descriptor_set_layouts {
                device.destroy_descriptor_set_layout(*desc, None);
            }
        };
    }

}