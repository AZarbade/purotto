//! A module for defining application data flow

use egui_plot::PlotPoints;
use std::collections::{HashMap, VecDeque};

/// A container for managing multiple data streams and their states.
pub struct DataContainer {
    /// Stores measurements for each stream. The key is the stream name, and the value is a buffer (VecDeque) of measurements.
    pub measurements: HashMap<String, VecDeque<f64>>,
    /// The maximum number of measurements to keep. By default, 500;
    pub look_back: usize,
    /// Tracks total number of data streams being inputed.
    pub stream_count: usize,
    /// Tracks whether each stream should be plotted. The key is the stream name, and the value is a boolean.
    pub plot_tracker: HashMap<String, bool>,
}

// TODO: remove repetition here.
impl Default for DataContainer {
    fn default() -> Self {
        Self {
            measurements: Default::default(),
            look_back: 500,
            stream_count: Default::default(),
            plot_tracker: Default::default(),
        }
    }
}

impl DataContainer {
    /// Appends a new measurement to a specific data stream.
    ///
    /// This method adds a new measurement to the specified stream, maintaining the maximum number
    /// of measurements defined by `look_back`. It also updates the `stream_count`.
    /// NOTE: This method automatically creates a new key for given index.
    /// index: 0 --> key: Stream_0
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the stream to update.
    /// * `value` - The new measurement to append.
    ///
    /// # Example
    ///
    /// ```
    /// let mut container = DataContainer::default();
    /// container.append_values(0, 1.0);
    ///
    /// assert_eq!(container.measurements["Stream_0"], vec![1.0].into());
    /// assert_eq!(container.stream_count, 1);
    /// ```
    pub fn append_values(&mut self, index: usize, value: f64) {
        let stream_index = format!("Stream_{index}");
        let deque = self.measurements.entry(stream_index).or_default();
        deque.push_back(value);
        if deque.len() > self.look_back {
            deque.pop_front();
        }
        self.stream_count = self.measurements.len();
    }

    /// Retrieves [`PlotPoints`] for a specific data stream index.
    ///
    /// This method returns [`PlotPoints`] object containing the measurements for the specified stream.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the stream to retrieve plot points for.
    ///
    /// # Returns
    ///
    /// A [`PlotPoints`] object containing the measurements for the specified stream index.
    ///
    /// # Example
    ///
    /// ```
    /// let mut container = DataContainer::default();
    /// container.append_values(0, 1.0);
    /// container.append_values(0, 2.0);
    /// let plot_points = container.get_plotpoints(0);
    ///
    /// assert_eq!(plot_points.points().len(), 2);
    /// assert_eq!(plot_points.points()[0].y, 1.0);
    /// assert_eq!(plot_points.points()[1].y, 2.0);
    /// ```
    pub fn get_plotpoints(&self, index: usize) -> PlotPoints {
        let stream_index = format!("Stream_{index}");
        let values = self.measurements.get(&stream_index).unwrap();
        let mediate = values.iter().copied();
        PlotPoints::from_ys_f64(&Vec::from_iter(mediate))
    }

    /// Updates the plot tracker to ensure all streams are accounted for.
    ///
    /// This method initializes the `plot_tracker` if it's empty, adding an entry for each stream
    /// with a default value of `false`.
    ///
    /// # Example
    ///
    /// ```
    /// let mut container = DataContainer::default();
    /// // WARN: explicitly setting count, to be removed in future
    /// container.stream_count = 3;
    /// container.update_tracker();
    ///
    /// assert_eq!(container.plot_tracker.len(), 3);
    /// assert_eq!(container.plot_tracker["Stream_0"], false);
    /// assert_eq!(container.plot_tracker["Stream_1"], false);
    /// assert_eq!(container.plot_tracker["Stream_2"], false);
    ///
    /// // Calling update_tracker() again should not change anything
    /// container.update_tracker();
    /// assert_eq!(container.plot_tracker.len(), 3);
    /// ```
    pub fn update_tracker(&mut self) {
        if self.plot_tracker.is_empty() {
            for i in 0..self.stream_count {
                self.plot_tracker.insert(format!("Stream_{i}"), false);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_container_default() {
        let container = DataContainer::default();
        assert_eq!(container.measurements.len(), 0);
        assert_eq!(container.look_back, 500);
        assert_eq!(container.stream_count, 0);
        assert_eq!(container.plot_tracker.len(), 0);
    }

    #[test]
    fn test_append_values() {
        let mut container = DataContainer::default();

        // Test appending to a single stream
        container.append_values(0, 1.0);
        container.append_values(0, 2.0);

        assert_eq!(container.measurements["Stream_0"], vec![1.0, 2.0]);
        assert_eq!(container.stream_count, 1);

        // Test appending to multiple streams
        container.append_values(1, 3.0);
        container.append_values(2, 4.0);

        assert_eq!(container.measurements["Stream_1"], vec![3.0]);
        assert_eq!(container.measurements["Stream_2"], vec![4.0]);
        assert_eq!(container.stream_count, 3);
    }

    #[test]
    fn test_append_values_lookback() {
        let mut container = DataContainer {
            look_back: 2,
            ..Default::default()
        };

        container.append_values(0, 1.0);
        container.append_values(0, 2.0);
        container.append_values(0, 3.0);

        assert_eq!(container.measurements["Stream_0"], vec![2.0, 3.0]);
    }

    #[test]
    fn test_get_plotpoints() {
        let mut container = DataContainer::default();
        container.append_values(0, 1.0);
        container.append_values(0, 2.0);

        let plot_points = container.get_plotpoints(0);

        assert_eq!(plot_points.points().len(), 2);
        assert_eq!(plot_points.points()[0].x, 0.0);
        assert_eq!(plot_points.points()[0].y, 1.0);
        assert_eq!(plot_points.points()[1].x, 1.0);
        assert_eq!(plot_points.points()[1].y, 2.0);
    }

    #[test]
    fn test_update_tracker() {
        let mut container = DataContainer::default();
        container.stream_count = 3;
        container.update_tracker();

        assert_eq!(container.plot_tracker.len(), 3);
        assert_eq!(container.plot_tracker["Stream_0"], false);
        assert_eq!(container.plot_tracker["Stream_1"], false);
        assert_eq!(container.plot_tracker["Stream_2"], false);

        // Test that calling update_tracker() again doesn't change anything
        container.update_tracker();
        assert_eq!(container.plot_tracker.len(), 3);

        // Test updating an existing tracker
        container.plot_tracker.insert("Stream_1".to_string(), true);
        container.update_tracker();
        assert_eq!(container.plot_tracker["Stream_1"], true);
    }

    #[test]
    fn test_multiple_streams() {
        let mut container = DataContainer::default();

        container.append_values(0, 1.0);
        container.append_values(1, 2.0);
        container.append_values(2, 3.0);
        container.append_values(0, 4.0);

        assert_eq!(container.stream_count, 3);
        assert_eq!(container.measurements["Stream_0"], vec![1.0, 4.0]);
        assert_eq!(container.measurements["Stream_1"], vec![2.0]);
        assert_eq!(container.measurements["Stream_2"], vec![3.0]);
    }
}
