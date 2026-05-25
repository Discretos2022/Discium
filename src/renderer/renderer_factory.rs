use crate::{renderer::{baserenderer::BaseRenderer, renderer_enum::RendererEnum, renderer_type::RendererType, vulkanrenderer::VulkanRenderer}, window::rawhandle::RawHandle};




pub struct RendererFactory;


impl RendererFactory {

    pub fn create(renderer_type: RendererType, raw_handle: &RawHandle) -> RendererEnum {

        match renderer_type {
            RendererType::Vulkan => RendererEnum::Vulkan(VulkanRenderer::create(raw_handle)),
        }

    }

}