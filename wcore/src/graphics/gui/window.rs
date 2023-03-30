use egui::WidgetText;

use crate::graphics::context::Graphics;

use super::view::View;

pub trait Window<State>: View<State> {
    type Title: Into<WidgetText>;
    fn title() -> Self::Title;

    #[allow(unused_variables)]
    fn build<'a>(window: egui::Window<'a>, ctx: &'_ egui::Context) -> egui::Window<'a> { window }

    #[allow(unused_variables)]
    fn show(&mut self, state: State, view: &wgpu::TextureView, graphics: &mut Graphics, ui: &mut egui::Ui);

    fn set_visible(&mut self, _value: bool) {}
    fn get_visible(&self) -> bool { true }

    fn toggle_visible(&mut self) { self.set_visible(!self.get_visible()); }
}

impl<T: Window<State>, State> View<State> for T  {
    default fn show(&mut self, state: State, view: &wgpu::TextureView, graphics: &mut Graphics, ctx: &egui::Context) {
        let mut show_startup = self.get_visible();
        Self::build(egui::Window::new(Self::title()), ctx)
          .open(&mut show_startup)
          .show(ctx, |ui| {
            Window::show(self, state, view, graphics, ui);
        });

        self.set_visible(self.get_visible() && show_startup);
    }
}