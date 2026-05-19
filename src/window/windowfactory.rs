use crate::window::basewindow::BaseWindow;

#[cfg(target_os = "windows")]
use crate::window::win32window::Win32Window;

#[cfg(target_os = "macos")]
use crate::window::macoswindow::MacOSWindow;

pub struct WindowFactory;

impl WindowFactory {

    pub fn create() -> Box<dyn BaseWindow> {

        #[cfg(target_os = "windows")]
        return Box::new(Win32Window::create());

        // #[cfg(target_os = "linux")]
        // return Self::create_linux();

        #[cfg(target_os = "macos")]
        return Box::new(MacOSWindow::create());

    }

}