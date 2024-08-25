use egui_plot::PlotPoints;
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
pub struct DataContainer {
    pub measurements: HashMap<usize, VecDeque<f64>>,
    pub stream_count: usize,
}

impl DataContainer {
    pub fn append_values(&mut self, index: usize, value: f64) {
        self.measurements.entry(index).or_default().push_back(value);
        self.stream_count = self.measurements.len();
    }

    pub fn get_plotpoints(&self, index: usize) -> PlotPoints {
        let values = self.measurements.get(&index).unwrap();
        let mediate = values.iter().copied();
        PlotPoints::from_ys_f64(&Vec::from_iter(mediate))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_values() {
        let mut container = DataContainer::default();

        container.append_values(0, 1.0);
        container.append_values(0, 2.0);
        container.append_values(1, 3.0);

        assert_eq!(container.stream_count, 2);
        assert_eq!(container.measurements.get(&0).unwrap().len(), 2);
        assert_eq!(container.measurements.get(&1).unwrap().len(), 1);
    }

    #[test]
    fn test_append_values_order() {
        let mut container = DataContainer::default();

        container.append_values(0, 1.0);
        container.append_values(0, 2.0);
        container.append_values(0, 3.0);

        let values = container.measurements.get(&0).unwrap();
        assert_eq!(
            values.iter().copied().collect::<Vec<f64>>(),
            vec![1.0, 2.0, 3.0]
        );
    }

    #[test]
    fn test_multiple_streams() {
        let mut container = DataContainer::default();

        container.append_values(0, 1.0);
        container.append_values(1, 2.0);
        container.append_values(2, 3.0);

        assert_eq!(container.stream_count, 3);
        assert_eq!(container.measurements.get(&0).unwrap().len(), 1);
        assert_eq!(container.measurements.get(&1).unwrap().len(), 1);
        assert_eq!(container.measurements.get(&2).unwrap().len(), 1);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_get_plotpoints_nonexistent_index() {
        let container = DataContainer::default();
        container.get_plotpoints(0);
    }

    #[test]
    fn test_get_plotpoints() {
        todo!();
        //let mut container = DataContainer::default();
        //
        //container.append_values(0, 1.0);
        //container.append_values(0, 2.0);
        //container.append_values(0, 3.0);
        //
        //let plot_points = container.get_plotpoints(0);
        //assert_eq!(plot_points.count(), 3);
        //assert_eq!(
        //    plot_points.points().collect::<Vec<_>>(),
        //    vec![[0.0, 1.0].into(), [1.0, 2.0].into(), [2.0, 3.0].into(),]
        //);
    }
}
