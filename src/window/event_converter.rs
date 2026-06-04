
#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::*;

#[cfg(target_os = "linux")]
use xkbcommon::xkb;


use crate::{input::keyboard_input::KeyCode};
use crate::input::mouse_input::MouseButton;



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

        return KeyCode::Unknown;

    }


    #[cfg(target_os = "linux")]
    pub fn key_event_to_key_code(keysym: xkb::Keysym) -> KeyCode {

        match keysym.raw() {

            xkb::keysyms::KEY_Right => { return KeyCode::Right }
            xkb::keysyms::KEY_Left => { return KeyCode::Left }
            xkb::keysyms::KEY_Up => { return KeyCode::Up }
            xkb::keysyms::KEY_Down => { return KeyCode::Down }

            xkb::keysyms::KEY_0 => { return KeyCode::Num0 }
            xkb::keysyms::KEY_1 => { return KeyCode::Num1 }
            xkb::keysyms::KEY_2 => { return KeyCode::Num2 }
            xkb::keysyms::KEY_3 => { return KeyCode::Num3 }
            xkb::keysyms::KEY_4 => { return KeyCode::Num4 }
            xkb::keysyms::KEY_5 => { return KeyCode::Num5 }
            xkb::keysyms::KEY_6 => { return KeyCode::Num6 }
            xkb::keysyms::KEY_7 => { return KeyCode::Num7 }
            xkb::keysyms::KEY_8 => { return KeyCode::Num8 }
            xkb::keysyms::KEY_9 => { return KeyCode::Num9 }

            xkb::keysyms::KEY_a | xkb::keysyms::KEY_A => { return KeyCode::A }
            xkb::keysyms::KEY_b | xkb::keysyms::KEY_B => { return KeyCode::B }
            xkb::keysyms::KEY_c | xkb::keysyms::KEY_C => { return KeyCode::C }
            xkb::keysyms::KEY_d | xkb::keysyms::KEY_D => { return KeyCode::D }
            xkb::keysyms::KEY_e | xkb::keysyms::KEY_E => { return KeyCode::E }
            xkb::keysyms::KEY_f | xkb::keysyms::KEY_F => { return KeyCode::F }
            xkb::keysyms::KEY_g | xkb::keysyms::KEY_G => { return KeyCode::G }
            xkb::keysyms::KEY_h | xkb::keysyms::KEY_H => { return KeyCode::H }
            xkb::keysyms::KEY_i | xkb::keysyms::KEY_I => { return KeyCode::I }
            xkb::keysyms::KEY_j | xkb::keysyms::KEY_J => { return KeyCode::J }
            xkb::keysyms::KEY_k | xkb::keysyms::KEY_K => { return KeyCode::K }
            xkb::keysyms::KEY_l | xkb::keysyms::KEY_L => { return KeyCode::L }
            xkb::keysyms::KEY_m | xkb::keysyms::KEY_M => { return KeyCode::M }
            xkb::keysyms::KEY_n | xkb::keysyms::KEY_N => { return KeyCode::N }
            xkb::keysyms::KEY_o | xkb::keysyms::KEY_O => { return KeyCode::O }
            xkb::keysyms::KEY_p | xkb::keysyms::KEY_P => { return KeyCode::P }
            xkb::keysyms::KEY_q | xkb::keysyms::KEY_Q => { return KeyCode::Q }
            xkb::keysyms::KEY_r | xkb::keysyms::KEY_R => { return KeyCode::R }
            xkb::keysyms::KEY_s | xkb::keysyms::KEY_S => { return KeyCode::S }
            xkb::keysyms::KEY_t | xkb::keysyms::KEY_T => { return KeyCode::T }
            xkb::keysyms::KEY_u | xkb::keysyms::KEY_U => { return KeyCode::U }
            xkb::keysyms::KEY_v | xkb::keysyms::KEY_V => { return KeyCode::V }
            xkb::keysyms::KEY_w | xkb::keysyms::KEY_W => { return KeyCode::W }
            xkb::keysyms::KEY_x | xkb::keysyms::KEY_X => { return KeyCode::X }
            xkb::keysyms::KEY_y | xkb::keysyms::KEY_Y => { return KeyCode::Y }
            xkb::keysyms::KEY_z | xkb::keysyms::KEY_Z => { return KeyCode::Z }

            _ => {}
            
        }

        return KeyCode::Unknown;

    }


    #[cfg(target_os = "linux")]
    pub fn  mouse_code_to_mouse_button(code: u32) -> MouseButton {

        match code {
            
            272 => MouseButton::Left,
            273 => MouseButton::Right,
            274 => MouseButton::Middle,

            _ => MouseButton::Unknown
        }

    }

}
