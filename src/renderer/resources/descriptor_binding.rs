use crate::renderer::resources::descriptor_type::DescriptorType;
use crate::renderer::resources::shader_type::ShaderType;



pub struct DescriptorBinding {

    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub shader_type: ShaderType,

}