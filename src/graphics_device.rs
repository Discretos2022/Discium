use crate::renderer::renderer::Renderer;
use crate::window::window::Window;


pub struct GraphicsDevice {
    pub window: Window,
    pub renderer: Renderer,
}