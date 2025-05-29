use crate::neurals::neural::NeuralNetwork;
use rand::Rng;
use std::time::Instant;

pub struct CPUNeuralNetwork {
    _weights: Vec<f32>,
    _bias: [f32; 1],
}

impl CPUNeuralNetwork {
    /// Конструирует нейронную сеть с поддержкой процессора.
    pub fn new() -> Result<CPUNeuralNetwork, String> {
        let mut rng = rand::rng();

        let result = CPUNeuralNetwork {
            _bias: [rng.random_range(-1.0..1.0)],
            _weights: vec![],
        };

        Ok(result)
    }

    fn _sigmoid(x: f32) -> f32 {
        1. / (1. + (-x).exp())
    }
}

impl NeuralNetwork for CPUNeuralNetwork {
    fn fit(
        &mut self,
        train_data: &Vec<Vec<f32>>,
        labels: &Vec<f32>,
        epochs: usize,
        learning_rate: f32,
    ) -> Result<&impl NeuralNetwork, String> {
        if train_data.len() == 0 {
            return Err(format!("no data to train provided"));
        }
        if train_data.len() != labels.len() {
            return Err(format!("train and label lengths are different"));
        }

        // Обновить веса
        let mut rng = rand::rng();

        self._weights = vec![0.0; train_data[0].len()];
        for weight_index in 0..self._weights.len() {
            self._weights[weight_index] = rng.random_range(-1.0..1.0);
        }

        let now = Instant::now();
        println!("Starting CPU only training\n\
                  =================================");

        for epoch in 0..epochs {
            let mut weights_new = self._weights.clone();
            let mut bias_new = self._bias[0];

            for dataset_index in 0..train_data.len() {
                let y_pred = self.predict(&vec![train_data[dataset_index].clone()])?[0];
                let error = labels[dataset_index] - y_pred;

                for prop_index in 0..train_data[dataset_index].len() {
                    weights_new[prop_index] += learning_rate * (
                        error * y_pred * (1f32 - y_pred) * train_data[dataset_index][prop_index]
                    );
                }
                bias_new += learning_rate * error * y_pred * (1f32 - y_pred);
            }

            self._weights = weights_new;
            self._bias[0] = bias_new;

            if epoch % 100 == 0 {
                println!("Current epoch: {}\n\
                Current weights: {:?}\n\
                Current bias: {}", epoch, self._weights, self._bias[0]);
            }
        }

        println!("=================================\n\
        GPU Accelerated train completed!\n\
          Total epochs: {}\n\
          Result weights: {:?}\n\
          Result bias: {:?}\n\
          Elapsed time: {:.2?}
        ",
        epochs, self._weights, self._bias[0], now.elapsed());

        Ok(self)
    }

    fn predict(&self, data: &Vec<Vec<f32>>) -> Result<Vec<f32>, String> {
        if data.len() == 0 {
            return Ok([].to_vec());
        }

        if self._weights.len() != data[0].len() {
            return Err(format!(
                "data and train set props lengths ({} and {}) are different",
                data.len(),
                self._weights.len()
            ));
        }

        let mut results = vec![0f32; data.len()];
        for dataset_index in 0..data.len() {
            for prop_index in 0..data[dataset_index].len() {
                results[dataset_index] += data[dataset_index][prop_index] * self._weights[prop_index];    
            }

            results[dataset_index] = CPUNeuralNetwork::_sigmoid(results[dataset_index] + self._bias[0]);
        }

        Ok(results)
    }
}