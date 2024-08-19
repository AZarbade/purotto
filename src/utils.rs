use bus::{Bus, BusReader};
use std::io::{BufRead, Stdin};
use std::thread;

pub fn stdin_reader(stdin: Stdin) -> BusReader<Vec<f64>> {
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
