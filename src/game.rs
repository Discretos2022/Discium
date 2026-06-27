use std::time::{Duration, Instant};

use glam::{Mat4, Vec3};

use crate::gameconfig::GameConfig;

use crate::gamebase::GameBase;
use crate::graphics_device::GraphicsDevice;
use crate::input::keyboard_input::KeyboardInput;
use crate::input::mouse_input::MouseInput;
use crate::renderer::renderer::Renderer;
use crate::renderer::renderer_type::RendererType;
// use crate::renderer::resource_handles::{IndexBufferHandle, VertexBufferHandle};
// use crate::renderer::resources::base_vertex::BaseVertex;
// use crate::renderer::resources::blend_mode::BlendMode;
// use crate::renderer::resources::blend_state::BlendState;
// use crate::renderer::resources::descriptor_binding::DescriptorBinding;
// use crate::renderer::resources::descriptor_type::DescriptorType;
// use crate::renderer::resources::pipeline_config::PipelineConfig;
// use crate::renderer::resources::scissor_config::ScissorConfig;
// use crate::renderer::resources::shader_type::ShaderType;
// use crate::renderer::resources::uniform_mvp::UniformMVP;
// use crate::renderer::resources::vertex_position_color::VertexPositionColor;
// use crate::renderer::resources::viewport_config::ViewportConfig;
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

        let mut graphics_device = GraphicsDevice {
            renderer: vulkan,
            window: win,
        };

        let mut window_events: Vec<WindowEvent> = vec![];

        let LOGIC_FRAME_TIME = Duration::from_nanos(16_666_667);
        let mut lastTime = Instant::now();
        let mut accumulator = Duration::from_millis(0);
        
        let mut running: bool = true;

        let start_time =  std::time::Instant::now();

        // let vertex_buffer1: VertexBufferHandle<VertexPositionColor> = vulkan.create_vertex_buffer(4);
        // let vertices: &[VertexPositionColor] = &[
        //     VertexPositionColor { pos: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
        //     VertexPositionColor { pos: [ 0.5, -0.5], color: [0.0, 1.0, 0.0] },
        //     VertexPositionColor { pos: [ 0.5,  0.5], color: [0.0, 0.0, 1.0] },
        //     VertexPositionColor { pos: [-0.5,  0.5], color: [1.0, 1.0, 1.0] },
        // ];
        // vulkan.set_vertex_buffer_data(vertex_buffer1, vertices);


        // let index_buffer1: IndexBufferHandle<u16> = vulkan.create_index_buffer(6);
        // let indices: &[u16] = &[0, 1, 2, 2, 3, 0];
        // vulkan.set_index_buffer_data(index_buffer1, indices);



        // let vertex_buffer2: VertexBufferHandle<VertexPositionColor> = vulkan.create_vertex_buffer(4);
        // let vertices: &[VertexPositionColor] = &[
        //     VertexPositionColor { pos: [-0.5 + 1.0, -0.5 + 1.0], color: [1.0, 0.0, 0.0] },
        //     VertexPositionColor { pos: [ 0.5 + 1.0, -0.5 + 1.0], color: [0.0, 1.0, 0.0] },
        //     VertexPositionColor { pos: [ 0.5 + 1.0,  0.5 + 1.0], color: [0.0, 0.0, 1.0] },
        //     VertexPositionColor { pos: [-0.5 + 1.0,  0.5 + 1.0], color: [1.0, 1.0, 1.0] },
        // ];
        // vulkan.set_vertex_buffer_data(vertex_buffer2, vertices);


        // let index_buffer2: IndexBufferHandle<u16> = vulkan.create_index_buffer(6);
        // let indices: &[u16] = &[0, 1, 2, 2, 3, 0];
        // vulkan.set_index_buffer_data(index_buffer2, indices);


        // let vertex_buffer3: VertexBufferHandle<VertexPositionColor> = vulkan.create_vertex_buffer(4);
        // let vertices: &[VertexPositionColor] = &[
        //     VertexPositionColor { pos: [-0.5 - 1.0, -0.5 - 1.0], color: [1.0, 0.0, 0.0] },
        //     VertexPositionColor { pos: [ 0.5 - 1.0, -0.5 - 1.0], color: [0.0, 1.0, 0.0] },
        //     VertexPositionColor { pos: [ 0.5 - 1.0,  0.5 - 1.0], color: [0.0, 0.0, 1.0] },
        //     VertexPositionColor { pos: [-0.5 - 1.0,  0.5 - 1.0], color: [1.0, 1.0, 1.0] },
        // ];
        // vulkan.set_vertex_buffer_data(vertex_buffer3, vertices);


        // let index_buffer3: IndexBufferHandle<u16> = vulkan.create_index_buffer(6);
        // let indices: &[u16] = &[0, 1, 2, 2, 3, 0];
        // vulkan.set_index_buffer_data(index_buffer3, indices);


        // let vertex_shader = vulkan.create_shader("C:/Users/Joshua/Documents/Joshua Siedel/Programmation Rust/discium/discium/src/renderer/shaders/bin/vert.spv", ShaderType::Vertex);
        // let fragment_shader = vulkan.create_shader("C:/Users/Joshua/Documents/Joshua Siedel/Programmation Rust/discium/discium/src/renderer/shaders/bin/frag.spv", ShaderType::Fragment);


        // let blend = BlendState::create().blend_states(vec![BlendMode::AlphaBlend]);
        // let descriptor_bindings = DescriptorBinding { binding: 0, descriptor_type: DescriptorType::UniformBuffer, shader_type: ShaderType::Vertex };

        // let shader_layout = vulkan.create_shader_layout(vec![descriptor_bindings]);

        // let pipeline_config = PipelineConfig::create([vertex_shader, fragment_shader].to_vec(), VertexPositionColor::get_vertex_declaration(), shader_layout)
        //     .blend_state(blend);

        // let pipeline = vulkan.create_pipeline(pipeline_config);

        // let viewport_config = ViewportConfig::create(win.get_window_size().0 as f32, win.get_window_size().1 as f32);
        // let scissor_config = ScissorConfig::create(win.get_window_size().0 as u32, win.get_window_size().1 as u32);

        // let viewport = vulkan.create_viewport(viewport_config);
        // let scissor = vulkan.create_scissor(scissor_config);

        // let uniform_buffer = vulkan.create_uniform_buffer::<UniformMVP>(shader_layout);


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

            // let max: u32 = System::get_max_memory();
            // println!("MEMORY : {} Bytes", max);

            // let alpha = accumulator.as_secs_f32() / LOGIC_FRAME_TIME.as_secs_f32();
            self.base.draw(&mut graphics_device);

            // Test du cycle de dessin



            // let time = start_time.elapsed().as_secs_f32();
            // let ubo = UniformMVP {
            //     model: Mat4::from_rotation_z(time * 90.0f32.to_radians()) * Mat4::from_rotation_y(time * 90.0f32.to_radians()) * Mat4::from_rotation_x(-time * 90.0f32.to_radians()),
            //     // 2D
            //     //model: Mat4::IDENTITY,
            //     view: Mat4::look_at_rh(
            //     Vec3::new(2.0, 2.0, 2.0),
            //     Vec3::new(0.0, 0.0, 0.0),
            //     Vec3::new(0.0, 0.0, 1.0)
            //     ),
            //     // 2D
            //     //view: Mat4::IDENTITY,
            //     proj: {
            //         let mut proj = Mat4::perspective_rh(
            //         45.0f32.to_radians(),
            //         win.get_window_size().0 as f32 / win.get_window_size().1 as f32,
            //         0.1,
            //         10.0
            //         );
            //         // 2D
            //         // let mut proj = Mat4::orthographic_rh(
            //         // 0.0,
            //         // self.swapchain_extent.width as f32,
            //         // self.swapchain_extent.height as f32, 
            //         // 0.0,
            //         // -1.0,
            //         // 1.0,
            //         // );
            //         proj.y_axis.y *= -1.0; // à supprimer pour la 2D
            //         proj
            //     }
            // };






            // vulkan.begin_draw();
            // vulkan.set_pipeline(pipeline);
            // vulkan.set_uniform_buffer(uniform_buffer);
            // vulkan.set_viewport(viewport);
            // vulkan.set_scissor(scissor);

            // vulkan.set_uniform_buffer_data(uniform_buffer, ubo);
            // vulkan.draw_image();

            // vulkan.draw_indexed(vertex_buffer1, index_buffer1);

            // if KeyboardInput::is_key_down(KeyCode::Left) {
            //     vulkan.draw_indexed(vertex_buffer2, index_buffer2);
            // }

            // if KeyboardInput::is_key_down(KeyCode::Right) {
            //     vulkan.draw_indexed(vertex_buffer3, index_buffer3);
            // }

            // vulkan.end_draw();
            //-------------------------------------------------------------------------------------------------

        }

        println!("END OF PROGRAM !");

    }

}