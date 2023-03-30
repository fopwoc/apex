use egui::ClippedPrimitive;
use egui_winit::winit::{event_loop::EventLoop, window::Window};
use egui_winit::winit::{event::WindowEvent, dpi::PhysicalSize};
use wgpu::CommandBuffer;

use crate::graphics::context::Graphics;

pub struct Egui {
    pub context     : egui::Context,
    pub renderer    : egui_wgpu::Renderer,
    pub screen_desc : egui_wgpu::renderer::ScreenDescriptor,
    pub winit_state : egui_winit::State,
}

impl Egui {
    pub fn new<T>(event_loop: &EventLoop<T>, graphics: &Graphics, width: u32, height: u32, scale: f64) -> Self {
        let context = egui::Context::default();
        let renderer = egui_wgpu::Renderer::new(&graphics.device, graphics.format, None, 1);
        let screen_desc = egui_wgpu::renderer::ScreenDescriptor {
            size_in_pixels   : [width, height],
            pixels_per_point : scale as f32,
        };

        #[allow(unused_mut)]
        let mut winit_state = egui_winit::State::new(event_loop);

        #[allow(unused_must_use)] { // Not all platforms notify us about scale
            winit_state.on_event(&context, &WindowEvent::ScaleFactorChanged {
                scale_factor   : scale,
                new_inner_size : &mut PhysicalSize::from((width, height))
            });
        }

        return Self {
            context,
            renderer,
            screen_desc,
            winit_state,
        }
    }

    pub fn prepare(&mut self, window: &Window, graphics: &mut Graphics, encoder: &mut wgpu::CommandEncoder, run_ui: impl FnOnce(&mut Graphics, &egui::Context)) -> (Vec<ClippedPrimitive>, Vec<wgpu::CommandBuffer>){
        let egui_output = self.context.run(self.winit_state.take_egui_input(window), |ui| run_ui(graphics, ui));
        
        // Free textures
        for id in &egui_output.textures_delta.free {
            self.renderer.free_texture(id);
        }

        // Upload textures
        for (id, image_delta) in &egui_output.textures_delta.set {
            self.renderer.update_texture(
                &graphics.device,
                &graphics.queue,
                *id,
                image_delta,
            );
        }

        // Generate vertices and render commands
        let clipped_primitives = self.context.tessellate(egui_output.shapes);
        let commands = self.renderer.update_buffers(
            &graphics.device,
            &graphics.queue,
            encoder,
            &clipped_primitives,
            &self.screen_desc,
        );

        return (clipped_primitives, commands);
    }

    pub fn render<'this: 'pass, 'pass>(&'this mut self, graphics: &Graphics, render_pass: &mut wgpu::RenderPass<'pass>, clipped_primitives: &[ClippedPrimitive], commands: Vec<CommandBuffer>) {
        self.renderer.render(render_pass, clipped_primitives, &self.screen_desc);
        graphics.queue.submit(commands);
    }

    pub fn scale(&mut self, scale: f64) {
        self.screen_desc.pixels_per_point = scale as f32;
        
        #[allow(unused_must_use)] { // Not all platforms notify us about scale
            self.winit_state.on_event(&self.context, &WindowEvent::ScaleFactorChanged {
                scale_factor   : scale,
                new_inner_size : &mut { let v = self.screen_desc.size_in_pixels; PhysicalSize { width: v[0], height: v[1] } }
            });
        }
    }
}