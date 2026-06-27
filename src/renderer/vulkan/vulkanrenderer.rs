use std::{collections::HashSet, ffi::CStr};

use ash;
use ash::vk;

use crate::renderer::resource_handles::{ScissorHandle, ShaderLayoutHandle, UniformBufferHandle, ViewportHandle};
use crate::renderer::resources::base_uniform::BaseUniform;
use crate::renderer::resources::descriptor_binding::DescriptorBinding;
use crate::renderer::resources::scissor_config::ScissorConfig;
use crate::renderer::vulkan::vulkan_descriptor_set_layout::VulkanDescriptorSetLayout;
use crate::renderer::vulkan::vulkan_uniformbuffer::VulkanUniformBuffer;
use crate::renderer::vulkan::vulkan_utils::VulkanUtils;
use crate::{renderer::{baserenderer::BaseRenderer, resource_handles::{IndexBufferHandle, PipelineHandle, VertexBufferHandle}, resources::{base_index::BaseIndex, base_vertex::BaseVertex, pipeline_config::PipelineConfig, shader_type::ShaderType, viewport_config::ViewportConfig}, vulkan::{vulkan_indexbuffer::VulkanIndexBuffer, vulkan_pipeline::VulkanPipeline, vulkan_shader::VulkanShader, vulkan_surface::VulkanSurface, vulkan_vertexbuffer::VulkanVertexBuffer}}, window::rawhandle::RawHandle};


pub struct VulkanRenderer {

    raw_handle: RawHandle,
    recreation_needed: bool,

    entry: ash::Entry,
    instance: ash::Instance,

    surface: ash::vk::SurfaceKHR,
    surface_loader: ash::khr::surface::Instance,

    physical_device: ash::vk::PhysicalDevice,
    device: ash::Device,

    graphics_queue: ash::vk::Queue,
    present_queue: ash::vk::Queue,

    swapchain: ash::vk::SwapchainKHR,
    swapchain_loader: ash::khr::swapchain::Device,
    swapchain_images: Vec<vk::Image>,
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,

    swapchain_image_views: Vec<vk::ImageView>,

    render_pass: vk::RenderPass,

    descriptor_set_layouts: Vec<VulkanDescriptorSetLayout>,
    pipelines: Vec<VulkanPipeline>,

    swapchain_frame_buffers: Vec<vk::Framebuffer>,

    command_pool: vk::CommandPool,
    descriptor_pool: vk::DescriptorPool,

    command_buffers: Vec<vk::CommandBuffer>,
    vertex_buffers: Vec<VulkanVertexBuffer>,
    index_buffers: Vec<VulkanIndexBuffer>,
    uniform_buffers: Vec<VulkanUniformBuffer>,

    shaders: Vec<VulkanShader>,
    viewports: Vec<vk::Viewport>,
    scissors: Vec<vk::Rect2D>,

    vertex_buffer_next_id: u32,
    index_buffer_next_id: u32,
    uniform_buffer_next_id: u32,
    shader_next_id: u32,
    viewport_next_id: u32,
    scissor_next_id: u32,
    descriptor_set_layout_next_id: u32,
    pipeline_next_id: u32,

    current_pipeline: Option<PipelineHandle>,
    current_uniform_buffer: Option<u32>,

    acquire_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphore: Vec<vk::Semaphore>,
    in_flight_fence: Vec<vk::Fence>,
    images_in_flight: Vec<vk::Fence>,

    current_frame: u32,
    is_paused: bool,
    
    is_swapchain_valid: bool, 

    // Etats de la frame
    image_index: u32,
    wait_semaphores: Vec<vk::Semaphore>,
    wait_stages: Vec<vk::PipelineStageFlags>,
    signal_semaphores: Vec<vk::Semaphore>,

}

struct SwapchainSupportDetails {
    capabilities: ash::vk::SurfaceCapabilitiesKHR,
    formats: Vec<ash::vk::SurfaceFormatKHR>,
    present_modes: Vec<ash::vk::PresentModeKHR>,
}

struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

impl QueueFamilyIndices {
    fn is_complete(&self) -> bool {
        return self.graphics_family.is_some() && self.present_family.is_some();
    }
}



impl BaseRenderer for VulkanRenderer {

