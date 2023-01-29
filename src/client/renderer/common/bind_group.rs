pub struct BindGroup {
    pub group: wgpu::BindGroup,
    pub layout: Option<Layout>,
}

impl BindGroup {
    pub fn new(
        device: &wgpu::Device,
        layout_entries: &[wgpu::BindGroupLayoutEntry],
        group_entries: &[wgpu::BindGroupEntry],
        label: String) -> Self {

        let layout = Layout::new(
            device,
            layout_entries,
            &(label.clone() + "Bind Group Layout"),
        );

        let group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &layout.0,
                entries: group_entries,
                label: Some(&(label + "Bind Group")),
            }
        );

        Self {
            group,
            layout: Some(layout),
        }
    }

    pub fn _from_layout(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        group_entries: &[wgpu::BindGroupEntry],
        label: String) -> Self {

        let group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout,
                entries: group_entries,
                label: Some(&(label + "Bind Group Layout")),
            }
        );

        Self {
            group,
            layout: None,
        }
    }

    pub fn uniform_bg(
        device: &wgpu::Device,
        group_entries: &[wgpu::BindGroupEntry],
        label: String,
    ) -> Self {
        let layout = Layout::uniform_layout(
            device,
            &(label.clone() + "Bind Group Layout"),
        );

        let group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &layout.0,
                entries: group_entries,
                label: Some(&(label + "Bind Group")),
            }
        );

        Self {
            group,
            layout: Some(layout),
        }
    }
}

pub struct Layout(pub wgpu::BindGroupLayout);

impl Layout {
    pub fn new(
        device: &wgpu::Device,
        entries: &[wgpu::BindGroupLayoutEntry],
        label: &str,
    ) -> Self {
        Self(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries,
            label: Some(label),
        }))
    }

    pub fn uniform_layout(
        device: &wgpu::Device,
        label: &str,
    ) -> Self {
        Self(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some(label),
        }))
    }
}