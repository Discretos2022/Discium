use std::sync::{LazyLock, Mutex};


static KEYBOARD: LazyLock<Mutex<KeyboardInput>> = LazyLock::new(|| Mutex::new(KeyboardInput::new()));

pub struct KeyboardInput {
    
    old_state: Vec<KeyCode>,
    state: Vec<KeyCode>,

}


impl KeyboardInput {

    fn new() -> KeyboardInput {
        return Self { 
            old_state: Vec::new(),
            state: Vec::new(),
        }
    }

    pub fn update_pressed(key_code: KeyCode) {

        let mut keyboard = KEYBOARD.lock().unwrap();
        if !keyboard.state.contains(&key_code) {
            keyboard.state.push(key_code);
        }

    }

    pub fn update_released(key_code: KeyCode) {

        let mut keyboard = KEYBOARD.lock().unwrap();
        match keyboard.state.iter().position(|k| k == &key_code) {
            Some(i) => { _ = keyboard.state.swap_remove(i) },
            None => { return; },
        }

    }

    pub fn swap_state() {
        let mut keyboard = KEYBOARD.lock().unwrap();
        keyboard.old_state = keyboard.state.clone();
    }

    pub fn is_key_down(key_code: KeyCode) -> bool {
        if KEYBOARD.lock().unwrap().state.contains(&key_code) {
            return true;
        }
        return false;
    }

    pub fn is_key_up(key_code: KeyCode) -> bool {
        if KEYBOARD.lock().unwrap().state.contains(&key_code) {
            return false;
        }
        return true;
    }

    pub fn is_key_clic(key_code: KeyCode) -> bool {
        let keyboard = KEYBOARD.lock().unwrap();
        if keyboard.state.contains(&key_code) && !keyboard.old_state.contains(&key_code) {
            return true;
        }
        return false;
    }

    pub fn is_key_unclic(key_code: KeyCode) -> bool {
        let keyboard = KEYBOARD.lock().unwrap();
        if !keyboard.state.contains(&key_code) && !keyboard.old_state.contains(&key_code) {
            return true;
        }
        return false;
    }
    

}


#[derive(Clone, PartialEq)]
pub enum KeyCode {

    Right, Left, Up, Down,

    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    
    Q, W, E, R, T, Z, U, I, O, P, A, S, D, F, G, H, J, K, L, Y, X, C, V, B, N, M,

    Unknow,

}
