
#[cfg(target_os = "linux")]
use {
    std::env,
    
    crate::window::{basewindow::BaseWindow, linux::{linux_enum::LinuxEnum, wayland_window::WaylandWindow}},
};



#[cfg(target_os = "linux")]
pub struct LinuxWindow {

    window_handle: LinuxEnum,

}

#[cfg(not(target_os = "linux"))]
pub struct LinuxWindow;


#[cfg(target_os = "linux")]
impl BaseWindow for LinuxWindow {

    fn create(config: &crate::window::window_config::WindowConfig) -> Self where Self: Sized {

        let session_type = env::var("XDG_SESSION_TYPE").unwrap_or_default();

        if session_type == "wayland" {
            return LinuxWindow { window_handle: LinuxEnum::Wayland(WaylandWindow::create(config)) };
        }
        else if session_type == "x11" {
            // return LinuxWindow { window_handle: LinuxEnum::Wayland(WaylandWindow::create(config)) };
            panic!("Linux : X11 Is Not Supported !");
        }

        panic!("Linux : No Graphics Session !");

    }

    fn pool_events(&mut self) -> Vec<crate::window::event_enum::WindowEvent> {
        match &mut self.window_handle {
            LinuxEnum::Wayland(wayland_window) => wayland_window.pool_events(),
        }
    }

    fn get_raw_handle(&self) -> crate::window::rawhandle::RawHandle {
        match &self.window_handle {
            LinuxEnum::Wayland(wayland_window) => wayland_window.get_raw_handle(),
        }
    }

    fn get_window_size(&self) -> (u32, u32) {
        match &self.window_handle {
            LinuxEnum::Wayland(wayland_window) => wayland_window.get_window_size(),
        }
    }

}