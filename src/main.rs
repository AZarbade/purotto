use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use std::io::{self, BufRead, Stdin};
use std::sync::{
    mpsc::{channel, Receiver},
    Arc, Mutex,
};
use std::thread;

mod dataset;

fn stdin_reader(stdin: Stdin) -> Receiver<Vec<f32>> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                let data: Vec<f32> = line
                    .split_whitespace()
                    .filter_map(|x| x.parse::<f32>().ok())
                    .collect();
                if !data.is_empty() {
                    tx.send(data).unwrap();
                }
            }
        }
    });
    return rx;
}

fn stdin_processer(storage: Arc<Mutex<dataset::DataStore>>, rx: Receiver<Vec<f32>>) {
    loop {
        if let Ok(rx) = rx.recv() {
            storage
                .lock()
                .expect("ERROR: failed to acquire lock on storage")
                .add_entry(rx);
        }
    }
}

#[derive(Default)]
struct MyApp {
    storage: Arc<Mutex<dataset::DataStore>>,
}

impl MyApp {
    fn data_line(&self, index: usize) -> Line {
        return Line::new(PlotPoints::from_ys_f32(
            &self.storage.lock().unwrap().get_stream(index),
        ));
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut plot_rect = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Value Plotting");

            let plot = Plot::new("Plot").legend(Legend::default());
            let inner = plot.show(ui, |plot_ui| {
                plot_ui.line(self.data_line(1));
            });
            plot_rect = Some(inner.response.rect);
            ui.ctx().request_repaint();
        });
    }
}

fn main() {
    let storage = Arc::new(Mutex::new(dataset::DataStore::new()));
    let storage_socket = Arc::clone(&storage);
    thread::spawn(move || {
        let stdin = stdin_reader(io::stdin());
        stdin_processer(storage_socket, stdin);
    });

    // FIX: use different method for this
    // - sleeping to let data flow in storage
    use std::time::Duration;
    thread::sleep(Duration::from_secs(1));

    let storage_endpoint = Arc::clone(&storage);
    return eframe::run_native(
        "My egui App with a plot",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::new(MyApp {
                storage: storage_endpoint,
            }))
        }),
    )
    .expect("app");
}
