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
            #[cfg(target_os = "windows")]
            WindowEnum::Windows(w) => w.get_raw_handle(),
            #[cfg(target_os = "linux")]
            WindowEnum::Linux(w) => w.get_raw_handle(),
            _ => panic!("Platoform Was Not Supported !"),
        }

    }

    pub fn get_window_size(&self) -> (u32, u32) {

        match &self.window_handle {
            #[cfg(target_os = "windows")]
            WindowEnum::Windows(w) => w.get_window_size(),
            #[cfg(target_os = "linux")]
            WindowEnum::Linux(w) => w.get_window_size(),
            _ => panic!("Platoform Was Not Supported !"),
        }

    }

    pub fn pool_events(&mut self) -> Vec<WindowEvent> {

        match &mut self.window_handle {
            #[cfg(target_os = "windows")]
            WindowEnum::Windows(w) => w.pool_events(),
            #[cfg(target_os = "linux")]
            WindowEnum::Linux(w) => w.pool_events(),
            _ => panic!("Platoform Was Not Supported !"),
        }

    }
    
}