use crate::input::{keyboard_input::KeyCode, mouse_input::MouseButton};


pub enum WindowEvent {

    Resize { width: u32, height: u32 },
    Exit,
    Minimized,
    Restored,

    KeyPressed { keycode: KeyCode },
    KeyReleased { keycode: KeyCode },

    MousePressed { button: MouseButton },
    MouseReleased { button: MouseButton },

    MousePosition { position: (i32, i32) },

}