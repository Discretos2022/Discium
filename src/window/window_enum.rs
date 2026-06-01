use crate::window::{linuxwindow::LinuxWindow, win32window::Win32Window};


pub enum WindowEnum {

    Windows(Win32Window),
    Linux(LinuxWindow),
    // MacOS(),

}