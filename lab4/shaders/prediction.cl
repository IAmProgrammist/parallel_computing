float sigmoid(float x) {
    return 1. / (1. + exp(-x));
}

__kernel void prediction(__global const float* input_x,
                         __global const float* input_weights,
                         const float bias,
                         unsigned int NODES_AMOUNT,
                         __global float* predictions,
                         __local float* local_data) {
    int group_id = get_group_id(0);
    int local_id = get_local_id(0);
    
    int group_size = get_local_size(0);

    float val = 0.0f;
    if (local_id < NODES_AMOUNT) 
        val = input_x[group_id * NODES_AMOUNT + local_id] * input_weights[local_id];
    
    local_data[local_id] = val;

    barrier(CLK_LOCAL_MEM_FENCE);
    
    for (int stride = group_size >> 1; stride > 0; stride >>= 1) {
        if (local_id < stride)
            local_data[local_id] += local_data[local_id + stride];
        barrier(CLK_LOCAL_MEM_FENCE);
    }

    if (!local_id) 
        predictions[group_id] = sigmoid(local_data[0] + bias);
}