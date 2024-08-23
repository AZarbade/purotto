use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

const DEBUG: bool = true;

// [[ 1 2 3 4 ]
//  [ 1 2 3 4 ]
//  [ 1 2 3 4 ]
//  [ 1 2 3 4 ]]
//
// into this...
//
//  [ 1 1 1 1 ]
//  [ 2 2 2 2 ]
//  [ 3 3 3 3 ]
//  [ 4 4 4 4 ]

fn stdin_parser(stdin: io::Stdin) -> (Arc<Mutex<HashMap<usize, Vec<f64>>>>, JoinHandle<()>) {
    let streams: Arc<Mutex<HashMap<usize, Vec<f64>>>> = Arc::new(Mutex::new(HashMap::new()));
    let streams_clone = Arc::clone(&streams);
    let reader_handle = thread::spawn(move || {
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                let values: Vec<f64> = line
                    .split_whitespace()
                    .filter_map(|x| x.parse::<f64>().ok())
                    .collect();

                if !values.is_empty() {
                    for (i, &val) in values.iter().enumerate() {
                        streams
                            .lock()
                            .unwrap()
                            .entry(i)
                            .or_insert_with(Vec::new)
                            .push(val);
                    }

                    if DEBUG {
                        eprintln!("-----------");
                        for (i, stream) in streams.lock().unwrap().iter() {
                            eprintln!("[STDIN_READER]\tStream {}: {:?}", i, stream);
                        }
                    }
                }
            }
        }
    });
    return (streams_clone, reader_handle);
}

#[derive(Default)]
struct MyApp {
    payload: Arc<Mutex<HashMap<usize, Vec<f64>>>>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let mut plot_rect = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PLOTTING");

            let my_plot = Plot::new("my plot");
            let data = self.payload.lock().unwrap();
            let stream = data.get(&0).unwrap();
            let inner = my_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from_ys_f64(&stream[..])));
            });

            plot_rect = Some(inner.response.rect);
            ctx.request_repaint();
        });
    }
}

fn main() {
    let mut thread_handles: Vec<JoinHandle<()>> = Vec::new();
    let stdin = io::stdin();
    let (data, read_handle) = stdin_parser(stdin);
    thread_handles.push(read_handle);

    if DEBUG {
        std::thread::sleep_ms(2000);
        println!("[MAIN]\t{:?}", data.lock().unwrap().get(&0).unwrap());
    }

    let applet = Box::<MyApp>::new(MyApp {
        payload: data,
        ..Default::default()
    });

    eframe::run_native(
        "my app",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(applet)),
    )
    .unwrap();

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
