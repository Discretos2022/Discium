use crate::{renderer::{baserenderer::BaseRenderer, renderer_enum::RendererEnum, renderer_factory::RendererFactory, renderer_type::RendererType, resource_handles::{IndexBufferHandle, VertexBufferHandle}, resources::{base_vertex::BaseVertex, base_index::BaseIndex}}, window::rawhandle::RawHandle};



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

    pub fn end_draw(&mut self) {
        match &mut self.renderer_handle {
            RendererEnum::Vulkan(vulkan_renderer) => vulkan_renderer.end_draw(),
        }
    }

    pub fn draw_indexed<V: BaseVertex, I: BaseIndex>(&self, vertex_buffer_handle: VertexBufferHandle<V>, index_buffer_handle: IndexBufferHandle<I>) {
        match &self.renderer_handle {
            RendererEnum::Vulkan(vulkan_renderer) => vulkan_renderer.draw_indexed(vertex_buffer_handle, index_buffer_handle),
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


    pub fn create_vertex_buffer<V: BaseVertex>(&mut self, size: u64) -> VertexBufferHandle<V> {

        let id: u32 = match &mut self.renderer_handle {
            RendererEnum::Vulkan(vulkan_renderer) => {
                vulkan_renderer.create_vertex_buffer::<V>(size)
            },
        };

        return VertexBufferHandle::new(id);
    }

    pub fn set_vertex_buffer_data<V: BaseVertex>(&mut self, handle: VertexBufferHandle<V>, data: &[V]) {

        match &mut self.renderer_handle {
            RendererEnum::Vulkan(vulkan_renderer) => {
                vulkan_renderer.set_vertex_buffer_data(handle, data);
            },
        }
        
    }

    pub fn create_index_buffer<I: BaseIndex>(&mut self, size: u64) -> IndexBufferHandle<I> {
        
        let id: u32 = match &mut self.renderer_handle {
            RendererEnum::Vulkan(vulkan_renderer) => {
                vulkan_renderer.create_index_buffer::<I>(size)
            },
        };
        
        return IndexBufferHandle::new(id);
    }

    pub fn set_index_buffer_data<I: BaseIndex>(&mut self, handle: IndexBufferHandle<I>, data: &[I]) {

        match &mut self.renderer_handle {
            RendererEnum::Vulkan(vulkan_renderer) => {
                vulkan_renderer.set_index_buffer_data(handle, data);
            },
        }
        
    }

}