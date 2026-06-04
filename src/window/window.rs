#[cfg(target_os = "linux")]
use crate::window::linux::linux_window::LinuxWindow;

#[cfg(target_os = "windows")]
use crate::window::win32window::Win32Window;

use crate::window::{basewindow::BaseWindow, event_enum::WindowEvent, rawhandle::RawHandle, window_config::WindowConfig, windowfactory::WindowFactory};


pub struct Window {

    #[cfg(target_os = "windows")]
    window_handle: Win32Window,

    #[cfg(target_os = "linux")]
    window_handle: LinuxWindow,

    #[cfg(target_os = "macos")]
    window_handle: MacosWindow,

}


impl Window {

    pub fn create(config: &WindowConfig) -> Window {

        return Window {
            window_handle: WindowFactory::create(config),
        };

    }

    pub fn get_raw_handle(&self) -> RawHandle {
        return self.window_handle.get_raw_handle();
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        return self.window_handle.get_window_size();
    }

    pub fn pool_events(&mut self) -> Vec<WindowEvent> {
        return self.window_handle.pool_events();
    }
    
}