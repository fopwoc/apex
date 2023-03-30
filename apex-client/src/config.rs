use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// GPU rendering backend API
    #[arg(short, long, visible_alias = "api", default_value = "auto")]
    pub backend: Backend,

    /// Power preference
    #[arg(short, long, default_value = "low")]
    pub power_preference: PowerPreference,

    /// Select a specific graphics adapter
    #[arg(long)]
    pub gpu: Option<usize>,

    /// List available graphics adapters
    #[arg(long)]
    pub gpus: bool,

    /// Select a specific present mode
    #[arg(long)]
    pub mode: Option<usize>,

    /// List available present modes
    #[arg(long)]
    pub modes: bool,

    /// Force UI scaling
    #[arg(long)]
    pub scale: Option<f64>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Backend {
    /// Auto (WASM)
    Auto,
    /// Vulkan API
    Vulkan,
    /// Metal API (Apple platforms)
    Metal,
    /// Direct3D-12 (Windows)
    Dx12,
    /// Direct3D-11 (Windows)
    Dx11,
    /// OpenGL ES-3 (Linux, Android)
    Gl,
}

impl From<Backend> for wgpu::Backends {
    fn from(value: Backend) -> Self {
        return match value {
            Backend::Auto   => wgpu::Backends::all(),
            Backend::Vulkan => wgpu::Backends::VULKAN,
            Backend::Metal  => wgpu::Backends::METAL,
            Backend::Dx12   => wgpu::Backends::DX11,
            Backend::Dx11   => wgpu::Backends::DX11,
            Backend::Gl     => wgpu::Backends::GL,
        };
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PowerPreference {
    /// Low power, often an integrated GPU
    Low,
    /// Highest performance, often a discrete GPU
    High,
}

impl From<PowerPreference> for wgpu::PowerPreference {
    fn from(value: PowerPreference) -> Self {
        return match value {
            PowerPreference::Low  => wgpu::PowerPreference::LowPower,
            PowerPreference::High => wgpu::PowerPreference::HighPerformance,
        };
    }
}