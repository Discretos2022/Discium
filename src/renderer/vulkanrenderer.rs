
use std::{collections::HashSet, ffi::CStr, time::Instant};

use ash::vk;
use glam::{Mat4, Vec3};

use crate::{renderer::{baserenderer::BaseRenderer, vulkan_surface::VulkanSurface, vulkanuniformbufferobject::VulkanUniformBufferObject, vulkanvertex::VulkanVertex}, window::rawhandle::RawHandle};




pub struct VulkanRenderer {

    start_time: Instant,
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
    
    descriptor_set_layout: vk::DescriptorSetLayout,
    
    pipeline_layout: vk::PipelineLayout,
    graphics_pipeline: vk::Pipeline,

    swapchain_frame_buffers: Vec<vk::Framebuffer>,

    command_pool: vk::CommandPool,
    descriptor_pool: vk::DescriptorPool,

    command_buffers: Vec<vk::CommandBuffer>,


    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    index_buffer: vk::Buffer,
    index_buffer_memory: vk::DeviceMemory,
    uniform_buffers: Vec<vk::Buffer>,
    uniform_buffers_memory: Vec<vk::DeviceMemory>,
    descriptor_sets: Vec<vk::DescriptorSet>,


    // image_available_semaphore: Vec<vk::Semaphore>,
    acquire_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphore: Vec<vk::Semaphore>,
    in_flight_fence: Vec<vk::Fence>,
    images_in_flight: Vec<vk::Fence>,

    current_frame: u32,
    is_paused: bool,

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
        let descriptor_set_layout = Self::create_descriptor_set_layout(&device);
        let (pipeline_layout, graphics_pipeline) = Self::create_graphics_pipeline(&device, swapchain_extent, descriptor_set_layout, render_pass);
        let swapchain_frame_buffers = Self::create_frame_buffers(&device, &swapchain_image_views, render_pass, swapchain_extent);
        let command_pool = Self::create_command_pool(&instance, &device, &surface_loader, surface, physical_device);
        let descriptor_pool = Self::create_descriptor_pool(&device, &swapchain_images);
        
        let (vertex_buffer, vertex_buffer_memory) = Self::create_vertex_buffer(&instance, &device, physical_device, command_pool, graphics_queue);
        let (index_buffer, index_buffer_memory) = Self::create_index_buffer(&instance, &device, physical_device, command_pool, graphics_queue);
        let (uniform_buffers, uniform_buffers_memory) = Self::create_uniform_buffer(&instance, &device, physical_device, &swapchain_images);
        let descriptor_sets = Self::create_descriptor_sets(&device, &swapchain_images, &uniform_buffers, descriptor_set_layout, descriptor_pool);
        
        let command_buffers = Self::create_command_buffers(&device, &swapchain_frame_buffers, command_pool, render_pass, swapchain_extent, pipeline_layout, graphics_pipeline, vertex_buffer, index_buffer, &descriptor_sets);
        let (acquire_semaphores, render_finished_semaphore, in_flight_fence, images_in_flight) = Self::create_sync_objects(&device, &swapchain_images);


