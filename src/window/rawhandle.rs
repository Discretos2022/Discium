use std::ffi::c_void;


#[derive(Clone)]
pub enum RawHandle {

    Win32 { hwnd: isize, hinstance: isize },
    // X11(),
    Wayland { display: *mut c_void, surface: *mut c_void },
    // MacOS(),

}