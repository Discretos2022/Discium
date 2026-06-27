use std::time::{Duration, Instant};

use crate::gameconfig::GameConfig;

use crate::gamebase::GameBase;
use crate::graphics_device::GraphicsDevice;
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
        
        let win = Window::create(&self.game_config.window_config);
        let vulkan = Renderer::create(RendererType::Vulkan, &win.get_raw_handle(), win.get_window_size());

        let mut graphics_device = GraphicsDevice {
            renderer: vulkan,
            window: win,
        };

        let mut window_events: Vec<WindowEvent> = vec![];

        let LOGIC_FRAME_TIME = Duration::from_nanos(16_666_667);
        let mut lastTime = Instant::now();
        let mut accumulator = Duration::from_millis(0);
        
        let mut running: bool = true;

        self.base.initialize(&mut graphics_device);

        while running {

            let current_time = Instant::now();
            let delta_time = current_time - lastTime;
            lastTime = current_time;
            accumulator += delta_time;

            'logic_loop: while accumulator >= LOGIC_FRAME_TIME {

                window_events = graphics_device.window.pool_events();
                KeyboardInput::swap_state();
                MouseInput::swap_state();

                for e in window_events {

                    let window = &mut graphics_device.window;
                    let renderer = &mut graphics_device.renderer;
                
                    match e {
                        WindowEvent::Resize { width, height } => renderer.update_surface_dimension((width, height)),
                        WindowEvent::Exit => { running = false; break 'logic_loop; },
                        WindowEvent::Minimized => { renderer.pause(); },
                        WindowEvent::Restored => { renderer.resume(); renderer.update_surface_dimension(window.get_window_size()); },

                        WindowEvent::KeyPressed { keycode } => { KeyboardInput::update_pressed(keycode) },
                        WindowEvent::KeyReleased { keycode } => { KeyboardInput::update_released(keycode) },
                        
                        WindowEvent::MousePressed { button } => MouseInput::update_pressed(button),
                        WindowEvent::MouseReleased { button } => MouseInput::update_released(button),
                        WindowEvent::MousePosition { position } => MouseInput::update_position(position),
                    }

                }

                self.base.update(&mut graphics_device);
                accumulator -= LOGIC_FRAME_TIME;
            }

            if !running { break; }

            // let alpha = accumulator.as_secs_f32() / LOGIC_FRAME_TIME.as_secs_f32();
            self.base.draw(&mut graphics_device);

        }

        println!("END OF PROGRAM !");

    }

}