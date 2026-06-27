use crate::graphics_device::GraphicsDevice;



pub trait GameBase {

    fn initialize(&mut self, graphics_device: &mut GraphicsDevice);
    fn update(&mut self, graphics_device: &mut GraphicsDevice);
    fn draw(&mut self, graphics_device: &mut GraphicsDevice);

}