    fn create(raw_handle: &RawHandle, surface_dimension: (u32, u32)) -> Self {
        
        let entry = unsafe { ash::Entry::load().expect("Vulkan not available") };
        let instance = Self::create_vk_instance(&entry, raw_handle);
        let (surface, surface_loader) = Self::create_vk_surface(&entry, &instance, raw_handle);
        let physical_device = Self::find_physical_device(&instance, &surface_loader, surface);
        let (device, graphics_queue, present_queue) = Self::create_logical_device(&instance, &surface_loader, surface, physical_device);
        let (swapchain_loader, swapchain, swapchain_images, swapchain_format, swapchain_extent) = Self::create_swapchain(&instance, &surface_loader, &device, surface, physical_device, surface_dimension);
        let swapchain_image_views = Self::create_image_views(&device, &swapchain_images, swapchain_format);
        let render_pass = Self::create_render_pass(&device, swapchain_format);
        let swapchain_frame_buffers = Self::create_frame_buffers(&device, &swapchain_image_views, render_pass, swapchain_extent);
        let command_pool = Self::create_command_pool(&instance, &device, &surface_loader, surface, physical_device);
        let descriptor_pool = Self::create_descriptor_pool(&device, &swapchain_images);
        
        let command_buffers = Self::create_command_buffers(&device, &swapchain_frame_buffers, command_pool);
        let (acquire_semaphores, render_finished_semaphore, in_flight_fence, images_in_flight) = Self::create_sync_objects(&device, &swapchain_images);


        let properties = unsafe { instance.get_physical_device_properties(physical_device) };
        let name = unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) };
        println!("Selected GPU: {:?}", name);


        return Self {
            raw_handle: raw_handle.clone(),
            recreation_needed: false,
            entry,
            instance,
            surface,
            surface_loader,
            physical_device,
            device,
            graphics_queue,
            present_queue,
            swapchain,
            swapchain_loader,
            swapchain_images,
            swapchain_format,
            swapchain_extent,
            swapchain_image_views,
            render_pass,

            descriptor_set_layouts: Vec::new(),
            pipelines: Vec::new(),

            swapchain_frame_buffers,
            command_pool,
            descriptor_pool,

            command_buffers,
            vertex_buffers: Vec::new(),
            index_buffers: Vec::new(),
            uniform_buffers: Vec::new(),

            shaders: Vec::new(),
            viewports: Vec::new(),
            scissors: Vec::new(),

            vertex_buffer_next_id: 0,
            index_buffer_next_id: 0,
            uniform_buffer_next_id: 0,
            shader_next_id: 0,
            viewport_next_id: 0,
            scissor_next_id: 0,
            descriptor_set_layout_next_id: 0,
            pipeline_next_id: 0,

            current_pipeline: None,
            current_uniform_buffer: None,

            acquire_semaphores, 
            render_finished_semaphore,
            in_flight_fence,
            images_in_flight,

            current_frame: 0,
            is_paused: false,

            is_swapchain_valid: true,

            image_index: 0,
            wait_semaphores: Vec::new(),
            wait_stages: Vec::new(),
            signal_semaphores: Vec::new(),
        };

    }

    fn update_surface_dimension(&mut self, surface_dimension:(u32, u32)) {
        self.recreate_swapchain(surface_dimension);
    }

    fn pause(&mut self) {
        self.is_paused = true;
    }

    fn resume(&mut self) {
        self.is_paused = false;
    }

}


/// Create
impl VulkanRenderer {

    fn create_vk_instance(entry: &ash::Entry, raw_handle: &RawHandle) -> ash::Instance {

        let app_info = vk::ApplicationInfo::default()
            .application_name(c"VulkanApp")
            .api_version(vk::API_VERSION_1_2);

        let extensions = VulkanSurface::get_required_extensions(raw_handle);

        // #[cfg(debug_assertions)] -> Validation layers

        // #[cfg(debug_assertions)]
        let validation_layers = Self::VALIDATION_LAYERS;
        
        // #[cfg(not(debug_assertions))]
        // let validation_layers = &[];
        


        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(validation_layers);

        let instance = unsafe {
            entry.create_instance(&create_info, None)
                .expect("Vulkan Instance Error")
        };

        return instance;
    }


    fn create_vk_surface(entry: &ash::Entry, instance: &ash::Instance, raw_handle: &RawHandle) -> (ash::vk::SurfaceKHR, ash::khr::surface::Instance) {

        let (surface, surface_loader) = VulkanSurface::create_vk_surface(entry, instance, raw_handle);
        return (surface, surface_loader);

    }


    fn find_physical_device(instance: &ash::Instance, surface_loader: &ash::khr::surface::Instance, surface: ash::vk::SurfaceKHR) -> ash::vk::PhysicalDevice {

        let physical_devices = unsafe { instance.enumerate_physical_devices().expect("Vulkan is not supported by GPUs !") };

        let mut physical_device: Option<ash::vk::PhysicalDevice> = None;
        for d in physical_devices {
            if Self::is_device_suitable(instance, d, surface, surface_loader) {
                physical_device = Some(d);
                break;
            }
        }

        return physical_device.expect("No GPUs was supported !");
    }


    fn create_logical_device(instance: &ash::Instance, surface_loader: &ash::khr::surface::Instance, surface: ash::vk::SurfaceKHR, physical_device: ash::vk::PhysicalDevice) -> (ash::Device, ash::vk::Queue, ash::vk::Queue) {

        let indices = Self::find_queue_families(instance, surface_loader, physical_device, surface);

        let mut queue_create_infos: Vec<ash::vk::DeviceQueueCreateInfo> = Vec::new();
        let unique_queue_families: HashSet<u32> = HashSet::from([
            indices.graphics_family.unwrap(),
            indices.present_family.unwrap(),
        ]);

        let queue_priority: f32 = 1.0;
        for q in unique_queue_families {
            
            let info= ash::vk::DeviceQueueCreateInfo::default()
                .queue_family_index(q)
                .queue_priorities(std::slice::from_ref(&queue_priority));

            queue_create_infos.push(info);
        }

        let device_features = ash::vk::PhysicalDeviceFeatures::default();

        
        let create_info = ash::vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_features(&device_features)
            .enabled_extension_names(Self::DEVICE_EXTENSIONS);


        let device = unsafe { instance.create_device(physical_device, &create_info, None).expect("Logical Device Creation Failed !") };

        let graphics_queue = unsafe { device.get_device_queue(indices.graphics_family.unwrap(), 0) };
        let present_queue = unsafe { device.get_device_queue(indices.present_family.unwrap(), 0) };

        return (device, graphics_queue, present_queue);
    }


