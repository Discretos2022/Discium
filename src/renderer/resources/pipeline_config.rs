use crate::renderer::{resource_handles::{ShaderHandle, ShaderLayoutHandle}, resources::{blend_state::BlendState, descriptor_binding::DescriptorBinding, multisampler::Multisampler, primitive_type::PrimitiveType, rasterizer::Rasterizer, scissor_config::ScissorConfig, vertex_declaration::VertexDeclaration, viewport_config::ViewportConfig}};


pub struct PipelineConfig {

    pub vertex_declaration: VertexDeclaration,
    pub shaders: Vec<ShaderHandle>,
    pub primitive_type: PrimitiveType,

    pub rasterizer: Rasterizer,
    pub multisampler: Multisampler,
    pub blend_state: BlendState,
    pub shader_layout_handle: ShaderLayoutHandle,
    // pub descriptor_bindings: Vec<DescriptorBinding>,

    // pub viewports: Vec<Viewport>,
    // pub scissors: Vec<Scissor>,

    pub viewport_count: u32,
    pub scissor_count: u32,

}


impl PipelineConfig {

    pub fn create(shaders: Vec<ShaderHandle>, vertex_declaration: VertexDeclaration, shader_layout_handle: ShaderLayoutHandle) -> Self {

        return Self {
            vertex_declaration: vertex_declaration,
            shaders: shaders,
            primitive_type: PrimitiveType::TriangleList,
            rasterizer: Rasterizer::create(),
            multisampler: Multisampler::create(),
            blend_state: BlendState::create(),
            // descriptor_bindings: Vec::new(),
            shader_layout_handle: shader_layout_handle,
            // viewports: Vec::new(),
            // scissors: Vec::new(),
            viewport_count: 1,
            scissor_count: 1,
        }

    }


    pub fn vertex_declaration(mut self, vertex_declaration: VertexDeclaration) -> Self {
        self.vertex_declaration = vertex_declaration;
        return self;
    }

    pub fn primitive_type(mut self, primitive_type: PrimitiveType) -> Self {
        self.primitive_type = primitive_type;
        return self;
    }

    pub fn rasterizer(mut self, rasterizer: Rasterizer) -> Self {
        self.rasterizer = rasterizer;
        return self;
    }

    pub fn multisampler(mut self, multisampler: Multisampler) -> Self {
        self.multisampler = multisampler;
        return self;
    }

    pub fn blend_state(mut self, blend_state: BlendState) -> Self {
        self.blend_state = blend_state;
        return self;
    }

    pub fn shader_layout_handle(mut self, shader_layout_handle: ShaderLayoutHandle) -> Self {
        self.shader_layout_handle = shader_layout_handle;
        return self;
    }

    pub fn viewport_count(mut self, viewport_count: u32) -> Self {
        self.viewport_count = viewport_count;
        return self;
    }

    pub fn scissor_count(mut self, scissor_count: u32) -> Self {
        self.scissor_count = scissor_count;
        return self;
    }

}
