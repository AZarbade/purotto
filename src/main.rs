#![allow(dead_code)]
use std::io::{self, BufRead};

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
        println!("DataStore Info:");
        println!("  Length: {}", self.store_len);
        println!("  Width: {}", self.store_width);
        if let Some(head) = self.store.first() {
            println!("  Head: {:?}", head);
        }
        if let Some(tail) = self.store.last() {
            println!("  Tail: {:?}", tail);
        }
    }
}

use std::sync::mpsc::{channel, Receiver};
use std::thread;
fn read_them_values() -> Receiver<Vec<f32>> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let stdin = io::stdin();
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
    let mut dataset = DataStore::new();
    let rx = read_them_values().recv().unwrap();
    dataset.add_entry(rx);
    dbg!(&dataset);
}
