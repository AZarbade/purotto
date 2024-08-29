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
/// // Simulate stdin input
/// thread::spawn(|| {
///     println!("1.0 2.0 3.0");
///     println!("4.0 5.0 6.0");
///     thread::sleep(Duration::from_millis(100));
///     println!("7.0 8.0 9.0");
/// });
///
/// let (data, handle) = stdin_parser(io::stdin());
///
/// // Wait a bit for the data to be processed
/// thread::sleep(Duration::from_millis(200));
///
/// let container = data.lock().unwrap();
/// assert_eq!(container.stream_count, 3);
/// assert_eq!(container.measurements["Stream_0"].len(), 3);
/// assert_eq!(container.measurements["Stream_1"].len(), 3);
/// assert_eq!(container.measurements["Stream_2"].len(), 3);
///
/// assert_eq!(*container.measurements["Stream_0"].back().unwrap(), 7.0);
/// assert_eq!(*container.measurements["Stream_1"].back().unwrap(), 8.0);
/// assert_eq!(*container.measurements["Stream_2"].back().unwrap(), 9.0);
///
/// // Clean up the thread
/// drop(container);
/// handle.join().unwrap();
/// ```
///
/// Note: This example simulates stdin input and may not work in all environments.
/// It's for illustration purposes only.
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
