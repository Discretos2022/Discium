
use {
    crate::{window::{basewindow::BaseWindow, event_converter::EventConverter, event_enum::WindowEvent, rawhandle::RawHandle, window_config::WindowConfig}},
    std::ffi::c_void,
    wayland_client::{Connection, Dispatch, Proxy, QueueHandle, protocol::{wl_compositor, wl_display::WlDisplay, wl_keyboard, wl_pointer, wl_registry, wl_seat, wl_surface::{self, WlSurface}}},
    wayland_protocols::xdg::shell::client::*,
    xkbcommon::xkb,
};



pub struct WaylandWindow {

    pub display: WlDisplay,
    pub surface: WlSurface,
    event_queue: wayland_client::EventQueue<AppData>,
    app_data: AppData,

}


impl BaseWindow for WaylandWindow {

    fn create(config: &WindowConfig) -> Self {


        let connection = Connection::connect_to_env().unwrap();

        let display = connection.display();

        let mut event_queue: wayland_client::EventQueue<AppData> = connection.new_event_queue();

        let qh = event_queue.handle();

        let registry = display.get_registry(&qh, ());

        let mut app_data: AppData = AppData { compositor: None, xdg_wm_base: None, seat: None, keyboard: None, pointer: None, xkb_context: None, xkb_state: None, window_size: (config.width, config.height), configured: false, event_list: Vec::new() };

        event_queue.roundtrip(&mut app_data).unwrap();

        let surface = app_data.compositor.as_ref().unwrap().create_surface(&qh, ());

        let xdg_surface = app_data.xdg_wm_base.as_ref().unwrap().get_xdg_surface(&surface, &qh, ());
        let xdg_toplevel = xdg_surface.get_toplevel(&qh, ());
        xdg_toplevel.set_title(config.title.to_string());
        surface.commit();

        // while !app_data.configured {
            event_queue.roundtrip(&mut app_data).unwrap();
        // }

        return Self {
            display: display,
            surface: surface,
            event_queue: event_queue,
            app_data: app_data,
        };

    }

    fn pool_events(&mut self) -> Vec<WindowEvent> {

        let WaylandWindow { event_queue, app_data, .. } = self;
        event_queue.dispatch_pending(app_data).unwrap();
        event_queue.flush().unwrap();

        return std::mem::take(&mut app_data.event_list);
    }

    fn get_raw_handle(&self) -> RawHandle {
        return RawHandle::Wayland {
            display: self.display.id().as_ptr() as *mut c_void,
            surface: self.surface.id().as_ptr() as *mut c_void,
        };
    }
    
    fn get_window_size(&self) -> (u32, u32) {
        self.app_data.window_size
    }

}


struct AppData {
    compositor: Option<wl_compositor::WlCompositor>,
    xdg_wm_base: Option<xdg_wm_base::XdgWmBase>,
    seat: Option<wl_seat::WlSeat>,
    keyboard: Option<wl_keyboard::WlKeyboard>,
    pointer: Option<wl_pointer::WlPointer>,
    xkb_context: Option<xkb::Context>,
    xkb_state: Option<xkb::State>,
    configured: bool,
    window_size: (u32, u32),
    event_list: Vec<WindowEvent>,
}


impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<AppData>,
    ) {
        // When receiving events from the wl_registry, we are only interested in the
        // `global` event, which signals a new available global.
        // When receiving this event, we just print its characteristics in this example.
        if let wl_registry::Event::Global { name, interface, version } = event {

            match interface.as_str() {
                
                "wl_compositor" => {

                    let compositor = registry.bind::<wl_compositor::WlCompositor, _, _>(name, version, qh, ());
                    state.compositor = Some(compositor);

                }

                "xdg_wm_base" => {

                    let xdg_wm_base = registry.bind::<xdg_wm_base::XdgWmBase, _, _>(name, version, qh, ());
                    state.xdg_wm_base = Some(xdg_wm_base);

                }

                "wl_seat" => {
                    let seat = registry.bind::<wl_seat::WlSeat, _, _>(name, version, qh, ());
                    state.seat = Some(seat);
                }

                _ => {}
 
            }

            println!("[{}] {} (v{})", name, interface, version);
        }
    }
}


impl Dispatch<wl_compositor::WlCompositor, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &wl_compositor::WlCompositor,
        _: wl_compositor::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        
    }
}


#[cfg(target_os = "linux")]
impl Dispatch<wl_surface::WlSurface, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &wl_surface::WlSurface,
        _: wl_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        
    }
}


impl Dispatch<xdg_wm_base::XdgWmBase, ()> for AppData {
    fn event(
        _: &mut Self,
        xdg_wm_base: &xdg_wm_base::XdgWmBase,
        event: xdg_wm_base::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        
        if let xdg_wm_base::Event::Ping { serial } = event {
            xdg_wm_base.pong(serial);
        }

    }
}


impl Dispatch<xdg_surface::XdgSurface, ()> for AppData {
    fn event(
        state: &mut Self,
        xdg_surface: &xdg_surface::XdgSurface,
        event: xdg_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        
        if let xdg_surface::Event::Configure { serial } = event {
            xdg_surface.ack_configure(serial);
            state.configured = true;
        }

    }
}


