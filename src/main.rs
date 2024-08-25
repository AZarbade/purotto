mod datacontainer;
mod reader;

use crate::datacontainer::DataContainer;
use eframe::egui;
use egui_plot::{Legend, Line, Plot};
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct MyAppConfig {
    dark_mode: bool,
}

#[derive(Default)]
struct App {
    payload: Arc<Mutex<DataContainer>>,
    config: MyAppConfig,
}

impl App {
    fn render_plot(&self, ui: &mut egui::Ui, index: usize) {
        let data = self.payload.lock().unwrap();
        let my_plot = Plot::new("My Plot").legend(Legend::default());
        let _inner = my_plot.show(ui, |ui| {
            ui.line(Line::new(data.get_plotpoints(index)).name("curve"));
        });
    }

    fn render_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add(egui::widgets::Label::new("Plotter"));
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let close_btn = ui.add(egui::Button::new("close"));
                    if close_btn.clicked() {
                        todo!();
                    }
                    let refresh_btn = ui.add(egui::Button::new("refresh"));
                    if refresh_btn.clicked() {
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

    fn render_left_panel(&self, ctx: &egui::Context) {
        egui::SidePanel::left("left panel").show(ctx, |ui| {
            ui.add_space(10.);
            ui.label("Available Streams");
            ui.separator();
            let stream_count = self.payload.lock().unwrap().stream_count;
            for i in 0..stream_count {
                ui.add(egui::Button::new(format!("Stream_{i}")));
            }
        });
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
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_plot(ui, 0);
            ctx.request_repaint();
        });
    }
}

fn main() {
    let mut thread_handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
    let stdin = std::io::stdin();
    let (data, read_handle) = reader::stdin_parser(stdin);
    thread_handles.push(read_handle);

    let applet = Box::<App>::new(App {
        payload: data,
        config: MyAppConfig::default(),
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
