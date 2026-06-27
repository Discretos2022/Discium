

pub struct ScissorConfig {

    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,

}


impl ScissorConfig {

    pub fn create(width: u32, height: u32) -> Self {
        return Self {
            x: 0,
            y: 0,
            width: width,
            height: height
        }
    }

    pub fn x(&mut self, x: i32) -> &mut Self {
        self.x = x;
        return self;
    }

    pub fn y(&mut self, y: i32) -> &mut Self {
        self.y = y;
        return self;
    }

    pub fn width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        return self;
    }

    pub fn height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        return self;
    }

}