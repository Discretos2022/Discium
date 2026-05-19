
// use windows::{
//     Win32::{Foundation::*, System::LibraryLoader::*, UI::WindowsAndMessaging::*, },
//     core::*,
// };

use windows::Win32::System::LibraryLoader::GetModuleHandleW;
// use windows::Win32::UI::WindowsAndMessaging::{
//     CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, IDC_ARROW, LoadCursorW, PostQuitMessage, RegisterClassW, WINDOW_EX_STYLE, WM_DESTROY, WNDCLASSW
// };
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::core::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::window::basewindow::BaseWindow;
use crate::window::rawhandle::RawHandle;


pub struct Win32Window {

    pub hwnd: HWND,
    pub hinstance: HINSTANCE,

}


impl BaseWindow for Win32Window {

    fn create() -> Self {

        unsafe {
            
            let hinstance = HINSTANCE(GetModuleHandleW(None).unwrap().0);

            let class_name = w!("Window Test");

            let wc = WNDCLASSW {
                hInstance: hinstance,
                lpszClassName: class_name,
                lpfnWndProc: Some(Self::wndproc),
                ..Default::default()
            };

            RegisterClassW(&wc);

            let hwnd = match CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name,
                w!("𝕯𝖎𝖘𝖈𝖎𝖚𝖒 0.1"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                800,
                600,
                None,
                None,
                Some(hinstance),
                None,
            ) {
                Ok(h) => h,
                Err(e) => panic!("Window Creation Was failed : {e:?}"),
            }; // Soit un .unwrap() à la place du match 

            ShowWindow(hwnd, SW_SHOW);

            return Self { 
                hwnd: hwnd, 
                hinstance: hinstance
            };

        }

    }

    fn pool_events(&self) -> bool {

        unsafe {
            let mut msg = MSG::default();
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() {

                if msg.message == WM_QUIT {
                    return false;
                }

                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        return true;
    }

    fn get_raw_handle(&self) -> RawHandle {

        let mut rect: RECT = RECT::default();

        unsafe { GetClientRect(self.hwnd, &mut rect) };

        return RawHandle::Win32 { hwnd: self.hwnd.0 as isize, hinstance: self.hinstance.0 as isize, width: rect.right, height: rect.bottom }
    }

}


impl Win32Window {

    // pub fn run(&self) {

    //     unsafe {
    //         let mut msg = MSG::default();
    //         while GetMessageW(&mut msg, None, 0, 0).into() {
    //             TranslateMessage(&msg);
    //             DispatchMessageW(&msg);
    //         }
    //     }

    // }

    extern "system" fn wndproc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            match msg {
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcW(hwnd, msg, wparam, lparam)
            }
        }
    }

}