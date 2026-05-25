use crate::window::{basewindow::BaseWindow, event_enum::WindowEvent, rawhandle::RawHandle, window_config::WindowConfig, window_enum::WindowEnum, windowfactory::WindowFactory};


pub struct Window {

    pub window_handle: WindowEnum,

}


impl Window {

    pub fn create(config: &WindowConfig) -> Window {

        return Window {
            window_handle: WindowFactory::create(config),
        };

    }


    pub fn get_raw_handle(&self) -> RawHandle {

        match &self.window_handle {
            WindowEnum::Windows(w) => w.get_raw_handle(),
        }

    }

    pub fn pool_events(&mut self) -> Vec<WindowEvent> {

        match &mut self.window_handle {
            WindowEnum::Windows(w) => w.pool_events(),
        }

    }
    
}