        let properties = unsafe { instance.get_physical_device_properties(physical_device) };
        let name = unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) };
        println!("Selected GPU: {:?}", name);


        return Self {
            start_time: std::time::Instant::now(),
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
            descriptor_set_layout,
            pipeline_layout,
            graphics_pipeline,
            swapchain_frame_buffers,
            command_pool,
            descriptor_pool,
            command_buffers,

            vertex_buffer,
            vertex_buffer_memory,
            index_buffer,
            index_buffer_memory,
            uniform_buffers,
            uniform_buffers_memory,
            descriptor_sets,

            // image_available_semaphore,
            acquire_semaphores, 
            render_finished_semaphore,
            in_flight_fence,
            images_in_flight,

            current_frame: 0,
            is_paused: false,
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
            .api_version(vk::API_VERSION_1_3);

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


    fn create_descriptor_set_layout(device: &ash::Device) -> vk::DescriptorSetLayout {

        let ubo_layout_binding = [vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::VERTEX)
            //.immutable_samplers();
        ];

        let layout_info = vk::DescriptorSetLayoutCreateInfo::default()
            .bindings(&ubo_layout_binding);

        let descriptor_set_layout = unsafe { device.create_descriptor_set_layout(&layout_info, None).expect("Descriptor Layout Creation Failed !") };

        return descriptor_set_layout;
    }


    fn create_graphics_pipeline(device: &ash::Device, swapchain_extent: vk::Extent2D, descriptor_set_layout: vk::DescriptorSetLayout, render_pass: vk::RenderPass) -> (vk::PipelineLayout, vk::Pipeline) {

        // let vert_shader_code = std::fs::read("shaders/vert.spv").expect("Opening Of 'vert.spv' Was Failed !");
        // let frag_shader_code = std::fs::read("shaders/frag.spv").expect("Opening Of 'frag.spv' Was Failed !");

        let vert_code = include_bytes!("shaders/bin/vert.spv");
        let frag_code = include_bytes!("shaders/bin/frag.spv");

        let vert_code = ash::util::read_spv(&mut std::io::Cursor::new(vert_code.as_ref())).unwrap();
        let vert_info = vk::ShaderModuleCreateInfo::default()
            .code(vert_code.as_slice());

        let frag_code = ash::util::read_spv(&mut std::io::Cursor::new(frag_code.as_ref())).unwrap();
        let frag_info = vk::ShaderModuleCreateInfo::default()
            .code(frag_code.as_slice());

        let vert_shader_module = unsafe { device.create_shader_module(&vert_info, None).expect("Vertex Shader Module Creation Failed !") };
        let frag_shader_module = unsafe { device.create_shader_module(&frag_info, None).expect("Fragment Shader Module Creation Failed !") };


        let vert_shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_shader_module)
            .name(c"main");

        let frag_shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_shader_module)
            .name(c"main");


        let shader_stages = [vert_shader_stage_info, frag_shader_stage_info ];

        let binding_description = [VulkanVertex::get_binding_description()];
        let attribute_descriptions = VulkanVertex::get_attribut_descriptions();


        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&binding_description)
            .vertex_attribute_descriptions(&attribute_descriptions);

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewports = [vk::Viewport::default()
            .x(0.0)
            .height(0.0)
            .width(swapchain_extent.width as f32)
            .height(swapchain_extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)
        ];

        let scissors = [vk::Rect2D::default()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(swapchain_extent)
        ];

        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);

        let rasteriser = vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .depth_bias_enable(false)
            .depth_bias_constant_factor(0.0)
            .depth_bias_clamp(0.0)
            .depth_bias_slope_factor(0.0);

        let multisampler = vk::PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1)
            .min_sample_shading(1.0)
            //.sample_mask()
            .alpha_to_coverage_enable(false)
            .alpha_to_one_enable(false);

        let color_blend_attachment = [
            vk::PipelineColorBlendAttachmentState::default()
                .color_write_mask(vk::ColorComponentFlags::R | vk::ColorComponentFlags::G | vk::ColorComponentFlags::B | vk::ColorComponentFlags::A)
        
                // .blend_enable(false)
                // .src_color_blend_factor(vk::BlendFactor::ONE)
                // .dst_color_blend_factor(vk::BlendFactor::ZERO)
                // .color_blend_op(vk::BlendOp::ADD)
                // .src_alpha_blend_factor(vk::BlendFactor::ONE)
                // .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
                // .alpha_blend_op(vk::BlendOp::ADD)
                
                .blend_enable(true)
                .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
                .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
                .color_blend_op(vk::BlendOp::ADD)
                .src_alpha_blend_factor(vk::BlendFactor::ONE)
                .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
                .alpha_blend_op(vk::BlendOp::ADD)
        ];

        let color_blending = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .attachments(&color_blend_attachment)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);

        let dynamic_states= [
            vk::DynamicState::VIEWPORT,
            vk::DynamicState::LINE_WIDTH,
        ];

        let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::default()
            .dynamic_states(&dynamic_states);

        let binding = [descriptor_set_layout];
        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&binding);
            // .push_constant_ranges()

        let pipeline_layout = unsafe { device.create_pipeline_layout(&pipeline_layout_info, None).expect("Pipeline Layout Creation Failed !") };

        let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasteriser)
            .multisample_state(&multisampler)
            // .depth_stencil_state()
            .color_blend_state(&color_blending)
            // .dynamic_state()
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0);
            // .base_pipeline_handle()
            // .base_pipeline_index();

        let graphics_pipeline = unsafe { device.create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None).expect("Graphics Pipeline Creation Failed !") };

        unsafe { 
            device.destroy_shader_module(vert_shader_module, None);
            device.destroy_shader_module(frag_shader_module, None);
        };

        return (pipeline_layout, graphics_pipeline[0]);
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


    fn create_command_buffers(device: &ash::Device, swapchain_frame_buffers: &Vec<vk::Framebuffer>, command_pool: vk::CommandPool, render_pass: vk::RenderPass, swapchain_extent: vk::Extent2D, pipeline_layout: vk::PipelineLayout, graphics_pipeline: vk::Pipeline, vertex_buffer: vk::Buffer, index_buffer: vk::Buffer, descriptor_sets: &Vec<vk::DescriptorSet>) -> Vec<vk::CommandBuffer> {

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(swapchain_frame_buffers.len() as u32);

        let command_buffers = unsafe { device.allocate_command_buffers(&command_buffer_allocate_info).expect("Command Buffer Allocation Failed !") };


        for (i, buffer) in command_buffers.iter().enumerate() {
            
            let command_buffer_begin_info = vk::CommandBufferBeginInfo::default()
                .flags(vk::CommandBufferUsageFlags::empty());
                //.inheritance_info();

            unsafe { device.begin_command_buffer(*buffer, &command_buffer_begin_info).expect("Begin Command Buffer Error !") };
            
            let clear_values = [vk::ClearValue::default()];

            let render_pass_begin_info = vk::RenderPassBeginInfo::default()
                .render_pass(render_pass)
                .framebuffer(swapchain_frame_buffers[i])
                .render_area(vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: swapchain_extent })
                .clear_values(&clear_values);

            unsafe { device.cmd_begin_render_pass(*buffer, &render_pass_begin_info, vk::SubpassContents::INLINE) };

            unsafe { device.cmd_bind_pipeline(*buffer, vk::PipelineBindPoint::GRAPHICS, graphics_pipeline) };

            let vertex_buffers = [vertex_buffer];
            let offsets = &[0 as u64];

            unsafe { device.cmd_bind_vertex_buffers(*buffer, 0, &vertex_buffers, offsets) };
            unsafe { device.cmd_bind_index_buffer(*buffer, index_buffer, 0, vk::IndexType::UINT16) };
            unsafe { device.cmd_bind_descriptor_sets(*buffer, vk::PipelineBindPoint::GRAPHICS, pipeline_layout, 0, &[descriptor_sets[i as usize]], &[]); }

            unsafe { device.cmd_draw_indexed(*buffer, Self::INDICES.len() as u32, 1, 0, 0, 0); }

            unsafe { device.cmd_end_render_pass(*buffer) };

            unsafe { device.end_command_buffer(*buffer).expect("End Command Buffer Error !") };

        }

        return command_buffers;
    }



    fn create_vertex_buffer(instance: &ash::Instance, device: &ash::Device, physical_device: ash::vk::PhysicalDevice, command_pool: vk::CommandPool, graphics_queue: vk::Queue) -> (vk::Buffer, vk::DeviceMemory) {

        let buffer_size = (std::mem::size_of::<VulkanVertex>() * Self::VERTICES.len()) as u64;

        let (staging_buffer, staging_buffer_memory) = Self::create_buffer(instance, device, physical_device, buffer_size, vk::BufferUsageFlags::TRANSFER_SRC, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);

        unsafe {
            let data = device.map_memory(staging_buffer_memory, 0, buffer_size, vk::MemoryMapFlags::empty())
                .expect("Map Memory Failed !") as *mut VulkanVertex;
            
            std::ptr::copy_nonoverlapping(Self::VERTICES.as_ptr(), data, Self::VERTICES.len());

            device.unmap_memory(staging_buffer_memory);
        }

        let (vertex_buffer, vertex_buffer_memory) = Self::create_buffer(instance, device, physical_device, buffer_size, vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER, vk::MemoryPropertyFlags::DEVICE_LOCAL);

        Self::copy_buffer(device, staging_buffer, vertex_buffer, buffer_size, command_pool, graphics_queue);

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        }

        return (vertex_buffer, vertex_buffer_memory);
    }

    fn create_index_buffer(instance: &ash::Instance, device: &ash::Device, physical_device: ash::vk::PhysicalDevice, command_pool: vk::CommandPool, graphics_queue: vk::Queue) -> (vk::Buffer, vk::DeviceMemory) {

        let buffer_size = (std::mem::size_of::<u16>() * Self::INDICES.len()) as u64;

        let (staging_buffer, staging_buffer_memory) = Self::create_buffer(instance, device, physical_device, buffer_size, vk::BufferUsageFlags::TRANSFER_SRC, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);

        unsafe {
            let data = device.map_memory(staging_buffer_memory, 0, buffer_size, vk::MemoryMapFlags::empty())
                .expect("Map Memory Failed !") as *mut u16;
            
            std::ptr::copy_nonoverlapping(Self::INDICES.as_ptr(), data, Self::INDICES.len());

            device.unmap_memory(staging_buffer_memory);
        }

        let (index_buffer, index_buffer_memory) = Self::create_buffer(instance, device, physical_device, buffer_size, vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER, vk::MemoryPropertyFlags::DEVICE_LOCAL);

        Self::copy_buffer(device, staging_buffer, index_buffer, buffer_size, command_pool, graphics_queue);

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        }

        return (index_buffer, index_buffer_memory);

    }

    fn create_uniform_buffer(instance: &ash::Instance, device: &ash::Device, physical_device: ash::vk::PhysicalDevice, swapchain_images: &Vec<vk::Image>) -> (Vec<vk::Buffer>, Vec<vk::DeviceMemory>) {

        let buffer_size = std::mem::size_of::<VulkanUniformBufferObject>() as u64;

        let mut uniform_buffers = Vec::<vk::Buffer>::new();
        let mut uniform_buffers_memory = Vec::<vk::DeviceMemory>::new();

        for i in 0..swapchain_images.len() {
            
            let (buffer, buffer_memory) = Self::create_buffer(instance, device, physical_device, buffer_size, vk::BufferUsageFlags::UNIFORM_BUFFER, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);

            uniform_buffers.push(buffer);
            uniform_buffers_memory.push(buffer_memory);
        }

        return (uniform_buffers, uniform_buffers_memory);
    }

    fn create_descriptor_sets(device: &ash::Device, swapchain_images: &Vec<vk::Image>, uniform_buffers: &Vec<vk::Buffer>, descriptor_set_layout: vk::DescriptorSetLayout, descriptor_pool: vk::DescriptorPool) -> Vec<vk::DescriptorSet> {

        let layouts = vec![descriptor_set_layout; swapchain_images.len()];

        let allocate_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(descriptor_pool)
            .set_layouts(&layouts);

        let descriptor_sets = unsafe { device.allocate_descriptor_sets(&allocate_info).expect("Descriptor Set Allocation Failed !") };

        for i in 0..swapchain_images.len() {

            let buffer_info = [
                vk::DescriptorBufferInfo::default()
                    .buffer(uniform_buffers[i])
                    .offset(0)
                    .range(std::mem::size_of::<VulkanUniformBufferObject>() as u64)
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

        // let mut image_available_semaphore: Vec<vk::Semaphore> = Vec::new();
        let mut acquire_semaphores: Vec<vk::Semaphore> = Vec::new();
        let mut render_finished_semaphore: Vec<vk::Semaphore> = Vec::new();
        let mut in_flight_fence: Vec<vk::Fence> = Vec::new();
        let images_in_flight = vec![vk::Fence::null(); swapchain_images.len()];

        let semaphore_info = vk::SemaphoreCreateInfo::default();

        let fence_info = vk::FenceCreateInfo::default()
            .flags(vk::FenceCreateFlags::SIGNALED);

        for i in 0..swapchain_images.len() {
            
            // let semaphore_1 = unsafe { device.create_semaphore(&semaphore_info, None).expect("Semaphore Creation Failed !") };
            let semaphore_2 = unsafe { device.create_semaphore(&semaphore_info, None).expect("Semaphore Creation Failed !") };
            
            // image_available_semaphore.push(semaphore_1);
            render_finished_semaphore.push(semaphore_2);
            
        }

        for i in 0..Self::MAX_FRAMES_IN_FLIGHT {

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

        let (pipeline_layout, graphics_pipeline) = Self::create_graphics_pipeline(&self.device, self.swapchain_extent, self.descriptor_set_layout, self.render_pass);
        self.pipeline_layout = pipeline_layout;
        self.graphics_pipeline = graphics_pipeline;


        let swapchain_frame_buffers = Self::create_frame_buffers(&self.device, &self.swapchain_image_views, self.render_pass, self.swapchain_extent);
        self.swapchain_frame_buffers = swapchain_frame_buffers;

        let descriptor_pool = Self::create_descriptor_pool(&self.device, &self.swapchain_images);
        self.descriptor_pool = descriptor_pool;

        let descriptor_sets = Self::create_descriptor_sets(&self.device, &self.swapchain_images, &self.uniform_buffers, self.descriptor_set_layout, self.descriptor_pool);
        self.descriptor_sets = descriptor_sets;
        
        let command_buffers = Self::create_command_buffers(&self.device, &self.swapchain_frame_buffers, self.command_pool, self.render_pass, self.swapchain_extent, self.pipeline_layout, self.graphics_pipeline, self.vertex_buffer, self.index_buffer, &self.descriptor_sets);
        self.command_buffers = command_buffers;

        let (acquire_semaphores, render_finished_semaphore, in_flight_fence, images_in_flight) = Self::create_sync_objects(&self.device, &self.swapchain_images);
        // self.image_available_semaphore = image_available_semaphore;
        self.acquire_semaphores = acquire_semaphores;
        self.render_finished_semaphore = render_finished_semaphore;
        self.in_flight_fence = in_flight_fence;
        self.images_in_flight = images_in_flight;
        

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


    const VERTICES: &[VulkanVertex] = &[
        VulkanVertex { pos: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
        VulkanVertex { pos: [ 0.5, -0.5], color: [0.0, 1.0, 0.0] },
        VulkanVertex { pos: [ 0.5,  0.5], color: [0.0, 0.0, 1.0] },
        VulkanVertex { pos: [-0.5,  0.5], color: [1.0, 1.0, 1.0] },
    ];

    // 2D
    // const VERTICES: &[VulkanVertex] = &[
    //     VulkanVertex { pos: [100.0, 100.0], color: [1.0, 0.0, 0.0] },
    //     VulkanVertex { pos: [500.0, 100.0], color: [0.0, 1.0, 0.0] },
    //     VulkanVertex { pos: [500.0, 500.0], color: [0.0, 0.0, 1.0] },
    //     VulkanVertex { pos: [100.0, 500.0], color: [1.0, 1.0, 1.0] },
    // ];

    const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

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

    pub fn draw_image(&mut self) {

        if self.is_paused { return; }

        unsafe { self.device.wait_for_fences(&[self.in_flight_fence[self.current_frame as usize]], true, u64::MAX).expect("Fence Waiting Failed !") };

        let acquire_semaphore = self.acquire_semaphores[self.current_frame as usize];

        let result = unsafe { self.swapchain_loader.acquire_next_image(self.swapchain, u64::MAX, acquire_semaphore, vk::Fence::null()) };

        // if self.recreation_needed {
        //     self.recreation_needed = false;
        //     self.recreate_swapchain(&self.raw_handle.clone());
        //     return;
        // }

        let image_index = match result {
            Ok((index, false)) => index,
            Ok((_, true)) => { self.recreate_swapchain((self.swapchain_extent.width, self.swapchain_extent.height)); return; }
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => { self.recreate_swapchain((self.swapchain_extent.width, self.swapchain_extent.height)); return; },
            Err(_) => panic!("Next Image Failed !")
        };

        self.re_create_command_buffer(image_index);

        if self.images_in_flight[image_index as usize] != vk::Fence::null() {
            unsafe { self.device.wait_for_fences(&[self.images_in_flight[image_index as usize]], true, u64::MAX).expect("Fence Waiting Failed !") };
        }

        self.images_in_flight[image_index as usize] = self.in_flight_fence[self.current_frame as usize];

        // Update Uniform Buffer
        self.update_uniform_buffer(image_index as usize);
        
        let wait_semaphores = [acquire_semaphore];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [self.render_finished_semaphore[image_index as usize]];
        
        let command_buffers = [self.command_buffers[image_index as usize]];
        let submit_info = vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);

        unsafe { self.device.reset_fences(&[self.in_flight_fence[self.current_frame as usize]]).expect("Fences Reset Failed !") };

        unsafe { self.device.queue_submit(self.graphics_queue,&[submit_info], self.in_flight_fence[self.current_frame as usize]).expect("Queue Submit Failed !") };

        let swapchains = [self.swapchain];

        let indices = [image_index];
        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&indices);

        let result = unsafe { self.swapchain_loader.queue_present(self.present_queue, &present_info) };

        match result {
            Ok(_) => {},
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => self.recreate_swapchain((self.swapchain_extent.width, self.swapchain_extent.height)),
            Err(vk::Result::SUBOPTIMAL_KHR) => self.recreate_swapchain((self.swapchain_extent.width, self.swapchain_extent.height)),
            Err(_) => panic!("Next Image Failed !")
        };

        self.current_frame = (self.current_frame + 1) % Self::MAX_FRAMES_IN_FLIGHT;

    }


    fn update_uniform_buffer(&self, current_image: usize) {

        let time = self.start_time.elapsed().as_secs_f32();

        let ubo = VulkanUniformBufferObject {

            model: Mat4::from_rotation_z(time * 90.0f32.to_radians()) * Mat4::from_rotation_y(time * 90.0f32.to_radians()) * Mat4::from_rotation_x(-time * 90.0f32.to_radians()),
            // 2D
            //model: Mat4::IDENTITY,
            view: Mat4::look_at_rh(
            Vec3::new(2.0, 2.0, 2.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0)
            ),
            // 2D
            //view: Mat4::IDENTITY,
            proj: {

                let mut proj = Mat4::perspective_rh(
                45.0f32.to_radians(),
                self.swapchain_extent.width as f32 / self.swapchain_extent.height as f32,
                0.1,
                10.0
                );
                // 2D
                // let mut proj = Mat4::orthographic_rh(
                // 0.0,
                // self.swapchain_extent.width as f32,
                // self.swapchain_extent.height as f32,
                // 0.0,
                // -1.0,
                // 1.0,
                // );
                proj.y_axis.y *= -1.0; // à supprimer pour la 2D
                proj

            }

        };

        unsafe {
            let data = self.device.map_memory(
                self.uniform_buffers_memory[current_image],
                0,
                std::mem::size_of::<VulkanUniformBufferObject>() as u64,
                vk::MemoryMapFlags::empty(),
            ).unwrap() as *mut VulkanUniformBufferObject;
            data.write(ubo);
            self.device.unmap_memory(self.uniform_buffers_memory[current_image]);
        }

    } 


    pub fn begin_draw(&mut self) {

        if self.is_paused { return; }

        // unsafe { self.device.wait_for_fences(&[self.in_flight_fence[self.current_frame as usize]], true, u64::MAX).expect("Fence Waiting Failed !") };

        // self.re_create_command_buffer();

    }

    fn re_create_command_buffer(&mut self, image_index: u32) {

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

        unsafe { self.device.cmd_bind_pipeline(buffer, vk::PipelineBindPoint::GRAPHICS, self.graphics_pipeline) };

        let vertex_buffers = [self.vertex_buffer];
        let offsets = &[0 as u64];

        unsafe { self.device.cmd_bind_vertex_buffers(buffer, 0, &vertex_buffers, offsets) };
        unsafe { self.device.cmd_bind_index_buffer(buffer, self.index_buffer, 0, vk::IndexType::UINT16) };
        unsafe { self.device.cmd_bind_descriptor_sets(buffer, vk::PipelineBindPoint::GRAPHICS, self.pipeline_layout, 0, &[self.descriptor_sets[image_index as usize]], &[]); }

        unsafe { self.device.cmd_draw_indexed(buffer, Self::INDICES.len() as u32, 1, 0, 0, 0); }

        unsafe { self.device.cmd_end_render_pass(buffer) };

        unsafe { self.device.end_command_buffer(buffer).expect("End Command Buffer Error !") };

        

    }


}


/// Destroy
impl VulkanRenderer {

    fn destroy_swapchain(&self) {

        unsafe {

            for i in 0..Self::MAX_FRAMES_IN_FLIGHT as usize {
                self.device.destroy_fence(self.in_flight_fence[i], None);
                self.device.destroy_semaphore(self.acquire_semaphores[i], None);
            }

            for i in 0..self.swapchain_images.len() as usize {
                self.device.destroy_semaphore(self.render_finished_semaphore[i], None);
                // self.device.destroy_semaphore(self.image_available_semaphore[i], None);
            }

            self.device.destroy_descriptor_pool(self.descriptor_pool, None);

            self.device.destroy_pipeline(self.graphics_pipeline, None);
            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
    
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

            // for i in 0..Self::MAX_FRAMES_IN_FLIGHT as usize {
            //     self.device.destroy_fence(self.in_flight_fence[i], None);
            //     self.device.destroy_semaphore(self.render_finished_semaphore[i], None);
            //     self.device.destroy_semaphore(self.image_available_semaphore[i], None);
            // }

            // self.device.destroy_descriptor_pool(self.descriptor_pool, None);

            for i in 0..self.uniform_buffers_memory.len() {
                self.device.destroy_buffer(self.uniform_buffers[i], None);
                self.device.free_memory(self.uniform_buffers_memory[i], None);
            }
            
            self.device.destroy_buffer(self.index_buffer, None);
            self.device.free_memory(self.index_buffer_memory, None);
            self.device.destroy_buffer(self.vertex_buffer, None);
            self.device.free_memory(self.vertex_buffer_memory, None);

            self.device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            self.device.destroy_command_pool(self.command_pool, None);

            self.destroy_swapchain();

            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }

    }

}