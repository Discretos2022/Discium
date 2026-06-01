use crate::{renderer::{baserenderer::BaseRenderer, renderer_enum::RendererEnum, renderer_factory::RendererFactory, renderer_type::RendererType, vulkanrenderer::VulkanRenderer}, window::rawhandle::RawHandle};



pub struct Renderer {

    pub renderer_handle: RendererEnum,

}


impl Renderer {

    pub fn create(renderer_type: RendererType, raw_handle: &RawHandle, surface_dimension: (u32, u32)) -> Renderer {
        return Self {
            renderer_handle: RendererFactory::create(renderer_type, raw_handle, surface_dimension),
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

    pub fn update_surface_dimension(&mut self, surface_dimension: (u32, u32)) {
        match &mut self.renderer_handle {
            RendererEnum::Vulkan(vulkan_renderer) => vulkan_renderer.update_surface_dimension(surface_dimension),
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