use crate::graphics::context::Graphics;

pub trait View<State> {
    fn show(&mut self, state: State, view: &wgpu::TextureView, graphics: &mut Graphics, ctx: &egui::Context);
}