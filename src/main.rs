use std::io::{self, BufRead, Stdin};
use std::sync::mpsc::{channel, Receiver};
use std::thread;

mod dataset;

fn read_them_values(stdin: Stdin) -> Receiver<Vec<f32>> {
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

fn main() {
    let mut dataset = dataset::DataStore::new();
    let rx = read_them_values(io::stdin());
    loop {
        if let Ok(rx) = rx.recv() {
            dataset.add_entry(rx);
            dbg!(&dataset);
        }
    }
}
