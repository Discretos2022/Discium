use crate::window::rawhandle::RawHandle;


pub trait BaseRenderer {

    fn create(raw_handle: &RawHandle, surface_dimension: (u32, u32)) -> Self where Self: Sized;
    fn update_surface_dimension(&mut self, surface_dimension: (u32, u32));
    // fn begin_draw(&mut self);
    // fn draw_image(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);

}