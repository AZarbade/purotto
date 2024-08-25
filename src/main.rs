mod datacontainer;
mod reader;
mod wrap_app;

use crate::datacontainer::DataContainer;
use eframe::egui;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

#[derive(Default)]
struct AppConfig {
    dark_mode: bool,
}

#[derive(Default)]
struct App {
    payload: Arc<Mutex<DataContainer>>,
    config: AppConfig,
    plot_tracker: HashMap<String, bool>,
}

impl App {
    fn render_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add(egui::widgets::Label::new("Plotter"));
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let close_btn = ui.add(egui::Button::new("\u{274C}"));
                    if close_btn.clicked() {
                        todo!();
                    }
                    let theme_btn = ui.add(egui::Button::new("change theme"));
                    if theme_btn.clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
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
        let stream_count = self.payload.lock().unwrap().stream_count;
        if self.plot_tracker.is_empty() {
            for i in 0..stream_count {
                self.plot_tracker.insert(format!("Stream_{i}"), false);
            }
        }
        for i in 0..stream_count {
            let stream_key = format!("Stream_{i}");
            if let Some(is_plotted) = self.plot_tracker.get_mut(&stream_key) {
                ui.add(egui::Checkbox::new(is_plotted, &stream_key));
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.config.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        self.render_top_panel(ctx);
        self.render_left_panel(ctx);
        egui::CentralPanel::default().show(ctx, |_| {
            let stream_count = self.payload.lock().unwrap().stream_count;
            for i in 0..stream_count {
                let stream_key = format!("Stream_{i}");
                if let Some(is_plotted) = self.plot_tracker.get_mut(&stream_key) {
                    if *is_plotted {
                        let data = self.payload.lock().unwrap().get_plotpoints(i);
                        let mut inner_applets = wrap_app::MovingStrings { stream_index: i };
                        inner_applets.show(ctx, data);
                    }
                }
            }
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
        config: AppConfig::default(),
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
