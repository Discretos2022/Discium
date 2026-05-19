use glam::Mat4;

#[repr(C)]
pub struct VulkanUniformBufferObject {
    pub model: Mat4,
    pub view: Mat4,
    pub proj: Mat4,
}