
#[derive(Clone)]
pub enum RawHandle {

    Win32 { hwnd: isize, hinstance: isize },
    // X11(),
    // Wayland(),
    // MacOS(),

}