use crate::{renderer::{baserenderer::BaseRenderer, renderer_enum::RendererEnum, renderer_factory::RendererFactory, renderer_type::RendererType, vulkanrenderer::VulkanRenderer}, window::rawhandle::RawHandle};



pub struct Renderer {

    pub renderer_handle: RendererEnum,

}


impl Renderer {

    pub fn create(renderer_type: RendererType, raw_handle: &RawHandle) -> Renderer {
        return Self {
            renderer_handle: RendererFactory::create(renderer_type, raw_handle),
        };
    }

    pub fn begin_draw(&mut self) {
        match &mut self.renderer_handle {
            RendererEnum::Vulkan(vulkan_renderer) => vulkan_renderer.begin_draw(),
        }
    }

    pub fn draw_image(&mut self) {
        match &mut self.renderer_handle {
            RendererEnum::Vulkan(vulkan_renderer) => vulkan_renderer.draw_image(),
        }
    }

    pub fn recreate_swapchain(&mut self, raw_handle: &RawHandle) {
        match &mut self.renderer_handle {
            RendererEnum::Vulkan(vulkan_renderer) => vulkan_renderer.recreate_swapchain(raw_handle),
        }
    }

    pub fn pause(&mut self) {
        match &mut self.renderer_handle {
            RendererEnum::Vulkan(vulkan_renderer) => vulkan_renderer.pause(),
        }
    }

    pub fn resume(&mut self) {
        match &mut self.renderer_handle {
            RendererEnum::Vulkan(vulkan_renderer) => vulkan_renderer.resume(),
        }
    }

}