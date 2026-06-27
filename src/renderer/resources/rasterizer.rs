use crate::renderer::resources::cull_mode::CullMode;
use crate::renderer::resources::front_mode::FrontMode;
use crate::renderer::resources::fill_mode::FillMode;


pub struct Rasterizer {

    pub fill_mode: FillMode,
    pub front_mode: FrontMode,
    pub cull_mode: CullMode,

}


impl Rasterizer {

    pub fn create() -> Self {
        return Self {
            fill_mode: FillMode::Fill,
            front_mode: FrontMode::ClockWise,
            cull_mode: CullMode::Back
        }
    }

    pub fn fill_mode(&mut self, fill_mode: FillMode) -> &mut Self {
        self.fill_mode = fill_mode;
        return self;
    }

    pub fn front_mode(&mut self, front_mode: FrontMode) -> &mut Self {
        self.front_mode = front_mode;
        return self;
    }

    pub fn cull_mode(&mut self, cull_mode: CullMode) -> &mut Self {
        self.cull_mode = cull_mode;
        return self;
    }

}