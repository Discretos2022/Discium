
// use windows::{
//     Win32::{Foundation::*, System::LibraryLoader::*, UI::WindowsAndMessaging::*, },
//     core::*,
// };

use std::collections::HashMap;
use std::sync::Mutex;

use windows::Win32::System::LibraryLoader::GetModuleHandleW;
// use windows::Win32::UI::WindowsAndMessaging::{
//     CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, IDC_ARROW, LoadCursorW, PostQuitMessage, RegisterClassW, WINDOW_EX_STYLE, WM_DESTROY, WNDCLASSW
// };
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::core::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::window::basewindow::BaseWindow;
use crate::window::event_enum::WindowEvent;
use crate::window::rawhandle::RawHandle;
use crate::window::window_config::WindowConfig;


pub struct Win32Window {

    pub hwnd: HWND,
    pub hinstance: HINSTANCE,

}

static WINPROC_EVENTS: std::sync::LazyLock<Mutex<HashMap<isize, Vec<WindowEvent>>>> = std::sync::LazyLock::new(||Mutex::new(HashMap::new()));

impl BaseWindow for Win32Window {

    fn create(config: &WindowConfig) -> Self {

        let title = config.title;
        let win_title: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();

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
                PCWSTR(win_title.as_ptr()), //    w!(𝕯𝖎𝖘𝖈𝖎𝖚𝖒 0.1)
                WS_OVERLAPPEDWINDOW | WS_VISIBLE, // Fullscreen : WS_VISIBLE | WS_MAXIMIZE | WS_MAXIMIZEBOX | WS_POPUP
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                config.width as i32,
                config.height as i32,
                None,
                None,
                Some(hinstance),
                None,
            ) {
                Ok(h) => h,
                Err(e) => panic!("Window Creation Was failed : {e:?}"),
            }; // Soit un .unwrap() à la place du match 

            ShowWindow(hwnd, SW_SHOW).expect("Window Showing Failed !");

            return Self { 
                hwnd: hwnd, 
                hinstance: hinstance,
            };

        }

    }

    fn pool_events(&mut self) -> Vec<WindowEvent> {

        let mut event_list: Vec<WindowEvent> = Vec::new();

        unsafe {
            let mut msg = MSG::default();
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() {

                if msg.message == WM_QUIT {
                    event_list.push(WindowEvent::Exit);
                }

                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        let mut winproc_events = WINPROC_EVENTS.lock()
            .unwrap()
            .remove(&(self.hwnd.0 as isize))
            .unwrap_or_default();

        event_list.append(&mut winproc_events);

        return event_list;
    }

    fn get_raw_handle(&self) -> RawHandle {
        return RawHandle::Win32 { hwnd: self.hwnd.0 as isize, hinstance: self.hinstance.0 as isize }
    }

}


impl Win32Window {

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

                WM_SYSCOMMAND => {

                    let w_param = (wparam.0 & 0xFFFF) as u32;

                    if w_param == SC_RESTORE {
                        Self::push_event(hwnd.0 as isize, WindowEvent::Restored);
                    }

                    DefWindowProcW(hwnd, msg, wparam, lparam)

                }

                WM_SIZE => {

                    let resize_type = (wparam.0 & 0xFFFF) as u32;
                    let width = (lparam.0 & 0xFFFF) as u32;
                    let height = ((lparam.0 >> 16) & 0xFFFF) as u32;

                    match resize_type {

                        SIZE_MINIMIZED => { Self::push_event(hwnd.0 as isize, WindowEvent::Minimized); }
                        SIZE_RESTORED => {  },
                        //  => { Self::push_event(WindowEvent::Resize { width, height }); },

                        _ => {},
                    
                    }

                    LRESULT(0)
                }

                _ => DefWindowProcW(hwnd, msg, wparam, lparam)
            }
        }
    }

    fn push_event(key: isize, event: WindowEvent) {

        WINPROC_EVENTS.lock().unwrap()
            .entry(key)
            .or_default()
            .push(event);

    }

}