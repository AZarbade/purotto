#![allow(dead_code, unused)]
use bus::{Bus, BusReader};
use eframe::egui;
use egui_plot::{Legend, Line, LineStyle, Plot, PlotPoints};
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;

fn read_stdin() -> BusReader<Vec<f64>> {
    let mut bus = Bus::new(100);
    let mut rx_main = bus.add_rx();

    let stdin = io::stdin();
    let broadcast_handle = thread::spawn(move || {
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

    return rx_main;
}

#[derive(Default)]
struct MainApp {
    data: Vec<f64>,
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut plot_rect = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            let my_plot = Plot::new("My Plot").legend(Legend::default());

            let line = Line::new(PlotPoints::from_ys_f64(&self.data));
            let inner = my_plot.show(ui, |plot_ui| {
                plot_ui.line(line);
            });

            plot_rect = Some(inner.response.rect);
            ctx.request_repaint();
        });
    }
}

fn main() {
    let stream = Arc::new(Mutex::new(Vec::<f64>::new()));
    let stream_clone = Arc::clone(&stream);
    thread::spawn(move || {
        let mut rx = read_stdin();
        loop {
            let val = rx.recv().unwrap()[0];
            stream.lock().unwrap().push(val);
            dbg!(&stream);
        }
    });

    let applet = Box::<MainApp>::new(MainApp {
        data: stream_clone.lock().unwrap().clone(),
        ..Default::default()
    });

    eframe::run_native(
        "my app",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(applet)),
    )
    .unwrap();
}
