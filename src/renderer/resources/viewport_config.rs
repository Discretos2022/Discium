


pub struct ViewportConfig {

    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,

}


impl ViewportConfig {

    pub fn create(width: f32, height: f32) -> Self {
        return Self {
            x: 0.0,
            y: 0.0,
            width: width,
            height: height,
            min_depth: 0.0,
            max_depth: 1.0,
        }
    }

    pub fn x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        return self;
    }

    pub fn y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        return self;
    }

    pub fn width(&mut self, width: f32) -> &mut Self {
        self.width = width;
        return self;
    }

    pub fn height(&mut self, height: f32) -> &mut Self {
        self.y = height;
        return self;
    }

    pub fn min_depth(&mut self, min_depth: f32) -> &mut Self {
        self.min_depth = min_depth;
        return self;
    }

    pub fn max_depth(&mut self, max_depth: f32) -> &mut Self {
        self.max_depth = max_depth;
        return self;
    }

}