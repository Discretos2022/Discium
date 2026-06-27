use ash::vk;

use crate::renderer::resources::cull_mode::CullMode;
use crate::renderer::resources::descriptor_type::DescriptorType;
use crate::renderer::resources::fill_mode::FillMode;
use crate::renderer::resources::front_mode::FrontMode;
use crate::renderer::resources::primitive_type::PrimitiveType;
use crate::renderer::resources::sample_level::SampleLevel;
use crate::renderer::resources::shader_type::ShaderType;
use crate::renderer::resources::vertex_declaration::VertexDeclaration;
use crate::renderer::resources::vertex_format::VertexFormat;



pub struct VulkanUtils;


impl VulkanUtils {

    pub fn get_vertex_binding_descriptions(declararion: &VertexDeclaration) -> vk::VertexInputBindingDescription {

        vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(declararion.stride)
            .input_rate(vk::VertexInputRate::VERTEX)

    }


    pub fn get_vertex_attribute_descriptions(declaration: &VertexDeclaration) -> Vec<vk::VertexInputAttributeDescription> {

        let mut attributes: Vec<vk::VertexInputAttributeDescription> = Vec::new();

        for e in &declaration.attributes {
            
            let attr = vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(e.location)
                .format(Self::vertex_format_to_vk_format(e.format))
                .offset(e.offset);

            attributes.push(attr);
        }

        return attributes;
    }


    pub fn vertex_format_to_vk_format(format: VertexFormat) -> vk::Format {
        match format {
            VertexFormat::Float2 => vk::Format::R32G32_SFLOAT,
            VertexFormat::Float3 => vk::Format::R32G32B32_SFLOAT,
            VertexFormat::Float4 => vk::Format::R32G32B32A32_SFLOAT,
            _ => panic!("Vertex Format not exist !"),
        }
    }


    pub fn primitive_type_to_vk_primitive_topology(primitive_type: PrimitiveType) -> vk::PrimitiveTopology {
        match primitive_type {
            PrimitiveType::PointList => vk::PrimitiveTopology::POINT_LIST,
            PrimitiveType::LineList => vk::PrimitiveTopology::LINE_LIST,
            PrimitiveType::LineStrip => vk::PrimitiveTopology::LINE_STRIP,
            PrimitiveType::TriangleList => vk::PrimitiveTopology::TRIANGLE_LIST,
            PrimitiveType::TriangleStrip => vk::PrimitiveTopology::TRIANGLE_STRIP,
            _ => panic!("Primitive Type not exist !"),
        }
    }


    pub fn shader_type_to_vk_shader_stage(shader_type: ShaderType) -> vk::ShaderStageFlags {
        match shader_type {
            ShaderType::Vertex => vk::ShaderStageFlags::VERTEX,
            ShaderType::Fragment => vk::ShaderStageFlags::FRAGMENT,
            ShaderType::Geometry => vk::ShaderStageFlags::GEOMETRY,
            ShaderType::Compute => vk::ShaderStageFlags::COMPUTE,
        }
    }


    pub fn fill_mode_to_vk_polygone_mode(fill_mode: FillMode) -> vk::PolygonMode {
        match fill_mode {
            FillMode::Fill => vk::PolygonMode::FILL,
            FillMode::Line => vk::PolygonMode::LINE,
            FillMode::Point => vk::PolygonMode::POINT,
        }
    }


    pub fn cull_mode_to_vk_cull_mode(cull_mode: CullMode) -> vk::CullModeFlags {
        match cull_mode {
            CullMode::None => vk::CullModeFlags::NONE,
            CullMode::Front => vk::CullModeFlags::FRONT,
            CullMode::Back => vk::CullModeFlags::BACK,
            CullMode::FrontAndBack => vk::CullModeFlags::FRONT_AND_BACK,
        }
    }


    pub fn front_mode_to_vk_front_face(front_mode: FrontMode) -> vk::FrontFace {
        match front_mode {
            FrontMode::CounterClockWise => vk::FrontFace::COUNTER_CLOCKWISE,
            FrontMode::ClockWise => vk::FrontFace::CLOCKWISE,
        }
    }


    pub fn sample_level_to_vk_sample_count(sample_level: SampleLevel) -> vk::SampleCountFlags {
        match sample_level {
            SampleLevel::Type1 => vk::SampleCountFlags::TYPE_1,
            SampleLevel::Type2 => vk::SampleCountFlags::TYPE_2,
            SampleLevel::Type4 => vk::SampleCountFlags::TYPE_4,
            SampleLevel::Type8 => vk::SampleCountFlags::TYPE_8,
            SampleLevel::Type16 => vk::SampleCountFlags::TYPE_16,
            SampleLevel::Type32 => vk::SampleCountFlags::TYPE_32,
            SampleLevel::Type64 => vk::SampleCountFlags::TYPE_64,
        }
    }


    pub fn descriptor_type_to_vk_descriptor_type(descriptor_type: DescriptorType) -> vk::DescriptorType {
        match descriptor_type {
            DescriptorType::Sampler => vk::DescriptorType::SAMPLER,
            DescriptorType::CombinedImageSampler => vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            DescriptorType::SampledImage => vk::DescriptorType::SAMPLED_IMAGE,
            DescriptorType::StorageImage => vk::DescriptorType::STORAGE_IMAGE,
            DescriptorType::UniformTexelBuffer => vk::DescriptorType::UNIFORM_TEXEL_BUFFER,
            DescriptorType::StorageTexelBuffer => vk::DescriptorType::STORAGE_TEXEL_BUFFER,
            DescriptorType::UniformBuffer => vk::DescriptorType::UNIFORM_BUFFER,
            DescriptorType::StorageBuffer => vk::DescriptorType::STORAGE_BUFFER,
            DescriptorType::UniformBufferDynamic => vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
            DescriptorType::StorageBufferDynamic => vk::DescriptorType::STORAGE_BUFFER_DYNAMIC,
            DescriptorType::InputAttachment => vk::DescriptorType::INPUT_ATTACHMENT,
        }
    }

}