    fn create_swapchain(instance: &ash::Instance, surface_loader: &ash::khr::surface::Instance, device: &ash::Device, surface: ash::vk::SurfaceKHR, physical_device: ash::vk::PhysicalDevice, surface_dimension: (u32, u32))
        -> (ash::khr::swapchain::Device, ash::vk::SwapchainKHR, Vec<vk::Image>, vk::Format, vk::Extent2D)
    {

        let swapchain_details = Self::get_swapchain_support_details(surface_loader, physical_device, surface);

        let surface_format = Self::get_swap_surface_format(&swapchain_details.formats);
        let present_mode = Self::get_swap_present_mode(&swapchain_details.present_modes);
        let extent = Self::get_swap_extend(&swapchain_details.capabilities, surface_dimension);

        let mut image_count = swapchain_details.capabilities.min_image_count;
        if swapchain_details.capabilities.max_image_count > 0 && image_count > swapchain_details.capabilities.max_image_count {
            image_count = swapchain_details.capabilities.max_image_count;
        }

        let mut create_info = ash::vk::SwapchainCreateInfoKHR::default()
            .surface(surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT);

        let indices = Self::find_queue_families(instance, surface_loader, physical_device, surface);
        let queue_family_indices = &[ indices.graphics_family.unwrap(), indices.present_family.unwrap() ];

        if indices.graphics_family != indices.present_family {
            create_info = create_info.image_sharing_mode(vk::SharingMode::CONCURRENT)
                .queue_family_indices(queue_family_indices);
        }
        else {
            create_info = create_info.image_sharing_mode(vk::SharingMode::EXCLUSIVE);
        }

        create_info = create_info.pre_transform(swapchain_details.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);
            // .old_swapchain(old_swapchain)

        let swapchain_loader = ash::khr::swapchain::Device::new(instance, device);

        let swapchain = unsafe { swapchain_loader.create_swapchain(&create_info, None).expect("Swapchain Creation Failed !") };

        let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain).expect("Swapchain Images Failed !") };

        let swapchain_image_format = surface_format.format;
        let swapchain_extent = extent;

        return (swapchain_loader, swapchain, swapchain_images, swapchain_image_format, swapchain_extent);

    }


    fn create_image_views(device: &ash::Device, swapchain_images: &Vec<vk::Image>, swapchain_format: vk::Format) -> Vec<vk::ImageView> {

        let mut swapchain_image_views: Vec<vk::ImageView> = Vec::new();

        let component_mappings = vk::ComponentMapping::default()
            .r(vk::ComponentSwizzle::IDENTITY)
            .g(vk::ComponentSwizzle::IDENTITY)
            .b(vk::ComponentSwizzle::IDENTITY)
            .a(vk::ComponentSwizzle::IDENTITY);

        let subresource_range = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1);

        for image in swapchain_images {
            
            let create_info = ash::vk::ImageViewCreateInfo::default()
                .image(*image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(swapchain_format)
                .components(component_mappings)
                .subresource_range(subresource_range);

            let image_view = unsafe { device.create_image_view(&create_info, None).expect("Image View Creation Failed !") };

            swapchain_image_views.push(image_view);
        }

        return swapchain_image_views;

    }


    fn create_render_pass(device: &ash::Device, swapchain_format: vk::Format) -> vk::RenderPass {
        
        let color_attachment = [
            vk::AttachmentDescription::default()
                .format(swapchain_format)
                .samples(vk::SampleCountFlags::TYPE_1)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
        ];

        let color_attachment_ref = [
            vk::AttachmentReference::default()
                .attachment(0)
                .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        ];

        let subpass = [
            vk::SubpassDescription::default()
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .color_attachments(&color_attachment_ref)
        ];

        let dependency = [
            vk::SubpassDependency::default()
                .src_subpass(vk::SUBPASS_EXTERNAL)
                .dst_subpass(0)
                .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags::NONE)
                .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
        ];

        let render_pass_info = vk::RenderPassCreateInfo::default()
            .attachments(&color_attachment)
            .subpasses(&subpass)
            .dependencies(&dependency);

        let render_pass = unsafe { device.create_render_pass(&render_pass_info, None).expect("Render Pass Creation Failed !") };

        return render_pass;

    }


    fn create_frame_buffers(device: &ash::Device, swapchain_image_views: &Vec<vk::ImageView>, render_pass: vk::RenderPass, swapchain_extent: vk::Extent2D) -> Vec<vk::Framebuffer> {

        let mut swapchain_frame_buffers: Vec<vk::Framebuffer> = Vec::new();

        for view in swapchain_image_views {
            
            let attachments: &[vk::ImageView] = &[
                *view
            ];

            let framebuffer_info = vk::FramebufferCreateInfo::default()
                .render_pass(render_pass)
                .attachments(&attachments)
                .width(swapchain_extent.width)
                .height(swapchain_extent.height)
                .layers(1);

            let framebuffer = unsafe { device.create_framebuffer(&framebuffer_info, None).expect("Framebuffer Creation Failed !") };
            swapchain_frame_buffers.push(framebuffer);
        }

        return swapchain_frame_buffers;
    }


    fn create_command_pool(instance: &ash::Instance, device: &ash::Device, surface_loader: &ash::khr::surface::Instance, surface: vk::SurfaceKHR, physical_device: vk::PhysicalDevice) -> vk::CommandPool {

        let queue_family_indices = Self::find_queue_families(instance, surface_loader, physical_device, surface);

        let pool_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(queue_family_indices.graphics_family.unwrap())
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        let command_pool = unsafe { device.create_command_pool(&pool_info, None).expect("Command Pool Creation Failed !") };

        return command_pool;
    }


    fn create_descriptor_pool(device: &ash::Device, swapchain_images: &Vec<vk::Image>) -> vk::DescriptorPool {

        let descriptor_pool_size = [
            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(swapchain_images.len() as u32)
        ];

        let pool_info = vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(&descriptor_pool_size)
            .max_sets(swapchain_images.len() as u32)
            .flags(vk::DescriptorPoolCreateFlags::empty());

        let descriptor_pool = unsafe { device.create_descriptor_pool(&pool_info, None).expect("Descriptor Pool Creation Failed !") };

        return descriptor_pool;
    }


    fn create_command_buffers(device: &ash::Device, swapchain_frame_buffers: &Vec<vk::Framebuffer>, command_pool: vk::CommandPool) -> Vec<vk::CommandBuffer> {

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(swapchain_frame_buffers.len() as u32);

        let command_buffers = unsafe { device.allocate_command_buffers(&command_buffer_allocate_info).expect("Command Buffer Allocation Failed !") };

        return command_buffers;
    }


    fn create_descriptor_sets(device: &ash::Device, swapchain_images: &Vec<vk::Image>, uniform_buffers: &Vec<vk::Buffer>, size: u64, descriptor_set_layout: &Vec<vk::DescriptorSetLayout>, descriptor_pool: vk::DescriptorPool) -> Vec<vk::DescriptorSet> {

        // On prend le descriptor_set_layout [0] pour le moment
        let layouts = vec![descriptor_set_layout[0]; swapchain_images.len()];

        let allocate_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(descriptor_pool)
            .set_layouts(&layouts);

        let descriptor_sets = unsafe { device.allocate_descriptor_sets(&allocate_info).expect("Descriptor Set Allocation Failed !") };

        for i in 0..swapchain_images.len() {

            let buffer_info = [
                vk::DescriptorBufferInfo::default()
                    .buffer(uniform_buffers[i])
                    .offset(0)
                    .range(size)
            ];

            let descriptor_write = [
                vk::WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[i])
                    .dst_binding(0)
                    .dst_array_element(0)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .buffer_info(&buffer_info)
                    // .image_info()
                    // .texel_buffer_view();
            ];

            unsafe { device.update_descriptor_sets(&descriptor_write, &[]) };
        }

        return descriptor_sets;
    }


    fn create_sync_objects(device: &ash::Device, swapchain_images: &Vec<vk::Image>) -> (Vec<vk::Semaphore>, Vec<vk::Semaphore>, Vec<vk::Fence>, Vec<vk::Fence>) {

        let mut acquire_semaphores: Vec<vk::Semaphore> = Vec::new();
        let mut render_finished_semaphore: Vec<vk::Semaphore> = Vec::new();
        let mut in_flight_fence: Vec<vk::Fence> = Vec::new();
        let images_in_flight = vec![vk::Fence::null(); swapchain_images.len()];

        let semaphore_info = vk::SemaphoreCreateInfo::default();

        let fence_info = vk::FenceCreateInfo::default()
            .flags(vk::FenceCreateFlags::SIGNALED);

        for _ in 0..swapchain_images.len() {
            
            let semaphore = unsafe { device.create_semaphore(&semaphore_info, None).expect("Semaphore Creation Failed !") };
            render_finished_semaphore.push(semaphore);
            
        }

        for _ in 0..Self::MAX_FRAMES_IN_FLIGHT {

            let fence = unsafe { device.create_fence(&fence_info, None).expect("Fence Creation Failed !") };
            in_flight_fence.push(fence);

            let semaphore = unsafe { device.create_semaphore(&semaphore_info, None).expect("Semaphore Creation Failed !") };
            acquire_semaphores.push(semaphore);

        }

        return (acquire_semaphores, render_finished_semaphore, in_flight_fence, images_in_flight);
    }

    pub fn recreate_swapchain(&mut self, surface_dimension: (u32, u32)) {

        // println!("RECREATE SWAPCHAIN -------------------------------------------------------------------------------------");

        let (width, height) = surface_dimension;
        if width == 0 || height == 0 { self.recreation_needed = true; return; }

        unsafe { self.device.device_wait_idle().expect("Wait Idle Failed !") };

        self.current_frame = 0;

        self.destroy_swapchain();

        let (swapchain_loader, swapchain, swapchain_images, swapchain_format, swapchain_extent) = Self::create_swapchain(&self.instance, &self.surface_loader, &self.device, self.surface, self.physical_device, surface_dimension);
        self.swapchain_loader = swapchain_loader;
        self.swapchain = swapchain;
        self.swapchain_images = swapchain_images;
        self.swapchain_format = swapchain_format;
        self.swapchain_extent = swapchain_extent;

        let swapchain_image_views = Self::create_image_views(&self.device, &self.swapchain_images, self.swapchain_format);
        self.swapchain_image_views = swapchain_image_views;

        let render_pass = Self::create_render_pass(&self.device, self.swapchain_format);
        self.render_pass = render_pass;

        let swapchain_frame_buffers = Self::create_frame_buffers(&self.device, &self.swapchain_image_views, self.render_pass, self.swapchain_extent);
        self.swapchain_frame_buffers = swapchain_frame_buffers;

        let descriptor_pool = Self::create_descriptor_pool(&self.device, &self.swapchain_images);
        self.descriptor_pool = descriptor_pool;

        for i in 0..self.uniform_buffers.len() {

            let descriptor_sets = {
                let descriptor_set_layout = &self.descriptor_set_layouts[self.uniform_buffers[i].shader_layout.0 as usize];
                Self::create_descriptor_sets(&self.device, &self.swapchain_images, &self.uniform_buffers[i].buffer, self.uniform_buffers[i].size, &descriptor_set_layout.descriptor_set_layouts, self.descriptor_pool)
            };

            self.uniform_buffers[i].update_descriptor_sets(descriptor_sets);

        }

        let (acquire_semaphores, render_finished_semaphore, in_flight_fence, images_in_flight) = Self::create_sync_objects(&self.device, &self.swapchain_images);
        self.acquire_semaphores = acquire_semaphores;
        self.render_finished_semaphore = render_finished_semaphore;
        self.in_flight_fence = in_flight_fence;
        self.images_in_flight = images_in_flight;

        self.is_swapchain_valid = true;

    }

}


