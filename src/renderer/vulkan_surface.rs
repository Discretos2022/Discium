use ash::vk;

use crate::window::rawhandle::RawHandle;


pub struct VulkanSurface;

// Base
impl VulkanSurface {

    pub fn get_required_extensions(raw_handle: &RawHandle) -> Vec<*const i8> {

        let mut extensions = vec![ash::khr::surface::NAME.as_ptr()];
        let mut platform_extension = Self::get_plateform_extension(raw_handle);
        extensions.append(&mut platform_extension);
        return extensions;

    }

    pub fn create_vk_surface(entry: &ash::Entry, instance: &ash::Instance, raw_handle: &RawHandle) -> (vk::SurfaceKHR, ash::khr::surface::Instance) {

        let surface =  Self::create_surface(entry, instance, raw_handle);
        let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);

        return (surface, surface_loader);

    }

}

// Platform
impl VulkanSurface {

    #[cfg(target_os = "windows")]
    fn create_surface(entry: &ash::Entry, instance: &ash::Instance, raw_handle: &RawHandle) -> vk::SurfaceKHR {
        
        let RawHandle::Win32 { hwnd, hinstance } = raw_handle else { panic!("RawHandle Is Not Valid !") };

        let create_info = vk::Win32SurfaceCreateInfoKHR::default()
            .hwnd(*hwnd as isize)
            .hinstance(*hinstance as isize);

        let loader = ash::khr::win32_surface::Instance::new(entry, instance);

        let surface = unsafe { loader.create_win32_surface(&create_info, None).expect("Vulkan Win32 Surface Creation Error") };

        return surface;

    }

    #[cfg(target_os = "windows")]
    fn get_plateform_extension(_raw_handle: &RawHandle) -> Vec<*const i8> {
        return vec![ash::khr::win32_surface::NAME.as_ptr()];
    }


    #[cfg(target_os = "linux")]
    fn create_surface(entry: &ash::Entry, instance: &ash::Instance, raw_handle: &RawHandle) -> vk::SurfaceKHR {

        match raw_handle {
            RawHandle::Wayland {display, surface} => {

                let loader = ash::khr::wayland_surface::Instance::new(entry, instance);

                let create_info = vk::WaylandSurfaceCreateInfoKHR::default()
                    .display(*display)
                    .surface(*surface);

                let surface = unsafe { loader.create_wayland_surface(&create_info, None).expect("Vulkan Wayland Surface Creation Error") };

                return surface;

            },
            _ => panic!("RawHandle Is Not Valid For Linux !"),
        };

    }

    #[cfg(target_os = "linux")]
    fn get_plateform_extension(raw_handle: &RawHandle) -> Vec<*const i8> {

        match raw_handle {
            RawHandle::Wayland { .. } => return vec![ash::khr::wayland_surface::NAME.as_ptr()],
            _ => panic!("RawHandle Is Not Valid For Linux !"),
        }

    }

}