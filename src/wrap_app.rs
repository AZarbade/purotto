#![allow(dead_code)]

use crate::datacontainer::DataContainer;
use eframe::egui::{self, vec2};
use egui_plot::{Line, Plot};
use std::sync::{Arc, Mutex};

// ----------------------------------------------------------------------------

#[derive(Default)]
pub struct Showcase {
    pub data: Arc<Mutex<DataContainer>>,
}

impl Showcase {
    fn name(&self) -> &str {
        "Showcase"
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new(self.name())
            .default_size(vec2(512., 256.))
            .vscroll(false)
            .show(ctx, |ui| self.ui(ui));
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            ui.ctx().request_repaint();
            let payload = self.data.lock().unwrap();
            let stream_count = payload.stream_count;
            for index in 0..stream_count {
                let stream_key = format!("Stream_{index}");
                if let Some(is_plotted) = payload.plot_tracker.get(&stream_key) {
                    if *is_plotted {
                        let data = payload.get_plotpoints(index);
                        let plot = Plot::new("plot").view_aspect(2.0);
                        plot.show(ui, |ui| {
                            ui.line(Line::new(data));
                        });
                    }
                }
            }
        });
    }
}

// ----------------------------------------------------------------------------
