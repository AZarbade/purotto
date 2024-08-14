#[derive(Debug)]
pub struct DataStore {
    pub store: Vec<Vec<f32>>,
    store_len: usize,
    store_width: usize,
}

impl DataStore {
    pub fn new() -> Self {
        DataStore {
            store: Vec::new(),
            store_len: 0,
            store_width: 0,
        }
    }

    pub fn get_entry(&self, stream_index: usize, entry_index: usize) -> f32 {
        return self.store[stream_index][entry_index];
    }

    pub fn add_entry(&mut self, data: Vec<f32>) {
        self.store_width = data.len();
        self.store.push(data);
        self.store_len += 1;
    }

    pub fn remove_entry(&mut self) {
        todo!();
    }

    pub fn info(&self) {
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
