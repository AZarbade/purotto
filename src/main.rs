use std::io::{self, BufRead, Stdin};
use std::sync::{
    mpsc::{channel, Receiver},
    Arc, Mutex,
};
use std::thread;

mod dataset;

fn stdin_reader(stdin: Stdin) -> Receiver<Vec<f32>> {
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

fn stdin_processer(storage: Arc<Mutex<dataset::DataStore>>, rx: Receiver<Vec<f32>>) {
    loop {
        if let Ok(rx) = rx.recv() {
            storage
                .lock()
                .expect("ERROR: failed to acquire lock on storage")
                .add_entry(rx);
        }
    }
}

fn main() {
    let storage = Arc::new(Mutex::new(dataset::DataStore::new()));
    let storage_socket = Arc::clone(&storage);
    thread::spawn(move || {
        let stdin = stdin_reader(io::stdin());
        stdin_processer(storage_socket, stdin);
    });

    // FIX: use different method for this
    // - sleeping to let data flow in storage
    use std::time::Duration;
    thread::sleep(Duration::from_secs(1));

    let storage_endpoint = Arc::clone(&storage);
    dbg!(storage_endpoint.lock().unwrap().store[0][0]);
}
