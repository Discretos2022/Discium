use crate::window::window_config::WindowConfig;



pub struct GameConfig {

    pub window_config: WindowConfig,
    // pub renderer: RendererConfig -> avec les extensions spécifique à vulkan, la version, etc...

}


impl Default for GameConfig {

    fn default() -> Self {
        Self {
            window_config: WindowConfig::default(),
        }

    }

}