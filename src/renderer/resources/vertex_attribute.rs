use crate::renderer::resources::vertex_format::VertexFormat;


pub struct VertexAttribute {
    pub location: u32,
    pub format: VertexFormat,
    pub offset: u32,
}