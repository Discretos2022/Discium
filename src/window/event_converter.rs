
#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::*;

#[cfg(target_os = "windows")]
use crate::{input::keyboard_input::KeyCode};



pub struct EventConverter;

impl EventConverter {

    #[cfg(target_os = "windows")]
    pub fn key_event_to_key_code(virtual_key_code: VIRTUAL_KEY) -> KeyCode {

        match virtual_key_code {

            VK_RIGHT => { return KeyCode::Right }
            VK_LEFT => { return KeyCode::Left }
            VK_UP => { return KeyCode::Up }
            VK_DOWN => { return KeyCode::Down }

            VK_0 => { return KeyCode::Num0 }
            VK_1 => { return KeyCode::Num1 }
            VK_2 => { return KeyCode::Num2 }
            VK_3 => { return KeyCode::Num3 }
            VK_4 => { return KeyCode::Num4 }
            VK_5 => { return KeyCode::Num5 }
            VK_6 => { return KeyCode::Num6 }
            VK_7 => { return KeyCode::Num7 }
            VK_8 => { return KeyCode::Num8 }
            VK_9 => { return KeyCode::Num9 }

            VK_A => { return KeyCode::A }
            VK_B => { return KeyCode::B }
            VK_C => { return KeyCode::C }
            VK_D => { return KeyCode::D }
            VK_E => { return KeyCode::E }
            VK_F => { return KeyCode::F }
            VK_G => { return KeyCode::G }
            VK_H => { return KeyCode::H }
            VK_I => { return KeyCode::I }
            VK_J => { return KeyCode::J }
            VK_K => { return KeyCode::K }
            VK_L => { return KeyCode::L }
            VK_M => { return KeyCode::M }
            VK_N => { return KeyCode::N }
            VK_O => { return KeyCode::O }
            VK_P => { return KeyCode::P }
            VK_Q => { return KeyCode::Q }
            VK_R => { return KeyCode::R }
            VK_S => { return KeyCode::S }
            VK_T => { return KeyCode::T }
            VK_U => { return KeyCode::U }
            VK_V => { return KeyCode::V }
            VK_W => { return KeyCode::W }
            VK_X => { return KeyCode::X }
            VK_Y => { return KeyCode::Y }
            VK_Z => { return KeyCode::Z }

            _ => {}
            
        }

        return KeyCode::Unknow;

    }

}
