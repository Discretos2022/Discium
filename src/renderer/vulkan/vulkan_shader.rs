use ash::vk;


pub struct VulkanShader {
    pub shader_stage: vk::ShaderStageFlags,
    pub module: vk::ShaderModule,
}


impl VulkanShader {

    pub fn create(path: &str, shader_stage: vk::ShaderStageFlags, device: &ash::Device) -> Self {

        let bytes = std::fs::read(path).expect("Shader Loading Was Failed !");
        let code = ash::util::read_spv(&mut std::io::Cursor::new(bytes.as_slice().as_ref())).unwrap();
        let info = vk::ShaderModuleCreateInfo::default()
            .code(code.as_slice());

        let shader_module = unsafe { device.create_shader_module(&info, None).expect("Shader Module Creation Failed !") };

        return Self {
            shader_stage: shader_stage,
            module: shader_module,
        };

    }


    pub fn destroy(&mut self, device: &ash::Device) {
        unsafe { device.destroy_shader_module(self.module, None) };
    }

}