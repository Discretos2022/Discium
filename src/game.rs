use std::time::{Duration, Instant};

use crate::gameconfig::GameConfig;

use crate::gamebase::GameBase;
use crate::input::keyboard_input::KeyboardInput;
use crate::input::mouse_input::MouseInput;
use crate::renderer::renderer::Renderer;
use crate::renderer::renderer_type::RendererType;
use crate::window::event_enum::WindowEvent;
use crate::window::window::Window;

pub struct Game<T: GameBase> {

    pub base: T,
    pub game_config: GameConfig,

}


impl<T: GameBase> Game<T> {

    pub fn new(base: T, game_config: GameConfig) -> Self {

        let title = game_config.window_config.title;
        println!("{title}");

        return Self {
            base: base,
            game_config: game_config,
        };
    }

    pub fn start(&mut self) {
        
        let mut win = Window::create(&self.game_config.window_config);
        let mut vulkan = Renderer::create(RendererType::Vulkan, &win.get_raw_handle(), win.get_window_size());

        let mut window_events: Vec<WindowEvent> = vec![];

        let LOGIC_FRAME_TIME = Duration::from_nanos(16_666_667);
        let mut lastTime = Instant::now();
        let mut accumulator = Duration::from_millis(0);
        
        let mut running: bool = true;

        while running {

            let current_time = Instant::now();
            let delta_time = current_time - lastTime;
            lastTime = current_time;
            accumulator += delta_time;

            'logic_loop: while accumulator >= LOGIC_FRAME_TIME {

                window_events = win.pool_events();
                KeyboardInput::swap_state();
                MouseInput::swap_state();

                for e in window_events {
                
                    match e {
                        WindowEvent::Resize { width, height } => vulkan.update_surface_dimension((width, height)),
                        WindowEvent::Exit => { running = false; break 'logic_loop; },
                        WindowEvent::Minimized => { vulkan.pause(); },
                        WindowEvent::Restored => { vulkan.resume(); vulkan.update_surface_dimension(win.get_window_size()); },

                        WindowEvent::KeyPressed { keycode } => { KeyboardInput::update_pressed(keycode) },
                        WindowEvent::KeyReleased { keycode } => { KeyboardInput::update_released(keycode) },
                        
                        WindowEvent::MousePressed { button } => MouseInput::update_pressed(button),
                        WindowEvent::MouseReleased { button } => MouseInput::update_released(button),
                        WindowEvent::MousePosition { position } => MouseInput::update_position(position),
                    }

                }


                self.base.update();
                accumulator -= LOGIC_FRAME_TIME;
            }

            if !running { break; }

            // let max: u32 = System::get_max_memory();
            // println!("MEMORY : {} Bytes", max);

            // let alpha = accumulator.as_secs_f32() / LOGIC_FRAME_TIME.as_secs_f32();
            // Draw();
            vulkan.begin_draw();
            vulkan.draw_image();
            // vulkan.recreate_swapchain(&win.get_raw_handle());

        }

        println!("END OF PROGRAM !");

    }

}