//! # Data Visualization Application
//!
//! This application provides real-time visualization of data streams using egui for the graphical interface.
//! It reads data from standard input, processes it, and displays it in interactive plots.
//!
//! ## Main Components
//!
//! - `App`: The main application struct that handles the UI and data management.
//! - `datacontainer`: A module for managing and storing data streams.
//! - `reader`: A module for reading and parsing input data.

mod datacontainer;
mod reader;

use crate::datacontainer::DataContainer;
use eframe::egui::{self, vec2};
use egui_plot::{Line, Plot};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

/// The main application struct.
///
/// This struct holds the application state and is responsible for rendering the UI.
#[derive(Default)]
struct App {
    /// The shared data container, wrapped in an Arc<Mutex<>> for thread-safe access.
    payload: Arc<Mutex<DataContainer>>,
    // config options
    /// The label displayed in the top panel of the application.
    label: String,
    /// A flag to toggle between light and dark mode.
    dark_mode: bool,
}

impl App {
    /// Renders the top panel of the application.
    ///
    /// This panel contains the application label and buttons for closing the app and toggling the theme.
    fn render_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add(egui::widgets::Label::new(&self.label));
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let close_btn =
                        ui.add(egui::Button::new(egui_phosphor::regular::X.to_string()));
                    if close_btn.clicked() {
                        todo!();
                    }
                    let theme_btn =
                        ui.add(egui::Button::new(egui_phosphor::regular::SUN.to_string()));
                    if theme_btn.clicked() {
                        self.dark_mode = !self.dark_mode;
                    }
                });
            });
            ui.add_space(10.);
        });
    }

    /// Renders the bottom panel of the application.
    ///
    /// This panel contains a button to auto-organize the layout.
    fn render_bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    if ui.button("Auto Organize").clicked() {
                        ui.ctx().memory_mut(|mem| mem.reset_areas());
                    }
                });
            });
            ui.add_space(10.);
        });
    }

    /// Renders the left panel of the application.
    ///
    /// This panel displays the available data streams and allows toggling their visibility.
    fn render_left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left panel").show(ctx, |ui| {
            ui.add_space(10.);
            ui.label("Available Streams");
            ui.separator();
            self.create_stream_toggles(ui);
        });
    }

    /// Creates toggles for each data stream in the UI.
    ///
    /// This function is called by `render_left_panel` to create checkboxes for each stream.
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

    /// Renders the plots for each active data stream.
    ///
    /// This function creates a separate window for each stream that is toggled on,
    /// displaying its data in a line plot.
    pub fn render_plot(&mut self, ctx: &egui::Context) {
        let payload = self.payload.lock().unwrap();
        let stream_count = payload.stream_count;
        for index in 0..stream_count {
            let stream_key = format!("Stream_{index}");
            if let Some(is_plotted) = payload.plot_tracker.get(&stream_key) {
                if *is_plotted {
                    let stream_id = egui::Id::new(stream_key);
                    egui::Window::new("")
                        .id(stream_id)
                        .default_size(vec2(512., 256.))
                        .show(ctx, |ui| {
                            ui.ctx().request_repaint();
                            let data = payload.get_plotpoints(index);
                            let plot = Plot::new("plot");
                            plot.show(ui, |ui| {
                                ui.line(Line::new(data));
                            });
                        });
                }
            }
        }
    }
}

impl eframe::App for App {
    /// The main update loop for the application.
    ///
    /// This function is called by the eframe runtime to update and render the application state.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        ctx.set_fonts(fonts);

        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        egui::CentralPanel::default().show(ctx, |_| {
            self.render_top_panel(ctx);
            self.render_left_panel(ctx);
            self.render_bottom_panel(ctx);
            self.render_plot(ctx);
        });
    }
}

/// The main entry point of the application.
///
/// This function sets up the data reading thread, initializes the application state,
/// and starts the eframe runtime.
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
