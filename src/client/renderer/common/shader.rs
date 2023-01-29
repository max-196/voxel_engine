use {
    std::path::Path,
    super::super::err,
    crate::common::files::read_file,
};

pub struct Shader {
    pub module: wgpu::ShaderModule,
    pub entry: &'static str,
}

impl Shader{
    pub fn new(
        device: &wgpu::Device,
        label:  &'static str,
        path:   &'static str,
        entry:  &'static str,
    ) -> Result<Self, err::RendererError> {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: label.into(),
            source: wgpu::ShaderSource::Wgsl(match read_file(Path::new(path)) {
                Ok((shader, length)) => {log::info!("Created shader module '{}' with length {}, with entry point '{}', at path {}", label, length, entry, path); shader.into()},
                Err(e) => return Err(err::RendererError::ShaderCreation(e)),
            }),
        });

        Ok (Self { module, entry })
    }
}