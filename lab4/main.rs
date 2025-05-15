use opencl3::command_queue::{CL_QUEUE_PROFILING_ENABLE, CommandQueue};
use opencl3::context::Context;
use opencl3::device::{CL_DEVICE_TYPE_GPU, Device, get_all_devices};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
use opencl3::program::Program;
use opencl3::types::{CL_BLOCKING, CL_NON_BLOCKING, cl_event, cl_float};
use std::ptr;

const PREDICTION_COMP_SHADER_PATH: &str = "./lab4/shaders/prediction.cl";
const PREDICTION_COMP_SHADER_NAME: &str = "prediction";
const RECALC_WEIGHTS_COMP_SHADER_PATH: &str = "./lab4/shaders/recalc_weights.cl";
const RECALC_WEIGHTS_COMP_SHADER_NAME: &str = "recalc_weights";

pub struct NeuralNetwork {
    _weights: Vec<f32>,
    _bias: f32,
    _queue: CommandQueue,
    _context: Context,
    _program_prediction: Program,
    _kernel_prediction: Kernel,
    _program_recalc_weights: Program,
    _kernel_recalc_weights: Kernel,
}

impl NeuralNetwork {
    pub fn new() -> Result<NeuralNetwork, String> {
        // Найти устройство
        let device_id = *get_all_devices(CL_DEVICE_TYPE_GPU)?
            .first()
            .expect("no device found in platform");
        let device = Device::new(device_id);

        // Создать контекст
        let context = Context::from_device(&device)?;

        // Создать очередь команд
        let queue = CommandQueue::create_default(&context, CL_QUEUE_PROFILING_ENABLE)?;

        // Загрузить вычислительные шейдеры
        let program_prediction = Program::create_and_build_from_source(&context, 
            std::fs::read_to_string(PREDICTION_COMP_SHADER_PATH).expect("can't open file for prediction shader").as_str(), 
            "")?;
        let kernel_prediction = Kernel::create(&program_prediction, PREDICTION_COMP_SHADER_NAME)?;
        let program_recalc_weights = Program::create_and_build_from_source(&context,
            std::fs::read_to_string(RECALC_WEIGHTS_COMP_SHADER_PATH).expect("can't open file for recalc weights shader").as_str(),
            "")?;
        let kernel_recalc_weights = Kernel::create(&program_recalc_weights, RECALC_WEIGHTS_COMP_SHADER_NAME)?;

        let result = NeuralNetwork {
            _bias: 0.0,
            _weights: vec![],
            _queue: queue,
            _context: context,
            _program_prediction: program_prediction,
            _kernel_prediction: kernel_prediction,
            _program_recalc_weights: program_recalc_weights,
            _kernel_recalc_weights: kernel_recalc_weights
        };

        Ok(result)
    }

