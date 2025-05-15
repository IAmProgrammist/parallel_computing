__kernel void recalc_weights(__global const float* input_labels,
                             __global const float* input_predictions,
                             __global const float* input_x,
                             __global float* output_weights,
                             __global float* output_bias,
                             unsigned int OBJECTS_AMOUNT,
                             float learning_rate,
                             __local float* local_data) {
    int global_id = get_global_id(0);
    int group_id = get_group_id(0);
    int local_id = get_local_id(0);

    int NODES_AMOUNT = get_global_size(0) / get_local_size(0);

    // group_id - номер признака
    // local_id - номер тестируемого объекта

    float error = 0.0;
    float grad = 0.0;
    if (local_id < OBJECTS_AMOUNT) {
        float y_pred = input_predictions[local_id];
        error = input_labels[local_id] - y_pred;

        grad = error * y_pred * 
               (1 - y_pred) *
               input_x[local_id * NODES_AMOUNT + group_id];
    }

    local_data[local_id] = grad;

    float grad_sum = sub_group_reduce_add(local_data[local_id]);
    if (local_id == 0) {
        output_weights[group_id] += learning_rate * grad_sum;
    }
}
