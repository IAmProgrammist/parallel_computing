float work_group_reduce_add(__local float* local_data) {
    int group_size = get_local_size(0);
    int local_id = get_local_id(0);
    float result = 0.0;
    
    barrier(CLK_LOCAL_MEM_FENCE);
    
    for (int stride = group_size >> 1; stride > 0; stride >>= 1) {
        if (local_id < stride)
            local_data[local_id] += local_data[local_id + stride];
        barrier(CLK_LOCAL_MEM_FENCE);
    }
    
    return local_data[0];
}

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
    float bias = 0.0;
    if (local_id < OBJECTS_AMOUNT) {
        float y_pred = input_predictions[local_id];
        error = input_labels[local_id] - y_pred;

        grad = error * y_pred * 
               (1 - y_pred) *
               input_x[local_id * NODES_AMOUNT + group_id];
        
        if (group_id == 0) {
            bias = error * y_pred * (1 - y_pred);
        }
    }

    local_data[local_id] = grad;

    float grad_sum = work_group_reduce_add(local_data);
    if (local_id == 0) {
        output_weights[group_id] += learning_rate * grad_sum;
    }

    // Скорректировать bias
    local_data[local_id] = bias;
    float bias_sum = work_group_reduce_add(local_data);

    if (group_id == 0 && local_id == 0) {
        output_bias[0] += bias_sum;
    }
}
