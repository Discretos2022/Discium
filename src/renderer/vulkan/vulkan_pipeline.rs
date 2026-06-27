use ash::vk;
use ash::vk::PipelineColorBlendAttachmentState;

use crate::renderer::resources::blend_mode::BlendMode;
use crate::renderer::resources::pipeline_config::PipelineConfig;
use crate::renderer::vulkan::vulkan_descriptor_set_layout::VulkanDescriptorSetLayout;
use crate::renderer::vulkan::vulkan_shader::VulkanShader;
use crate::renderer::vulkan::vulkan_utils::VulkanUtils;


pub struct VulkanPipeline {

    pub pipeline: vk::Pipeline,
    pub layout: vk::PipelineLayout,

}


impl VulkanPipeline {

    pub fn create(device: &ash::Device, render_pass: vk::RenderPass, shaders: &[&VulkanShader], descriptor_set_layout: &VulkanDescriptorSetLayout, config: PipelineConfig) -> Self {

        let mut shader_stages_info = Vec::new();

        for shader in shaders {

            let shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
                .stage(shader.shader_stage)
                .module(shader.module)
                .name(c"main");

            shader_stages_info.push(shader_stage_info);
        }

        let binding_description = [VulkanUtils::get_vertex_binding_descriptions(&config.vertex_declaration)];
        let attribute_descriptions = VulkanUtils::get_vertex_attribute_descriptions(&config.vertex_declaration);

        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&binding_description)
            .vertex_attribute_descriptions(&attribute_descriptions);

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(VulkanUtils::primitive_type_to_vk_primitive_topology(config.primitive_type))
            .primitive_restart_enable(false);

        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewport_count(config.viewport_count)
            .scissor_count(config.scissor_count);

        let rasteriser = vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(VulkanUtils::fill_mode_to_vk_polygone_mode(config.rasterizer.fill_mode))
            .line_width(1.0)
            .cull_mode(VulkanUtils::cull_mode_to_vk_cull_mode(config.rasterizer.cull_mode))
            .front_face(VulkanUtils::front_mode_to_vk_front_face(config.rasterizer.front_mode))
            .depth_bias_enable(false)
            .depth_bias_constant_factor(0.0)
            .depth_bias_clamp(0.0)
            .depth_bias_slope_factor(0.0);

        let multisampler = vk::PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(VulkanUtils::sample_level_to_vk_sample_count(config.multisampler.rasterization_sample_level))
            .min_sample_shading(1.0)
            //.sample_mask()
            .alpha_to_coverage_enable(false)
            .alpha_to_one_enable(false);

        let mut color_blend_attachment = Vec::new();
        for state in config.blend_state.blend_states {
            color_blend_attachment.push(Self::build_vk_pipeline_color_blend_attachment_state(state));
        }

        let color_blending = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .attachments(&color_blend_attachment)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&descriptor_set_layout.descriptor_set_layouts);

        let dynamic_states= [
            vk::DynamicState::VIEWPORT,
            vk::DynamicState::SCISSOR,
            // vk::DynamicState::LINE_WIDTH,
            // vk::DynamicState::BLEND_CONSTANTS
        ];

        let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::default()
            .dynamic_states(&dynamic_states);


        let pipeline_layout = unsafe { device.create_pipeline_layout(&pipeline_layout_info, None).expect("Pipeline Layout Creation Failed !") };

        let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages_info)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasteriser)
            .multisample_state(&multisampler)
            // .depth_stencil_state()
            .color_blend_state(&color_blending)
            .dynamic_state(&dynamic_state_info)
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0);
            // .base_pipeline_handle()
            // .base_pipeline_index();

        let pipeline = unsafe { device.create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None).expect("Graphics Pipeline Creation Failed !") };

        return Self {
            pipeline: pipeline[0],
            layout: pipeline_layout,
        }
    }


    pub fn build_vk_pipeline_color_blend_attachment_state(blend_mode: BlendMode) -> PipelineColorBlendAttachmentState {

        return match blend_mode {
            BlendMode::Opaque => {
                vk::PipelineColorBlendAttachmentState::default()
                    .blend_enable(false)
            },
            BlendMode::AlphaBlend => {
                vk::PipelineColorBlendAttachmentState::default()
                    .color_write_mask(vk::ColorComponentFlags::R | vk::ColorComponentFlags::G | vk::ColorComponentFlags::B | vk::ColorComponentFlags::A)
                    .blend_enable(true)
                    .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
                    .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
                    .color_blend_op(vk::BlendOp::ADD)
                    .src_alpha_blend_factor(vk::BlendFactor::ONE)
                    .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
                    .alpha_blend_op(vk::BlendOp::ADD)
            },
            BlendMode::Additive => {
                vk::PipelineColorBlendAttachmentState::default()
                    .color_write_mask(vk::ColorComponentFlags::R | vk::ColorComponentFlags::G | vk::ColorComponentFlags::B | vk::ColorComponentFlags::A)
                    .blend_enable(true)
                    .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
                    .dst_color_blend_factor(vk::BlendFactor::ONE)
                    .color_blend_op(vk::BlendOp::ADD)
                    .src_alpha_blend_factor(vk::BlendFactor::ONE)
                    .dst_alpha_blend_factor(vk::BlendFactor::ONE)
                    .alpha_blend_op(vk::BlendOp::ADD)
            },
            BlendMode::NonPremultiplied => {
                vk::PipelineColorBlendAttachmentState::default()
                    .color_write_mask(vk::ColorComponentFlags::R | vk::ColorComponentFlags::G | vk::ColorComponentFlags::B | vk::ColorComponentFlags::A)
                    .blend_enable(true)
                    .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
                    .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
                    .color_blend_op(vk::BlendOp::ADD)
                    .src_alpha_blend_factor(vk::BlendFactor::ONE)
                    .dst_alpha_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
                    .alpha_blend_op(vk::BlendOp::ADD)
            },
        }

    }


    pub fn destroy(&mut self, device: &ash::Device) {
        unsafe {
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.layout, None);
        }
    }

}