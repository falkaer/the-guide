struct Uniform {
    signal_length: u32,
    kernel_size: u32,
    not_used: u32,
    not_used: u32,
};

@group(0) @binding(0)
var<uniform> dimensions: Uniform;

@group(0) @binding(1)
var<storage, read> signal: array<f32>;

@group(0) @binding(2)
var<storage, read> kernel: array<f32>;

@group(0) @binding(3)
var<storage, write> output: array<f32>;

@compute @workgroup_size(32, 1, 1) 
fn convolution_naive(
    @builtin(global_invocation_id) global_id: vec3<u32>,
) {
    var result: f32 = 0.0;
    for (var k = 0; k < i32(dimensions.kernel_size); k++) {
        let ind = i32(global_id.x) - i32(dimensions.kernel_size) / 2 + k;
        if ind >= 0 && ind < i32(dimensions.signal_length) {
            result += kernel[k] * signal[ind];
        }
    }
    // we can at least coalesce writes
    if global_id.x < dimensions.signal_length {
        output[global_id.x] = result;
    }
}
