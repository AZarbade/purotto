#[derive(Debug)]
pub struct DataStore {
    store: Vec<Vec<f32>>,
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

    pub fn get(&self, stream_index: usize) -> Vec<f32> {
        return self.store[stream_index].clone();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ds = DataStore::new();
        assert_eq!(ds.store_len, 0);
        assert_eq!(ds.store_width, 0);
        assert!(ds.store.is_empty());
    }

    #[test]
    fn test_add_entry() {
        let mut ds = DataStore::new();
        ds.add_entry(vec![1.0, 2.0, 3.0]);
        assert_eq!(ds.store_len, 1);
        assert_eq!(ds.store_width, 3);
        assert_eq!(ds.store, vec![vec![1.0, 2.0, 3.0]]);

        ds.add_entry(vec![4.0, 5.0, 6.0]);
        assert_eq!(ds.store_len, 2);
        assert_eq!(ds.store_width, 3);
        assert_eq!(ds.store, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    }

    #[test]
    fn test_get() {
        let mut ds = DataStore::new();
        ds.add_entry(vec![1.0, 2.0, 3.0]);
        ds.add_entry(vec![4.0, 5.0, 6.0]);

        assert_eq!(ds.get(0), vec![1.0, 2.0, 3.0]);
        assert_eq!(ds.get(1), vec![4.0, 5.0, 6.0]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_get_out_of_bounds() {
        let ds = DataStore::new();
        ds.get(0);
    }
}
