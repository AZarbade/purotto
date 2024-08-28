use crate::DataContainer;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub fn stdin_parser(stdin: io::Stdin) -> (Arc<Mutex<DataContainer>>, JoinHandle<()>) {
    let streams = Arc::new(Mutex::new(DataContainer {
        look_back: 250,
        ..Default::default()
    }));
    let streams_clone = Arc::clone(&streams);
    let reader_handle = thread::spawn(move || {
        for line in stdin.lock().lines() {
            let values: Vec<f64> = line
                .unwrap()
                .split_whitespace()
                .filter_map(|x| x.parse::<f64>().ok())
                .collect();

            if !values.is_empty() {
                streams.lock().unwrap().stream_count = values.len();
                for (i, &val) in values.iter().enumerate() {
                    streams.lock().unwrap().append_values(i, val);
                }
            }
        }
    });
    (streams_clone, reader_handle)
}
