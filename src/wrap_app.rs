use eframe::egui::{self, vec2};
use egui_plot::{Line, Plot, PlotPoints};

// ----------------------------------------------------------------------------

#[derive(Default)]
pub struct MovingStrings {
    pub stream_index: usize,
}

impl MovingStrings {
    fn name(&self) -> &str {
        "Moving Strings"
    }

    pub fn show(&mut self, ctx: &egui::Context, data: PlotPoints) {
        egui::Window::new(self.name())
            .default_size(vec2(512., 256.))
            .vscroll(false)
            .show(ctx, |ui| self.ui(ui, data));
    }

    fn ui(&mut self, ui: &mut egui::Ui, data: PlotPoints) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            ui.ctx().request_repaint();
            self.render_plot(ui, data);
        });
    }

    fn render_plot(&self, ui: &mut egui::Ui, data: PlotPoints) {
        Plot::new("stream").show(ui, |ui| {
            ui.line(Line::new(data).name(format!("Stream_{}", self.stream_index)));
        });
    }
}

// ----------------------------------------------------------------------------
