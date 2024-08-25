use crate::DataContainer;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub fn stdin_parser(stdin: io::Stdin) -> (Arc<Mutex<DataContainer>>, JoinHandle<()>) {
    let streams = Arc::new(Mutex::new(DataContainer {
        ..Default::default()
    }));
    let streams_clone = Arc::clone(&streams);
    let mut look_back = 0;
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
                    // ignore initial (5) values
                    if look_back >= 5 {
                        streams.lock().unwrap().append_values(i, val);
                    }
                    look_back += 1;
                }
            }
        }
    });
    (streams_clone, reader_handle)
}
