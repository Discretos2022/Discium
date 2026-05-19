use std::time::{Duration, Instant};

use crate::gameconfig::GameConfig;

use crate::gamebase::GameBase;
use crate::renderer::baserenderer::BaseRenderer;
use crate::renderer::vulkanrenderer::VulkanRenderer;
use crate::window::windowfactory::WindowFactory;
use crate::utils::system::System;

pub struct Game<T: GameBase> {

    pub base: T,

}


impl<T: GameBase> Game<T> {

    pub fn new(base: T, config: GameConfig) -> Self {

        let title = config.title;
        println!("{title}");

        return Self {
            base: base,
        };
    }

    pub fn start(&mut self) {
        
        let win = WindowFactory::create();

        let mut vulkan = VulkanRenderer::create(&win.get_raw_handle());

        let LOGIC_FRAME_TIME = Duration::from_nanos(16_666_667);
        let mut lastTime = Instant::now();
        let mut accumulator = Duration::from_millis(0);
        
        let running: bool = true;

        while running {

            if !win.pool_events() {
                break;
            }

            let currentTime = Instant::now();
            let deltaTime = currentTime - lastTime;
            lastTime = currentTime;
            accumulator += deltaTime;
            
            while accumulator >= LOGIC_FRAME_TIME {
                self.base.update();
                accumulator -= LOGIC_FRAME_TIME;
            }

            let max: u32 = System::get_max_memory();
            println!("MEMORY : {} Bytes", max);

            // let alpha = accumulator.as_secs_f32() / LOGIC_FRAME_TIME.as_secs_f32();
            // Draw();
            vulkan.draw_image();

        }

        println!("END OF PROGRAM !");

    }

}