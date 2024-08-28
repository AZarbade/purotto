mod datacontainer;
mod reader;
mod wrap_app;

use crate::datacontainer::DataContainer;
use eframe::egui;
use egui_plot::{Line, Plot};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

#[derive(Default)]
struct App {
    payload: Arc<Mutex<DataContainer>>,
    // config options
    label: String,
    dark_mode: bool,
}

impl App {
    fn render_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add(egui::widgets::Label::new(&self.label));
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let close_btn = ui.add(egui::Button::new("close"));
                    if close_btn.clicked() {
                        todo!();
                    }
                    let theme_btn = ui.add(egui::Button::new("change theme"));
                    if theme_btn.clicked() {
                        self.dark_mode = !self.dark_mode;
                    }
                });
            });
            ui.add_space(10.);
        });
    }

    fn render_left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left panel").show(ctx, |ui| {
            ui.add_space(10.);
            ui.label("Available Streams");
            ui.separator();
            self.create_stream_toggles(ui);
        });
    }

    fn create_stream_toggles(&mut self, ui: &mut egui::Ui) {
        let mut payload = self.payload.lock().unwrap();
        payload.update_tracker();
        for i in 0..payload.stream_count {
            let stream_key = format!("Stream_{i}");
            if let Some(is_enabled) = payload.plot_tracker.get_mut(&stream_key) {
                ui.add(egui::Checkbox::new(is_enabled, &stream_key));
            }
        }
    }

    pub fn render_plot(&mut self, ctx: &egui::Context) {
        egui::Window::new("").show(ctx, |ui| {
            ui.ctx().request_repaint();
            let payload = self.payload.lock().unwrap();
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

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        egui::CentralPanel::default().show(ctx, |_| {
            self.render_top_panel(ctx);
            self.render_left_panel(ctx);
            self.render_plot(ctx);
        });
    }
}

fn main() {
    let mut thread_handles: Vec<JoinHandle<()>> = Vec::new();
    let stdin = std::io::stdin();
    let (data, read_handle) = reader::stdin_parser(stdin);
    thread_handles.push(read_handle);

    let applet = Box::<App>::new(App {
        payload: data,
        label: "Some nice name".to_string(),
        ..Default::default()
    });

    eframe::run_native(
        "My egui App with a plot",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(applet)),
    )
    .unwrap();

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
