mod datacontainer;
mod reader;

use crate::datacontainer::DataContainer;

fn main() {
    let mut thread_handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
    let stdin = std::io::stdin();
    let (_data, read_handle) = reader::stdin_parser(stdin);
    thread_handles.push(read_handle);

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
