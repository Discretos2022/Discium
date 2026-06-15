use crate::renderer::resources::{base_vertex::BaseVertex, vertex_attribute::VertexAttribute, vertex_format::VertexFormat};


#[derive(Copy, Clone)]
#[repr(C)]
pub struct VertexPositionColor {
    pub pos: [f32; 2],
    pub color: [f32; 3],
}


impl BaseVertex for VertexPositionColor {

    fn get_vertex_declaration() -> super::vertex_declaration::VertexDeclaration {
        
        return super::vertex_declaration::VertexDeclaration {
            stride: std::mem::size_of::<VertexPositionColor>() as u32,
            attributes: vec![

                VertexAttribute {
                    location: 0,
                    format: VertexFormat::Float2,
                    offset: 0,
                },

                VertexAttribute {
                    location: 1,
                    format: VertexFormat::Float3,
                    offset: std::mem::offset_of!(VertexPositionColor, color) as u32,
                }

            ],
        }

    }

}