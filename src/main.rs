use bus::{Bus, BusReader};
use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use std::io::{self, BufRead, Stdin};
use std::thread;

fn stdin_reader(stdin: Stdin) -> BusReader<Vec<f64>> {
    let mut bus = Bus::new(100);
    let rx = bus.add_rx();
    thread::spawn(move || {
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                let data: Vec<f64> = line
                    .split_whitespace()
                    .filter_map(|x| x.parse::<f64>().ok())
                    .collect();
                if !data.is_empty() {
                    bus.broadcast(data);
                }
            }
        }
    });
    return rx;
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let mut plot_rect = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Value Plotting");

            // let plot = Plot::new("Plot").legend(Legend::default());
            // let inner = plot.show(ui, |plot_ui| {
            //     plot_ui.line(Line::new(PlotPoints::from_ys_f32(&rx.recv().unwrap()[..1])));
            // });
            // plot_rect = Some(inner.response.rect);
            ui.ctx().request_repaint();
        });
    }
}

fn main() {
    // return eframe::run_native(
    //     "My egui App with a plot",
    //     eframe::NativeOptions::default(),
    //     Box::new(|_cc| Ok(Box::<MyApp>::default())),
    // )
    // .expect("app");
}