/// Utils
impl VulkanRenderer {

    fn is_device_suitable(instance: &ash::Instance, physical_device: ash::vk::PhysicalDevice, surface: ash::vk::SurfaceKHR, surface_loader: &ash::khr::surface::Instance) -> bool {
        
        let support_extensions = Self::check_device_extension_support(instance, physical_device);

        if !support_extensions { return false; }

        let swapchain_details = Self::get_swapchain_support_details(surface_loader, physical_device, surface);
        let swapchain_adequate = !swapchain_details.formats.is_empty() && !swapchain_details.present_modes.is_empty();

        let indices = Self::find_queue_families(instance, surface_loader, physical_device, surface);

        return indices.is_complete() && swapchain_adequate;
    }

    const VALIDATION_LAYERS: &[*const i8] = &[
        c"VK_LAYER_KHRONOS_validation".as_ptr(),
    ];

    const DEVICE_EXTENSIONS: &[*const i8] = &[
        ash::khr::swapchain::NAME.as_ptr(),
    ];

    const MAX_FRAMES_IN_FLIGHT: u32 = 2;

    fn check_device_extension_support(instance: &ash::Instance, physical_device: ash::vk::PhysicalDevice) -> bool {

        let available_extensions = unsafe { instance.enumerate_device_extension_properties(physical_device).expect("Error in extension check !") };

        Self::DEVICE_EXTENSIONS.iter().all(|&required| {
            available_extensions.iter().any(|ext| {
                let name = unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) };
                name == unsafe { CStr::from_ptr(required) }
            })
        })

    }

    fn get_swapchain_support_details(surface_loader: &ash::khr::surface::Instance, physical_device: ash::vk::PhysicalDevice, surface: ash::vk::SurfaceKHR) -> SwapchainSupportDetails {

        unsafe {

            let capabilities = surface_loader.get_physical_device_surface_capabilities(physical_device, surface).expect("Capabilities Error !");

            let formats = surface_loader.get_physical_device_surface_formats(physical_device, surface).expect("Formats Error !");

            let present_modes = surface_loader.get_physical_device_surface_present_modes(physical_device, surface).expect("Present Modes Error !");

            return SwapchainSupportDetails { capabilities, formats, present_modes };
        }

    }

    fn find_queue_families(instance: &ash::Instance, surface_loader: &ash::khr::surface::Instance, physical_device: ash::vk::PhysicalDevice, surface: ash::vk::SurfaceKHR) -> QueueFamilyIndices {

        unsafe {

            let mut indices = QueueFamilyIndices { graphics_family: None, present_family: None };

            let queue_families = instance.get_physical_device_queue_family_properties(physical_device);

            for (i, q) in queue_families.iter().enumerate()
             {
                
                let present_support = match surface_loader.get_physical_device_surface_support(physical_device, i as u32, surface) {
                    Ok(v) => v,
                    Err(_) => false,
                };

                if q.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                    indices.graphics_family = Some(i as u32);
                }

                if present_support {
                    indices.present_family = Some(i as u32);
                }

                if indices.is_complete() {
                    break;
                }

            }

            return indices;
        }

    }

    fn get_swap_surface_format(available_formats: &[ash::vk::SurfaceFormatKHR]) -> ash::vk::SurfaceFormatKHR {

        for f in available_formats {
            if f.format == vk::Format::B8G8R8A8_SRGB && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR {
                return *f;
            }
        }

        return available_formats[0];
    }

    fn get_swap_present_mode(available_present_modes: &[ash::vk::PresentModeKHR]) -> ash::vk::PresentModeKHR {

        for p in available_present_modes {
            if *p == vk::PresentModeKHR::MAILBOX {
                return *p;
            }
        }

        return vk::PresentModeKHR::FIFO;
    }

    fn get_swap_extend(capabilities: &ash::vk::SurfaceCapabilitiesKHR, surface_dimension: (u32, u32)) -> ash::vk::Extent2D {

        if capabilities.current_extent.width != u32::MAX {
            return capabilities.current_extent;
        }

        let (width, height) = surface_dimension;

        let actual_extend = vk::Extent2D::default()
            .width(width as u32)
            .height(height as u32);

        return actual_extend;
    }

    fn find_memory_type(instance: &ash::Instance, physical_device: ash::vk::PhysicalDevice, type_filter: u32, properties: vk::MemoryPropertyFlags) -> u32 {

        let memory_properties = unsafe { instance.get_physical_device_memory_properties(physical_device) };

        for i in 0..memory_properties.memory_type_count {
            if (type_filter & (1 << i)) != 0 && memory_properties.memory_types[i as usize].property_flags.contains(properties) {
                return i;
            }
        }

        panic!("No Available Memory Found !");

    }

    fn create_buffer(instance: &ash::Instance, device: &ash::Device, physical_device: ash::vk::PhysicalDevice, size: vk::DeviceSize, usage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags) -> (vk::Buffer, vk::DeviceMemory) {

        let buffer_info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let buffer = unsafe { device.create_buffer(&buffer_info, None).expect("Buffer Creation Failed !") };

        let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };

        let allocate_info = vk::MemoryAllocateInfo::default()
            .allocation_size(memory_requirements.size)
            .memory_type_index(Self::find_memory_type(instance, physical_device, memory_requirements.memory_type_bits, properties));

        let buffer_memory = unsafe { device.allocate_memory(&allocate_info, None).expect("Memory Allocation Failed !") };

        unsafe { device.bind_buffer_memory(buffer, buffer_memory, 0) };

        return (buffer, buffer_memory);
    }

    fn copy_buffer(device: &ash::Device, src_buffer: vk::Buffer, dst_buffer: vk::Buffer, size: vk::DeviceSize, command_pool: vk::CommandPool, graphics_queue: ash::vk::Queue) {

        let allocate_info = vk::CommandBufferAllocateInfo::default()
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_pool(command_pool)
            .command_buffer_count(1);

        let command_buffer = unsafe { device.allocate_command_buffers(&allocate_info).expect("Command Buffer Allocation Failed !") };

        let begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe { device.begin_command_buffer(command_buffer[0], &begin_info) };

        let copy_region = vk::BufferCopy::default()
            .src_offset(0)
            .dst_offset(0)
            .size(size);

        unsafe { device.cmd_copy_buffer(command_buffer[0], src_buffer, dst_buffer, &[copy_region]) };

        unsafe { device.end_command_buffer(command_buffer[0]) };

        let submit_info = vk::SubmitInfo::default()
            .command_buffers(&command_buffer);

        unsafe { 
            device.queue_submit(graphics_queue, &[submit_info], vk::Fence::null());
            device.queue_wait_idle(graphics_queue);
        };

    }

    pub fn begin_draw(&mut self) {

        if self.is_paused { return; }

        if !self.is_swapchain_valid {
            self.recreate_swapchain((self.swapchain_extent.width, self.swapchain_extent.height));
        }

        unsafe { self.device.wait_for_fences(&[self.in_flight_fence[self.current_frame as usize]], true, u64::MAX).expect("Fence Waiting Failed !") };

        let acquire_semaphore = self.acquire_semaphores[self.current_frame as usize];

        let result = unsafe { self.swapchain_loader.acquire_next_image(self.swapchain, u64::MAX, acquire_semaphore, vk::Fence::null()) };

        let image_index = match result {
            Ok((index, false)) => index,
            Ok((_, true)) => { self.is_swapchain_valid = false; return; }
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => { self.is_swapchain_valid = false; return; },
            Err(_) => panic!("Next Image Failed !")
        };

        self.start_command_buffer(image_index);

        if self.images_in_flight[image_index as usize] != vk::Fence::null() {
            unsafe { self.device.wait_for_fences(&[self.images_in_flight[image_index as usize]], true, u64::MAX).expect("Fence Waiting Failed !") };
        }

        self.images_in_flight[image_index as usize] = self.in_flight_fence[self.current_frame as usize];
        
        let wait_semaphores = [acquire_semaphore];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [self.render_finished_semaphore[image_index as usize]];

        self.image_index = image_index;
        self.wait_semaphores = wait_semaphores.to_vec();
        self.wait_stages = wait_stages.to_vec();
        self.signal_semaphores = signal_semaphores.to_vec();

    }

    pub fn end_draw(&mut self) {

        if self.is_paused { return; }
        if !self.is_swapchain_valid { return; }

        let image_index = self.image_index;

        self.end_command_buffer(image_index);
        
        let command_buffers = [self.command_buffers[image_index as usize]];
        let submit_info = vk::SubmitInfo::default()
            .wait_semaphores(&self.wait_semaphores)
            .wait_dst_stage_mask(&self.wait_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&self.signal_semaphores);

        unsafe { self.device.reset_fences(&[self.in_flight_fence[self.current_frame as usize]]).expect("Fences Reset Failed !") };

        unsafe { self.device.queue_submit(self.graphics_queue,&[submit_info], self.in_flight_fence[self.current_frame as usize]).expect("Queue Submit Failed !") };

        let swapchains = [self.swapchain];

        let indices = [image_index];
        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(&self.signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&indices);

        let result = unsafe { self.swapchain_loader.queue_present(self.present_queue, &present_info) };

        match result {
            Ok(_) => {},
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => self.is_swapchain_valid = false,
            Err(vk::Result::SUBOPTIMAL_KHR) => self.is_swapchain_valid = false,
            Err(_) => panic!("Next Image Failed !")
        };

        self.current_frame = (self.current_frame + 1) % Self::MAX_FRAMES_IN_FLIGHT;

    }

    fn start_command_buffer(&mut self, image_index: u32) {

        let buffer = self.command_buffers[image_index as usize];

        unsafe { 
            self.device.reset_command_buffer(
            buffer,
            vk::CommandBufferResetFlags::empty()
            ).expect("Command Buffer Reset Failed !");
        };

        let command_buffer_begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::empty());
            //.inheritance_info();

        unsafe { self.device.begin_command_buffer(buffer, &command_buffer_begin_info).expect("Begin Command Buffer Error !") };
        
        let clear_values = [vk::ClearValue::default()];

        let render_pass_begin_info = vk::RenderPassBeginInfo::default()
            .render_pass(self.render_pass)
            .framebuffer(self.swapchain_frame_buffers[image_index as usize])
            .render_area(vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: self.swapchain_extent })
            .clear_values(&clear_values);

        unsafe { self.device.cmd_begin_render_pass(buffer, &render_pass_begin_info, vk::SubpassContents::INLINE) };

    }

    fn end_command_buffer(&self, image_index: u32) {

        let buffer = self.command_buffers[image_index as usize];

        unsafe { self.device.cmd_end_render_pass(buffer) };
        unsafe { self.device.end_command_buffer(buffer).expect("End Command Buffer Error !") };

    }

    pub fn draw_indexed<V: BaseVertex, I: BaseIndex>(&self, vertex_buffer_handle: VertexBufferHandle<V>, index_buffer_handle: IndexBufferHandle<I>) {

        if self.is_paused { return; }
        if !self.is_swapchain_valid { return; }

        let buffer = self.command_buffers[self.image_index as usize];

        let offsets = &[0 as u64];

        unsafe { self.device.cmd_bind_vertex_buffers(buffer, 0, &[self.vertex_buffers[vertex_buffer_handle.0 as usize].buffer], offsets) };
        unsafe { self.device.cmd_bind_index_buffer(buffer, self.index_buffers[index_buffer_handle.0 as usize].buffer, 0, vk::IndexType::UINT16) };

        unsafe { self.device.cmd_draw_indexed(buffer, self.index_buffers[index_buffer_handle.0 as usize].count as u32, 1, 0, 0, 0); }

    }


    pub fn create_vertex_buffer<V: BaseVertex>(&mut self, size: u64) -> u32 {

        let buffer_size = (std::mem::size_of::<V>() * size as usize) as u64;
        let (vertex_buffer, vertex_buffer_memory) = Self::create_buffer(&self.instance, &self.device, self.physical_device, buffer_size, vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER, vk::MemoryPropertyFlags::DEVICE_LOCAL);

        let vertex_buffer = VulkanVertexBuffer {
            count: 0,
            size: buffer_size,
            buffer: vertex_buffer,
            buffer_memory: vertex_buffer_memory
        };
        
        self.vertex_buffers.push(vertex_buffer);

        let id = self.vertex_buffer_next_id;
        self.vertex_buffer_next_id += 1;

        return id;
    }


    pub fn set_vertex_buffer_data<V: BaseVertex>(&mut self, vertex_buffer_handle: VertexBufferHandle<V>, vertices: &[V]) {

        let vertex_buffer = &mut self.vertex_buffers[vertex_buffer_handle.0 as usize];

        let (staging_buffer, staging_buffer_memory) = Self::create_buffer(&self.instance, &self.device, self.physical_device, vertex_buffer.size, vk::BufferUsageFlags::TRANSFER_SRC, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);

        unsafe {
            let data_memory = self.device.map_memory(staging_buffer_memory, 0, vertex_buffer.size, vk::MemoryMapFlags::empty())
                .expect("Map Memory Failed !") as *mut V;
            
            std::ptr::copy_nonoverlapping(vertices.as_ptr(), data_memory, vertices.len());

            self.device.unmap_memory(staging_buffer_memory);
        }

        Self::copy_buffer(&self.device, staging_buffer, vertex_buffer.buffer, vertex_buffer.size, self.command_pool, self.graphics_queue);

        vertex_buffer.count = vertices.len();

        unsafe {
            self.device.destroy_buffer(staging_buffer, None);
            self.device.free_memory(staging_buffer_memory, None);
        }

    }

    pub fn create_index_buffer<I: BaseIndex>(&mut self, size: u64) -> u32 {

        let buffer_size = (std::mem::size_of::<I>() * size as usize) as u64;
        let (index_buffer, index_buffer_memory) = Self::create_buffer(&self.instance, &self.device, self.physical_device, buffer_size, vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER, vk::MemoryPropertyFlags::DEVICE_LOCAL);

        let index_buffer = VulkanIndexBuffer {
            count: 0,
            size: buffer_size,
            buffer: index_buffer,
            buffer_memory: index_buffer_memory
        };
        
        self.index_buffers.push(index_buffer);

        let id = self.index_buffer_next_id;
        self.index_buffer_next_id += 1;

        return id;
    }

    pub fn set_index_buffer_data<I: BaseIndex>(&mut self, index_buffer_handle: IndexBufferHandle<I>, indices: &[I]) {

        let index_buffer = &mut self.index_buffers[index_buffer_handle.0 as usize];

        let (staging_buffer, staging_buffer_memory) = Self::create_buffer(&self.instance, &self.device, self.physical_device, index_buffer.size, vk::BufferUsageFlags::TRANSFER_SRC, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);

        unsafe {
            let data_memory = self.device.map_memory(staging_buffer_memory, 0, index_buffer.size, vk::MemoryMapFlags::empty())
                .expect("Map Memory Failed !") as *mut I;
            
            std::ptr::copy_nonoverlapping(indices.as_ptr(), data_memory, indices.len());

            self.device.unmap_memory(staging_buffer_memory);
        }

        Self::copy_buffer(&self.device, staging_buffer, index_buffer.buffer, index_buffer.size, self.command_pool, self.graphics_queue);

        index_buffer.count = indices.len();

        unsafe {
            self.device.destroy_buffer(staging_buffer, None);
            self.device.free_memory(staging_buffer_memory, None);
        }

    }


    pub fn create_uniform_buffer<U: BaseUniform>(&mut self, shader_layout_handle: ShaderLayoutHandle) -> u32 {

        let buffer_size = std::mem::size_of::<U>() as u64;

        let mut uniform_buffers = Vec::<vk::Buffer>::new();
        let mut uniform_buffers_memory = Vec::<vk::DeviceMemory>::new();

        for _ in 0..self.swapchain_images.len() {
            
            let (buffer, buffer_memory) = Self::create_buffer(&self.instance, &self.device, self.physical_device, buffer_size, vk::BufferUsageFlags::UNIFORM_BUFFER, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);

            uniform_buffers.push(buffer);
            uniform_buffers_memory.push(buffer_memory);
        }

        let descriptor_set_layout = &self.descriptor_set_layouts[shader_layout_handle.0 as usize];
        let descriptor_sets = Self::create_descriptor_sets(&self.device, &self.swapchain_images, &uniform_buffers, buffer_size, &descriptor_set_layout.descriptor_set_layouts, self.descriptor_pool);

        let uniform_buffer = VulkanUniformBuffer {
            size: buffer_size,
            shader_layout: shader_layout_handle,
            buffer: uniform_buffers,
            buffer_memory: uniform_buffers_memory,
            descriptor_sets: descriptor_sets,
        };

        self.uniform_buffers.push(uniform_buffer);

        let id = self.uniform_buffer_next_id;
        self.uniform_buffer_next_id += 1;

        return id;
    }

    pub fn set_uniform_buffer_data<U: BaseUniform>(&mut self, uniform: U, handle: UniformBufferHandle<U>) {

        unsafe {
            let data = self.device.map_memory(
                self.uniform_buffers[handle.0 as usize].buffer_memory[self.current_frame as usize],
                0,
                std::mem::size_of::<U>() as u64,
                vk::MemoryMapFlags::empty(),
            ).unwrap() as *mut U;
            data.write(uniform);
            self.device.unmap_memory(self.uniform_buffers[handle.0 as usize].buffer_memory[self.current_frame as usize]);
        }

    }

    pub fn create_shader(&mut self, path: &str, shader_type: ShaderType) -> u32 {

        let shader = VulkanShader::create(path, VulkanUtils::shader_type_to_vk_shader_stage(shader_type), &self.device);
        self.shaders.push(shader);

        let id = self.shader_next_id;
        self.shader_next_id += 1;

        return id;
    }

    pub fn create_viewport(&mut self, config: ViewportConfig) -> u32 {

        let viewport = vk::Viewport::default()
            .x(config.x)
            .y(config.y)
            .width(config.width)
            .height(config.height)
            .min_depth(config.min_depth)
            .max_depth(config.max_depth);

        self.viewports.push(viewport);

        let id = self.viewport_next_id;
        self.viewport_next_id += 1;

        return id;
    }

    pub fn create_scissor(&mut self, config: ScissorConfig) -> u32 {

        let scissor = vk::Rect2D::default()
            .offset(vk::Offset2D::default().x(config.x).y(config.y))
            .extent(vk::Extent2D::default().width(config.width).height(config.height));

        self.scissors.push(scissor);

        let id = self.scissor_next_id;
        self.scissor_next_id += 1;

        return id;
    }

    pub fn create_descriptor_set_layout(&mut self, bindings: Vec<DescriptorBinding>) -> u32 {

        let descriptor_set_layout = VulkanDescriptorSetLayout::create(&self.device, bindings);

        self.descriptor_set_layouts.push(descriptor_set_layout);

        let id = self.descriptor_set_layout_next_id;
        self.descriptor_set_layout_next_id += 1;

        return id;
    }

    pub fn create_pipeline(&mut self, pipeline_config: PipelineConfig) -> u32 {

        let mut shaders: Vec<&VulkanShader> = Vec::new();
        for handle in &pipeline_config.shaders {
            shaders.push(&self.shaders[handle.0 as usize]);
        }

        let descriptor_set_layout = &self.descriptor_set_layouts[pipeline_config.shader_layout_handle.0 as usize];

        let pipeline = VulkanPipeline::create(&self.device, self.render_pass, &shaders, descriptor_set_layout, pipeline_config);
        self.pipelines.push(pipeline);

        let id = self.pipeline_next_id;
        self.pipeline_next_id += 1;

        return id;
    }

    pub fn set_pipeline(&mut self, pipeline_handle: PipelineHandle) {

        if self.is_paused { return; }
        if !self.is_swapchain_valid { return; }

        let buffer = self.command_buffers[self.image_index as usize];
        unsafe { self.device.cmd_bind_pipeline(buffer, vk::PipelineBindPoint::GRAPHICS, self.pipelines[pipeline_handle.0 as usize].pipeline) };
        self.current_pipeline = Some(pipeline_handle);

    }

    pub fn set_uniform_buffer<U: BaseUniform>(&mut self, uniform_buffer: UniformBufferHandle<U>) {

        if self.is_paused { return; }
        if !self.is_swapchain_valid { return; }

        let descriptor_set = self.uniform_buffers[uniform_buffer.0 as usize].descriptor_sets[self.image_index as usize];

        let buffer = self.command_buffers[self.image_index as usize];
        unsafe { self.device.cmd_bind_descriptor_sets(buffer, vk::PipelineBindPoint::GRAPHICS, self.pipelines[self.current_pipeline.expect("Pipeline Not Set !").0 as usize].layout, 0, &[descriptor_set], &[]); }
        self.current_uniform_buffer = Some(uniform_buffer.0);

    }

    pub fn set_viewport(&mut self, viewport: ViewportHandle) {

        if self.is_paused { return; }
        if !self.is_swapchain_valid { return; }

        let buffer = self.command_buffers[self.image_index as usize];
        unsafe { self.device.cmd_set_viewport(buffer, 0, &[self.viewports[viewport.0 as usize]]) };

    }

    pub fn set_scissor(&mut self, scissor: ScissorHandle) {

        if self.is_paused { return; }
        if !self.is_swapchain_valid { return; }

        let buffer = self.command_buffers[self.image_index as usize];
        unsafe { self.device.cmd_set_scissor(buffer, 0, &[self.scissors[scissor.0 as usize]]) };

    }

}


