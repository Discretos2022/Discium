use crate::{renderer::{baserenderer::BaseRenderer, renderer_enum::RendererEnum, renderer_type::RendererType, vulkan::vulkanrenderer::VulkanRenderer}, window::rawhandle::RawHandle};




pub struct RendererFactory;


impl RendererFactory {

    pub fn create(renderer_type: RendererType, raw_handle: &RawHandle, surface_dimension: (u32, u32)) -> RendererEnum {

        match renderer_type {
            RendererType::Vulkan => RendererEnum::Vulkan(VulkanRenderer::create(raw_handle, surface_dimension)),
        }

    }

}