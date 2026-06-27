use crate::renderer::resources::sample_level::SampleLevel;



pub struct Multisampler {

    pub rasterization_sample_level: SampleLevel,

}


impl Multisampler {

    pub fn create() -> Self {
        return Self {
            rasterization_sample_level: SampleLevel::Type1,
        }
    }

    pub fn rasterization_sample_level(&mut self, level: SampleLevel) -> &mut Self {
        self.rasterization_sample_level = level;
        return self;
    }

}