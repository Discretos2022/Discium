use glam::Mat4;

use crate::renderer::resources::base_uniform::BaseUniform;

#[repr(C)]
pub struct UniformMVP {
    pub model: Mat4,
    pub view: Mat4,
    pub proj: Mat4,
}


impl BaseUniform for UniformMVP {}