//! Collection of parsers to be used for data input

use crate::DataContainer;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

/// Parses standard input and returns custom [`DataContainer`] with a thread handle.
///
/// This method creates a new [`DataContainer`] wrapped in an [`Arc<Mutex<>>`] for thread-safe
/// access. It spawns a new thread that continuously reads from stdin, parses each line
/// into floating-point values, and appends these values to the appropriate streams in
/// the [`DataContainer`].
///
/// # Arguments
///
/// * [`io::Stdin`] - The standard input to read from.
///
/// # Returns
///
/// A tuple containing:
/// - An `Arc<Mutex<DataContainer>>` that can be used to access the parsed data.
/// - A `JoinHandle<()>` for the spawned thread, which can be used to wait for the parsing to complete.
///
/// # Example
///
/// ```
/// let stdin = std::io::stdin();
/// let (data, handle) = stdin_parser(stdin);
///
/// handle.join().unwrap(); // Remember to close thread handles
/// ```
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
