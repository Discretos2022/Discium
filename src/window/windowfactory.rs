use crate::window::{basewindow::BaseWindow, window_config::WindowConfig};

#[cfg(target_os = "windows")]
use crate::window::{win32window::Win32Window, window_enum::WindowEnum};

#[cfg(target_os = "macos")]
use crate::window::macoswindow::MacOSWindow;

pub struct WindowFactory;

impl WindowFactory {

    pub fn create(config: &WindowConfig) -> WindowEnum {

        #[cfg(target_os = "windows")]
        return WindowEnum::Windows(Win32Window::create(config));

        #[cfg(target_os = "macos")]
        return WindowEnum::MacOS(MacOsWindow::create());

        #[cfg(target_os = "linux")]
        return WindowEnum::Linux(LinuxWindow::create());

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        compile_error!("This target OS is not supported by windowing system !");

    }

}