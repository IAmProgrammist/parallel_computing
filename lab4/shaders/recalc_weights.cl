__kernel void recalc_weights(__global const float* input_labels,
                             __global const float* input_predictions,
                             __global const float* input_x,
                             __global float* output_weights,
                             __global float* output_bias,
                             unsigned int NODES_AMOUNT,
                             float learning_rate,
                             __local float* local_data) {
    int global_id = get_global_id(0);
    int group_id = get_group_id(0);
    int local_id = get_local_id(0);

    int group_size = get_local_size(0);
    float y_pred = input_predictions[group_id];

    float error = input_labels[group_id] - y_pred;

    float val = 0.0f;
    if (local_id < NODES_AMOUNT) val = error * y_pred * 
                                       (1 - y_pred) * 
                                        input_x[group_id * NODES_AMOUNT + local_id];
    local_data[local_id] = val;
    
    barrier(CLK_LOCAL_MEM_FENCE);
    
    
    for (int stride = group_size >> 1; stride > 0; stride >>= 1) {
        if (local_id < stride)
            local_data[local_id] += local_data[local_id + stride];
        barrier(CLK_LOCAL_MEM_FENCE);
    }

    if (!local_id) {
        output_weights[group_id] += learning_rate * local_data[0];
        output_bias[group_id] += learning_rate * 
                                 error * 
                                 y_pred * 
                                 (1 - y_pred);     
    }
}