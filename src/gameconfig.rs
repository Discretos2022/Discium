

pub struct GameConfig {

    pub title: String,
    pub width: u32,
    pub height: u32,
    // pub renderer: RendererConfig -> avec les extensions spécifique à vulkan, la version, etc...

}


impl Default for GameConfig {

    fn default() -> Self {
        Self {
            title: String::from("Discium Game"),
            width: 800,
            height: 600,
        }

    }

}