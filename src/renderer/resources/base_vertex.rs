use crate::renderer::resources::vertex_declaration::VertexDeclaration;


pub trait BaseVertex {

    fn get_vertex_declaration() -> VertexDeclaration;

}