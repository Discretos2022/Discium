use crate::renderer::resources::blend_mode::BlendMode;




pub struct BlendState {

    pub blend_states: Vec<BlendMode>,

}


impl BlendState {

    pub fn create() -> Self {
        return Self {
            blend_states: Vec::new(),
        }
    }

    pub fn blend_states(mut self, blend_modes: Vec<BlendMode>) -> Self {
        self.blend_states = blend_modes;
        return self;
    }

}