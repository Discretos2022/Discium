use ash::vk;


#[repr(C)]
pub struct VulkanVertex {

    pub pos: [f32; 2],
    pub color: [f32; 3],

}

impl VulkanVertex {

    pub fn get_binding_description() -> vk::VertexInputBindingDescription {

        vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(std::mem::size_of::<VulkanVertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)

    }

    pub fn get_attribut_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32_SFLOAT)
                .offset(0),
            
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(std::mem::offset_of!(VulkanVertex, color) as u32)
        ]
    }

}