/// Destroy
impl VulkanRenderer {

    fn destroy_swapchain(&mut self) {

        unsafe {

            for i in 0..Self::MAX_FRAMES_IN_FLIGHT as usize {
                self.device.destroy_fence(self.in_flight_fence[i], None);
                self.device.destroy_semaphore(self.acquire_semaphores[i], None);
            }

            for i in 0..self.swapchain_images.len() as usize {
                self.device.destroy_semaphore(self.render_finished_semaphore[i], None);
            }

            self.device.destroy_descriptor_pool(self.descriptor_pool, None);
    
            self.device.destroy_render_pass(self.render_pass, None);

            for framebuffer in &self.swapchain_frame_buffers {
                self.device.destroy_framebuffer(*framebuffer, None);
            }

            for view in &self.swapchain_image_views {
                self.device.destroy_image_view(*view, None);
            }

            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
        }

    }

}

impl Drop for VulkanRenderer {

    fn drop(&mut self) {

        unsafe {

            self.device.device_wait_idle().expect("Wait Idle Failed !");

            for uniform_buffer in &mut self.uniform_buffers {
                uniform_buffer.destroy(&self.device);
            }
            self.uniform_buffers.clear();

            for index_buffer in &mut self.index_buffers {
                index_buffer.destroy(&self.device);
            }
            self.index_buffers.clear();

            for vertex_buffer in &mut self.vertex_buffers {
                vertex_buffer.destroy(&self.device);
            }
            self.vertex_buffers.clear();

            for descriptor_set_layout in &mut self.descriptor_set_layouts {
                descriptor_set_layout.destroy(&self.device);
            }
            self.descriptor_set_layouts.clear();

            self.device.destroy_command_pool(self.command_pool, None);


            for pipeline in &mut self.pipelines {
                pipeline.destroy(&self.device);
            }

            for shader in &mut self.shaders {
                shader.destroy(&self.device);
            }


            self.destroy_swapchain();

            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }

    }

}