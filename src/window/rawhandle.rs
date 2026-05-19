
pub enum RawHandle {

    Win32 { hwnd: isize, hinstance: isize, width: i32, height: i32 },
    // X11(),
    // Wayland(),
    // MacOS(),

}