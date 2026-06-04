use std::sync::{LazyLock, Mutex};



static MOUSE: LazyLock<Mutex<MouseInput>> = LazyLock::new(|| Mutex::new(MouseInput::new()));

pub struct MouseInput {
    
    old_state: Vec<MouseButton>,
    state: Vec<MouseButton>,

    position: (i32, i32),
    old_position: (i32, i32),

}


impl MouseInput {

    fn new() -> MouseInput {
        return Self { 
            old_state: Vec::new(),
            state: Vec::new(),
            position: (0, 0),
            old_position: (0, 0),
        }
    }

    pub fn update_pressed(button: MouseButton) {

        let mut mouse = MOUSE.lock().unwrap();
        if !mouse.state.contains(&button) {
            mouse.state.push(button);
        }

    }

    pub fn update_released(button: MouseButton) {

        let mut mouse = MOUSE.lock().unwrap();
        match mouse.state.iter().position(|b| b == &button) {
            Some(i) => { _ = mouse.state.swap_remove(i) },
            None => { return; },
        }

    }

    pub fn update_position(position: (i32, i32)) {
        let mut mouse = MOUSE.lock().unwrap();
        mouse.position = position;
    }

    pub fn swap_state() {
        let mut mouse = MOUSE.lock().unwrap();
        mouse.old_state = mouse.state.clone();
        mouse.old_position = mouse.position.clone();
    }

    pub fn is_button_down(button: MouseButton) -> bool {
        if MOUSE.lock().unwrap().state.contains(&button) {
            return true;
        }
        return false;
    }

    pub fn is_button_up(button: MouseButton) -> bool {
        if MOUSE.lock().unwrap().state.contains(&button) {
            return false;
        }
        return true;
    }

    pub fn is_button_clic(button: MouseButton) -> bool {
        let mouse = MOUSE.lock().unwrap();
        if mouse.state.contains(&button) && !mouse.old_state.contains(&button) {
            return true;
        }
        return false;
    }

    pub fn is_button_unclic(button: MouseButton) -> bool {
        let mouse = MOUSE.lock().unwrap();
        if !mouse.state.contains(&button) && mouse.old_state.contains(&button) {
            return true;
        }
        return false;
    }

    pub fn get_position() -> (i32, i32) {
        return MOUSE.lock().unwrap().position;
    }

}



#[derive(Clone, PartialEq)]
pub enum MouseButton {

    Right, Left, Middle,

    Unknown,

}