    pub fn fit(&mut self, train_data: Vec<Vec<f32>>, labels: Vec<u8>, epochs: usize, learning_rate: f32) -> Result<&NeuralNetwork, String> {
        if train_data.len() == 0 {
            return Err(format!("no data to train provided"));
        }
        if train_data.len() != labels.len() {
            return Err(format!("train and label amount are different"));
        }

        self._weights = vec![0.0; train_data[0].len()];

        let mut train_data_aligned = vec![0f32; train_data[0].len() * train_data.len()];
        for i in 0..train_data.len() {
            for j in 0..train_data[0].len() {
                train_data_aligned[j + i * train_data.len()] = train_data[i][j];
            }
        }
        let nodes_amount = train_data[0].len();

        for _epoch in 0..epochs {
            let mut predictions = vec![0f32; labels.len()];

            // Получим предсказания
            let mut p_oclbuf_input_x = unsafe {
                Buffer::<cl_float>::create(&self._context, CL_MEM_READ_ONLY, train_data[0].len() * train_data.len(), ptr::null_mut())?
            };
            let mut p_oclbuf_input_weights = unsafe {
                Buffer::<cl_float>::create(&self._context, CL_MEM_READ_ONLY, self._weights.len(), ptr::null_mut())?
            };
            let mut p_oclbuf_predictions = unsafe {
                Buffer::<cl_float>::create(&self._context, CL_MEM_WRITE_ONLY, labels.len(), ptr::null_mut())?
            };

            let _writing_event = unsafe { self._queue.enqueue_write_buffer(&mut p_oclbuf_input_x, 
                CL_BLOCKING, 0, &train_data_aligned, &[])? };

            let _writing_event = unsafe { self._queue.enqueue_write_buffer(&mut p_oclbuf_input_weights, 
               CL_BLOCKING, 0, &self._weights, &[])? };

            let _writing_event = unsafe { self._queue.enqueue_write_buffer(&mut p_oclbuf_predictions, 
                CL_BLOCKING, 0, &predictions, &[])? };

            let kernel_event = unsafe {
                ExecuteKernel::new(&self._kernel_prediction)
                    .set_arg(&p_oclbuf_input_x)
                    .set_arg(&p_oclbuf_input_weights)
                    .set_arg(&self._bias)
                    .set_arg(&nodes_amount)
                    .set_arg(&p_oclbuf_predictions)
                    .set_arg_local_buffer(nodes_amount.next_power_of_two() * size_of::<f32>())
                    .set_local_work_size(nodes_amount.next_power_of_two())
                    .set_global_work_size(nodes_amount.next_power_of_two() * train_data.len())
                    .enqueue_nd_range(&self._queue)?
            };

            let mut events: Vec<cl_event> = Vec::default();
            events.push(kernel_event.get());

            // Create a results array to hold the results from the OpenCL device
            // and enqueue a read command to read the device buffer into the array
            // after the kernel event completes.
            let mut results = vec![0.0 as cl_float; labels.len()];
            let read_event =
                unsafe { self._queue.enqueue_read_buffer(&p_oclbuf_predictions, CL_BLOCKING, 0, &mut results, &events)? };

            // Wait for the read_event to complete.
            read_event.wait()?;

            println!("{:?}", results);

        }

        // Создать буферы
        /*
        let mut input_labels = unsafe {
            Buffer::<cl_float>::create(&self._context, CL_MEM_READ_ONLY, labels.len(), ptr::null_mut())?
        };
        let mut input_predictions = unsafe {
            Buffer::<cl_float>::create(&self._context, CL_MEM_READ_ONLY, ARRAY_SIZE, ptr::null_mut())?
        };
        let input_x = unsafe {
            Buffer::<cl_float>::create(&self._context, CL_MEM_WRITE_ONLY, ARRAY_SIZE, ptr::null_mut())?
        };
        let output_weights = unsafe {
            Buffer::<cl_float>::create(&self._context, CL_MEM_WRITE_ONLY, ARRAY_SIZE, ptr::null_mut())?
        };
        let output_bias = unsafe {
            Buffer::<cl_float>::create(&self._context, CL_MEM_WRITE_ONLY, ARRAY_SIZE, ptr::null_mut())?
        };
        */

        Ok(self)
    }

    pub fn predict(&self, data: &[f32]) -> Result<f32, String> {
        if self._weights.len() != data.len() {
            return Err(format!("data and train set props lengths ({} and {}) are different", data.len(), self._weights.len()));
        }

        Ok(1.0)
    }
}

fn main() {
    let mut nn = NeuralNetwork::new().expect("oops");
    nn.fit(vec![
        vec![0.1, 0.2, 0.3, 0.4, 0.5],
        vec![0.2, 0.1, 0.4, 0.3, 0.5],
        vec![0.6, 0.7, 0.8, 0.9, 1.0],
        vec![0.9, 0.8, 1.0, 0.7, 0.6]
    ], 
    vec![0, 0, 1, 1], 
    1, 
0.1).expect("oops");
}
