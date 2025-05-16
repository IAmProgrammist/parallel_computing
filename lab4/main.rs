mod neurals;

use crate::neurals::neural::NeuralNetwork;

const TRAIN_DATA_PATH: &str = "./lab4/datasets/train_data";
const LABELS_PATH: &str = "./lab4/datasets/labels";
const EPOCHS: usize = 1000;
const LEARNING_RATE: f32 = 0.1;

fn load_data() -> Result<(Vec<Vec<f32>>, Vec<f32>), String> {
    // Загрузить объекты
    let mut result_train_data: Vec<Vec<f32>> = vec![];
    for train_data in std::fs::read_to_string(TRAIN_DATA_PATH)
        .expect("train data file is not accessible")
        .lines()
    {
        let mut train_object: Vec<f32> = vec![];
        for prop in train_data.split(" ") {
            train_object.push(prop.parse::<f32>().expect("invalid float variable in train data file"));
        }

        if result_train_data.len() > 0 {
            if result_train_data[0].len() != train_object.len() {
                return Err(format!("objects have different props amount"));
            }
        }

        result_train_data.push(train_object);
    }

    // Загрузить метки
    let mut result_labels: Vec<f32> = vec![];
    for labels_data in std::fs::read_to_string(LABELS_PATH)
        .expect("label lines file is not accessible")
        .lines()
    {
        result_labels.push(labels_data.parse::<f32>().expect("invalid float variable in labels file"));
    }

    if result_train_data.len() != result_labels.len() {
        return Err(format!("train data and labels amounts are different"));
    }

    Ok((result_train_data, result_labels))
}

fn main() {
    let (train_data, train_labels) = load_data().expect("invalid dataset");

    // Тренировка при помощи GPU
    let mut nn = neurals::opencl::OpenCLNeuralNetwork::new().expect("failed GPU Accelerated train");
    nn.fit(&train_data, &train_labels, EPOCHS, LEARNING_RATE)
        .expect("failed GPU Accelerated train");

    // Тренировка при помощи CPU
    let mut nn = neurals::cpu::CPUNeuralNetwork::new().expect("failed CPU train");
    nn.fit(&train_data, &train_labels, EPOCHS, LEARNING_RATE)
        .expect("failed CPU train");
}
