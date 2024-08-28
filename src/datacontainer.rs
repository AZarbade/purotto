use egui_plot::PlotPoints;
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
pub struct DataContainer {
    pub measurements: HashMap<String, VecDeque<f64>>,
    pub stream_count: usize,
    pub look_back: usize,
    pub plot_tracker: HashMap<String, bool>,
}

impl DataContainer {
    pub fn append_values(&mut self, index: usize, value: f64) {
        let stream_index = format!("Stream_{index}");
        let deque = self.measurements.entry(stream_index).or_default();
        deque.push_back(value);
        if deque.len() > self.look_back {
            deque.pop_front();
        }
        self.stream_count = self.measurements.len();
    }

    pub fn get_plotpoints(&self, index: usize) -> PlotPoints {
        let stream_index = format!("Stream_{index}");
        let values = self.measurements.get(&stream_index).unwrap();
        let mediate = values.iter().copied();
        PlotPoints::from_ys_f64(&Vec::from_iter(mediate))
    }

    pub fn update_tracker(&mut self) {
        if self.plot_tracker.is_empty() {
            for i in 0..self.stream_count {
                self.plot_tracker.insert(format!("Stream_{i}"), false);
            }
        }
    }
}
