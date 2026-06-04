use crate::window::{basewindow::BaseWindow, window_config::WindowConfig};

#[cfg(target_os = "windows")]
use crate::window::win32window::Win32Window;

#[cfg(target_os = "linux")]
use crate::window::linux::linux_window::LinuxWindow;

#[cfg(target_os = "macos")]
use crate::window::macoswindow::MacOSWindow;

pub struct WindowFactory;

impl WindowFactory {

    #[cfg(target_os = "windows")]
    pub fn create(config: &WindowConfig) -> Win32Window {
        return Win32Window::create(config);
    }

    #[cfg(target_os = "linux")]
    pub fn create(config: &WindowConfig) -> LinuxWindow {
        return LinuxWindow::create(config);
    }

    #[cfg(target_os = "macos")]
    pub fn create(config: &WindowConfig) -> MacOSWindow {
        return WindowEnum::MacOS(MacOsWindow::create());
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    pub fn create(config: &WindowConfig) -> Win32Window {
        compile_error!("This target OS is not supported by windowing system !");
    }

}