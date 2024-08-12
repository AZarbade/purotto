#![allow(dead_code)]
use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
struct DataStore {
    store: Vec<Vec<f32>>,
    store_len: usize,
    store_width: usize,
}

impl DataStore {
    fn new() -> Self {
        DataStore {
            store: Vec::new(),
            store_len: 0,
            store_width: 0,
        }
    }

    fn get(&self, stream_index: usize) -> Vec<f32> {
        return self.store[stream_index].clone();
    }

    fn add_entry(&mut self, data: Vec<f32>) {
        self.store_width = data.len();
        self.store.push(data);
        self.store_len += 1;
    }

    fn remove_entry(&mut self) {
        todo!();
    }

    fn info(&self) {
        // print: len, width, head/tail
        todo!();
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                dbg!(tx.send(line).unwrap());
            }
        }
    });

    let mut dataset = DataStore::new();

    for received in rx {
        let data: Vec<f32> = received
            .split_whitespace()
            .filter_map(|x| x.parse::<f32>().ok())
            .collect();

        if !data.is_empty() {
            dataset.add_entry(data);
        }
        dbg!(&dataset.get(0));
    }
}

// NOTE:
// [src/main.rs:60:9] &dataset = DataStore {
//     store: [
//         [
//             248.0,
//             175.0,
//             160.0,
//             180.0,
//         ],
//         [
//             279.0,
//             46.0,
//             172.0,
//             81.0,
//         ],
//     ],
//     store_len: 2,
//     store_width: 4,
// }
