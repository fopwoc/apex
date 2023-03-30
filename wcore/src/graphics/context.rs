use egui_winit::winit::dpi::PhysicalSize;

pub struct Graphics {
    pub device  : wgpu::Device,
    pub surface : wgpu::Surface,
    pub queue   : wgpu::Queue,
    pub format  : wgpu::TextureFormat,
    pub config  : wgpu::SurfaceConfiguration,

    pub size    : PhysicalSize<u32>,
    pub scale   : f64,
}