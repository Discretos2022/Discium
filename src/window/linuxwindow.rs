
#[cfg(target_os = "linux")]
use {

    std::ffi::c_void,

    wayland_client::{Connection, Dispatch, QueueHandle, Proxy, protocol::{wl_compositor, wl_display::WlDisplay, wl_registry, wl_surface::{self, WlSurface}}},
    wayland_protocols::xdg::shell::client::*,

    crate::window::{basewindow::BaseWindow, event_enum::WindowEvent, rawhandle::RawHandle, window_config::WindowConfig},
};





#[cfg(target_os = "linux")]
pub struct LinuxWindow {

    pub display: WlDisplay,
    pub surface: WlSurface,
    event_queue: wayland_client::EventQueue<AppData>,
    app_data: AppData,

}

#[cfg(not(target_os = "linux"))]
pub struct LinuxWindow;

#[cfg(target_os = "linux")]
impl BaseWindow for LinuxWindow {

    fn create(config: &WindowConfig) -> Self {


        let connection = Connection::connect_to_env().unwrap();

        let display = connection.display();

        let mut event_queue: wayland_client::EventQueue<AppData> = connection.new_event_queue();

        let qh = event_queue.handle();

        let registry = display.get_registry(&qh, ());

        let mut app_data: AppData = AppData { compositor: None, xdg_wm_base: None, window_size: (config.width, config.height), configured: false, event_list: Vec::new() };

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

        let LinuxWindow { event_queue, app_data, .. } = self;
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


#[cfg(target_os = "linux")]
struct AppData {
    compositor: Option<wl_compositor::WlCompositor>,
    xdg_wm_base: Option<xdg_wm_base::XdgWmBase>,
    configured: bool,
    window_size: (u32, u32),
    event_list: Vec<WindowEvent>,
}


#[cfg(target_os = "linux")]
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

                _ => {}
 
            }

            println!("[{}] {} (v{})", name, interface, version);
        }
    }
}


#[cfg(target_os = "linux")]
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


#[cfg(target_os = "linux")]
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


#[cfg(target_os = "linux")]
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


#[cfg(target_os = "linux")]
impl Dispatch<xdg_toplevel::XdgToplevel, ()> for AppData {
    fn event(
        state: &mut Self,
        _: &xdg_toplevel::XdgToplevel,
        event: xdg_toplevel::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {

        if let xdg_toplevel::Event::Configure { width, height, states } = event {

            if width == 0 || height == 0 {
                // Le compositeur laisse le choix, on garde notre taille
            } else {
                state.window_size = (width as u32, height as u32);
            }

        }

    }
}