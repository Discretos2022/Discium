use crate::window::{event_enum::WindowEvent, rawhandle::RawHandle, window_config::WindowConfig};


pub trait BaseWindow {

    fn create(config: &WindowConfig) -> Self where Self: Sized;
    fn pool_events(&mut self) -> Vec<WindowEvent>;
    fn get_raw_handle(&self) -> RawHandle;
    fn get_window_size(&self) -> (u32, u32);

}