pub trait Bindable {
    fn bind<'pass, 'uniform: 'pass>(&'uniform self, render_pass: &mut wgpu::RenderPass<'pass>, index: u32);
    fn layout(&self) -> &wgpu::BindGroupLayout;
    fn group(&self) -> &wgpu::BindGroup;
}