impl Dispatch<xdg_toplevel::XdgToplevel, ()> for AppData {
    fn event(
        state: &mut Self,
        _: &xdg_toplevel::XdgToplevel,
        event: xdg_toplevel::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {

        if let xdg_toplevel::Event::Close = event {
            state.event_list.push(WindowEvent::Exit);
        }

        if let xdg_toplevel::Event::Configure { width, height, states } = event {

            if width == 0 || height == 0 {
                // Le compositeur laisse le choix, on garde notre taille
            } else {
                state.window_size = (width as u32, height as u32);
                state.event_list.push(WindowEvent::Resize { width: width as u32, height: height as u32 });
            }

        }

    }
}


impl Dispatch<wl_seat::WlSeat, ()> for AppData {
    fn event(
        state: &mut Self,
        seat: &wl_seat::WlSeat,
        event: wl_seat::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<AppData>,
    ) {

        match event {
            wl_seat::Event::Capabilities { capabilities } => {

                if let wayland_client::WEnum::Value(c) = capabilities {

                    if c.contains(wl_seat::Capability::Keyboard) {
                        state.keyboard = Some(seat.get_keyboard(qh, ()));
                    }

                    if c.contains(wl_seat::Capability::Pointer) {
                        state.pointer = Some(seat.get_pointer(qh, ()));
                    }

                }

            },
            
            _ => {},
        }

    }
}


impl Dispatch<wl_keyboard::WlKeyboard, ()> for AppData {
    fn event(
        app_data: &mut Self,
        keyboard: &wl_keyboard::WlKeyboard,
        event: wl_keyboard::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        
        match event {

            wl_keyboard::Event::Keymap { format, fd, size } => {

                let context = xkb::Context::new(xkb::STATE_NO_FLAGS);

                let keymap = unsafe {
                    xkb::Keymap::new_from_fd(&context, fd, size as usize, xkb::KEYMAP_FORMAT_TEXT_V1, xkb::KEYMAP_COMPILE_NO_FLAGS).expect("Linux : Keymap Creation Failed !")
                };

                app_data.xkb_state = Some(xkb::State::new(&keymap.unwrap()));
            },

            wl_keyboard::Event::Enter { serial, surface, keys } => {},
            wl_keyboard::Event::Leave { serial, surface } => {},
            
            wl_keyboard::Event::Key { serial, time, key, state } => {

                if app_data.xkb_state.is_some() {

                    let keycode = xkb::Keycode::from(key + 8);
                    let keysym = app_data.xkb_state.as_ref().unwrap().key_get_one_sym(keycode);

                    match state {
                        wayland_client::WEnum::Value(key_state) => {
                            match key_state {
                                wl_keyboard::KeyState::Pressed => { app_data.event_list.push(WindowEvent::KeyPressed { keycode: EventConverter::key_event_to_key_code(keysym) }) },
                                wl_keyboard::KeyState::Released => { app_data.event_list.push(WindowEvent::KeyReleased { keycode: EventConverter::key_event_to_key_code(keysym) }) },
                                wl_keyboard::KeyState::Repeated => {},
                                _ => {},
                            }
                        },
                        wayland_client::WEnum::Unknown(_) => {},
                    }

                }


            },

            wl_keyboard::Event::Modifiers { serial, mods_depressed, mods_latched, mods_locked, group } => {},
            wl_keyboard::Event::RepeatInfo { rate, delay } => {},
            _ => {},
        }

    }
}


impl Dispatch<wl_pointer::WlPointer, ()> for AppData {
    fn event(
        app_data: &mut Self,
        pointer: &wl_pointer::WlPointer,
        event: wl_pointer::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {

        match event {
            wl_pointer::Event::Enter { serial, surface, surface_x, surface_y } => {},
            wl_pointer::Event::Leave { serial, surface } => {},
            wl_pointer::Event::Motion { time, surface_x, surface_y } => {
                app_data.event_list.push(WindowEvent::MousePosition { position: (surface_x.round() as i32, surface_y.round() as i32) });
            },
            wl_pointer::Event::Button { serial, time, button, state } => {
                match state {
                    wayland_client::WEnum::Value(button_state) => {
                        match button_state {
                            wl_pointer::ButtonState::Pressed => app_data.event_list.push(WindowEvent::MousePressed { button: EventConverter::mouse_code_to_mouse_button(button) }),
                            wl_pointer::ButtonState::Released => app_data.event_list.push(WindowEvent::MouseReleased { button: EventConverter::mouse_code_to_mouse_button(button) }),
                            _ => {},
                        }
                    },
                    wayland_client::WEnum::Unknown(_) => {},
                }
            },
            wl_pointer::Event::Axis { time, axis, value } => {},
            wl_pointer::Event::Frame => {},
            wl_pointer::Event::AxisSource { axis_source } => {},
            wl_pointer::Event::AxisStop { time, axis } => {},
            wl_pointer::Event::AxisDiscrete { axis, discrete } => {},
            wl_pointer::Event::AxisValue120 { axis, value120 } => {},
            wl_pointer::Event::AxisRelativeDirection { axis, direction } => {},
            _ => {},
        }
        
    }
}