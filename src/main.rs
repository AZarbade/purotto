use bus::BusReader;
use eframe::egui::{self, Response};
use egui_plot::{Legend, Line, LineStyle, Plot, PlotPoints};
use std::io;

mod utils;

#[derive(Default)]
struct MainApp {}

struct LineDemo {
    time: f64,
    show_axes: bool,
    show_grid: bool,
    line_style: LineStyle,
    receiver: BusReader<Vec<f64>>,
}

impl Default for LineDemo {
    fn default() -> Self {
        Self {
            time: 0.0,
            show_axes: true,
            show_grid: true,
            line_style: LineStyle::Solid,
            receiver,
        }
    }
}

impl LineDemo {
    fn plot_data(&self) -> Line {
        todo!();
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        let plot = Plot::new("Line Demo")
            .legend(Legend::default())
            .show_axes(self.show_axes)
            .show_grid(self.show_grid);

        let response = plot
            .show(ui, |plot_ui| {
                plot_ui.line(self.plot_data());
            })
            .response;
        return response;
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}

fn main() {
    let mut rx = utils::stdin_reader(io::stdin());
    loop {
        let data = rx.recv().unwrap();
        println!("{:?}", &data);
    }

    // return eframe::run_native(
    //     "plotter",
    //     eframe::NativeOptions::default(),
    //     Box::new(|_cc| Ok(Box::<MainApp>::default())),
    // )
    // .unwrap();
}
