use bus::{Bus, BusReader};
use eframe::egui::{self, Response};
use egui_plot::{Legend, Line, LineStyle, Plot, PlotPoints};
use std::io::{self, BufRead, Stdin};
use std::thread;

/// Reads data from standard input and broadcasts it to multiple receivers.
///
/// This function creates a specified number of receivers that will all receive the same
/// data read from standard input. Each line of input is parsed as a vector of f64 values.
///
/// # Arguments
///
/// * `stdin` - The standard input to read from.
/// * `n` - The number of receivers to create.
///
/// # Returns
///
/// A vector of `BusReader<Vec<f64>>`, each of which will receive the parsed input data.
///
/// # Example
///
/// ```
/// use std::io;
/// let mut receivers = stdin_reader(io::stdin(), 2);
/// let mut rx1 = receivers.pop().unwrap();
/// let mut rx2 = receivers.pop().unwrap();
/// ```
///
/// # Notes
///
/// - The function spawns a new thread to read from stdin continuously.
/// - Empty lines and non-numeric inputs are ignored.
/// - The bus has a capacity of 100 messages. If this limit is reached, older messages may be dropped.
fn stdin_reader(stdin: Stdin, n: usize) -> Vec<BusReader<Vec<f64>>> {
    let mut bus = Bus::new(100);
    let mut rxs = Vec::with_capacity(n);

    for _ in 0..n {
        rxs.push(bus.add_rx());
    }

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
    return rxs;
}

#[derive(Default)]
struct MainApp {}

struct LineDemo {
    time: f64,
    show_axes: bool,
    show_grid: bool,
    line_style: LineStyle,
    stream: [f64],
}

impl Default for LineDemo {
    fn default() -> Self {
        Self {
            time: 0.0,
            show_axes: true,
            show_grid: true,
            line_style: LineStyle::Solid,
            stream: ,
        }
    }
}

impl LineDemo {
    fn plot_data(&self) -> Line {
        let line = Line::new(PlotPoints::from_ys_f64(self.stream));
        return line;
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        let mut plot = Plot::new("Line Demo")
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

// TODO:
impl eframe::App for MainApp {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}

fn main() {
    let mut rxs = stdin_reader(io::stdin(), 1);
    let mut rx_1 = rxs.pop().unwrap();
    loop {
        let data = rx_1
            .recv()
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(i, y)| [i as f64, y]);
        println!("{:?}", &data);
    }

    // return eframe::run_native(
    //     "plotter",
    //     eframe::NativeOptions::default(),
    //     Box::new(|_cc| Ok(Box::<MainApp>::default())),
    // )
    // .unwrap();
}
