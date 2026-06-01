
#[cfg(target_os = "windows")]
use {

    std::collections::HashMap,
    std::sync::Mutex,

    windows::Win32::Graphics::Gdi::ScreenToClient,
    windows::Win32::System::LibraryLoader::GetModuleHandleW,
    windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM},
    windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY,
    windows::core::*,
    windows::Win32::UI::WindowsAndMessaging::*,

    crate::input::mouse_input::MouseButton,
    crate::window::basewindow::BaseWindow,
    crate::window::event_converter::{EventConverter},
    crate::window::event_enum::WindowEvent,
    crate::window::rawhandle::RawHandle,
    crate::window::window_config::WindowConfig,
};


#[cfg(target_os = "windows")]
pub struct Win32Window {

    pub hwnd: HWND,
    pub hinstance: HINSTANCE,
    cursor_position: POINT,

}

#[cfg(not(target_os = "windows"))]
pub struct Win32Window;

#[cfg(target_os = "windows")]
static WINPROC_EVENTS: std::sync::LazyLock<Mutex<HashMap<isize, Vec<WindowEvent>>>> = std::sync::LazyLock::new(||Mutex::new(HashMap::new()));

#[cfg(target_os = "windows")]
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
                cursor_position: POINT { x: 0, y: 0 },
            };

        }

    }

    fn pool_events(&mut self) -> Vec<WindowEvent> {

        let mut event_list: Vec<WindowEvent> = Vec::new();

        unsafe {
            let mut msg = MSG::default();
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() {

                let m: u32 = msg.message;
                let w_param = (msg.wParam.0 & 0xFFFF) as u32;

                match m {
                    WM_QUIT         => event_list.push(WindowEvent::Exit),
                    
                    WM_KEYDOWN      => event_list.push(WindowEvent::KeyPressed { keycode: EventConverter::key_event_to_key_code(VIRTUAL_KEY(w_param as u16)) }),
                    WM_KEYUP        => event_list.push(WindowEvent::KeyReleased { keycode: EventConverter::key_event_to_key_code(VIRTUAL_KEY(w_param as u16)) }),
                    
                    WM_RBUTTONDOWN  => event_list.push(WindowEvent::MousePressed { button: MouseButton::Right }),
                    WM_RBUTTONUP    => event_list.push(WindowEvent::MouseReleased { button: MouseButton::Right }),
                    WM_MBUTTONDOWN  => event_list.push(WindowEvent::MousePressed { button: MouseButton::Middle }),
                    WM_MBUTTONUP    => event_list.push(WindowEvent::MouseReleased { button: MouseButton::Middle }),
                    WM_LBUTTONDOWN  => event_list.push(WindowEvent::MousePressed { button: MouseButton::Left }),
                    WM_LBUTTONUP    => event_list.push(WindowEvent::MouseReleased { button: MouseButton::Left }),

                    _ => {},
                }

                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        
        unsafe {
            let _ = GetCursorPos(&mut self.cursor_position);
            let _ = ScreenToClient(self.hwnd, &mut self.cursor_position);
        };
        event_list.push(WindowEvent::MousePosition { position: (self.cursor_position.x, self.cursor_position.y) });


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

    fn get_window_size(&self) -> (u32, u32) {
        
        let mut rect: RECT = RECT::default();
        unsafe { GetClientRect(self.hwnd, &mut rect).expect("Windows : Get Client Rect Failed !") };
        return (
            (rect.right - rect.left) as u32,
            (rect.bottom - rect.top) as u32
        );

    }

}


#[cfg(target_os = "windows")]
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