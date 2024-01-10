use crate::{
    gpu_vector::GPUVector,
    utility::{are_vectors_equivalent, mean_square_error, run_compute_shader, GPUHandles, Uniform},
};

// The length of filter is assumed to be oddly number, i.e. 1, 3, 5, 7, 9, 11
fn convolution_cpu(signal: &Vec<f32>, filter: &Vec<f32>) -> Vec<f32> {
    let filter_offset = filter.len() / 2;
    let mut output: Vec<f32> = vec![0.0; signal.len()];
    for signal_index in 0..signal.len() {
        for filter_index in 0..filter.len() {
            let offset_signal_index: i64 =
                signal_index as i64 - filter_offset as i64 + filter_index as i64;
            if -1 < offset_signal_index && offset_signal_index < signal.len() as i64 {
                output[signal_index] += signal[offset_signal_index as usize] * filter[filter_index];
            }
        }
    }

    output
}

fn test_ground_truth() -> bool {
    let signal: Vec<f32> = vec![1.0, 1.0, 1.0, 1.0, 1.0];
    let filter: Vec<f32> = vec![0.25, 0.5, -0.25];
    let ground_truth_output: Vec<f32> = vec![0.25, 0.5, 0.5, 0.5, 0.75];

    let output: Vec<f32> = convolution_cpu(&signal, &filter);

    assert!(output.len() == ground_truth_output.len());
    for index in 0..ground_truth_output.len() {
        if 0.00001 < (output[index] - ground_truth_output[index]).abs() {
            println!("Provided output: {:?}", output);
            println!("Ground truth output: {:?}", ground_truth_output);
            return false;
        }
    }

    true
}

fn run_naive_convolution(handles: &GPUHandles, signal: Vec<f32>, kernel: Vec<f32>) -> Vec<f32> {
    let signal_size = signal.len();
    let kernel_size = kernel.len();
    let output: Vec<f32> = vec![0.0; signal_size];

    let gpu_signal = GPUVector::new(&handles, signal, "signal", false);
    let gpu_kernel = GPUVector::new(&handles, kernel, "kernel", false);
    let mut gpu_output = GPUVector::new(&handles, output, "output", true);

    let uniform: Uniform = Uniform::new(&handles, signal_size, kernel_size, 0, 0);
    let shader_file: &'static str = include_str!("convolution_naive.wgsl");
    let shader_function: &str = "convolution_naive";

    let block_size_x: usize = 32;
    let launch_blocks_x: u32 = ((signal_size + block_size_x - 1) / block_size_x) as u32;
    let block_size_y: usize = 1;
    let launch_blocks_y: u32 = 1;

    run_compute_shader(
        handles,
        block_size_x,
        launch_blocks_x,
        block_size_y,
        launch_blocks_y,
        shader_file,
        shader_function,
        &uniform,
        &gpu_signal,
        &gpu_kernel,
        &mut gpu_output,
    );

    return gpu_output.cpu_data;
}

pub fn convolution(handles: &GPUHandles) -> bool {
    // A small test to ensure that the convolution_cpu function is actually correct.
    let ground_truth_is_correct: bool = test_ground_truth();
    println!(
        "Convolution ground truth function is correct: {}",
        ground_truth_is_correct
    );
    assert!(ground_truth_is_correct);

    let data_element_count: usize = 1000000;
    let filter_size: usize = 19;
    // NB: changed signal to be a sine so small discrepancies wont blow up
    let signal: Vec<f32> = (0..data_element_count)
        .map(|x| (x as f32 * 0.01).sin())
        .collect();
    // let signal: Vec<f32> = (0..data_element_count).map(|x| x as f32 * 1.0).collect();
    let filter: Vec<f32> = (0..filter_size).map(|x| x as f32 * -0.1).collect();

    let ground_truth: Vec<f32> = convolution_cpu(&signal, &filter);

    //
    // 0) Do 1D convolution on the GPU, don't use shared memory.
    // Make sure to keep the filter and signal large enough to offset the cost of data transfer
    //
    // 1) Make another version using a zero padded version of the original signal. Do not use any if's
    // inside the inner for-loop. This zero padding is (filter_size - 1) / 2 on each side of the
    // signal. What happens if you increase the padding with 0's to ensure that the signal is
    // always a multiple of your block size? HINT - You should be able to remove the outer if-guard.
    //
    // 2) Make a new version of 1D convolution which uses shared memory and zero padding.
    // See which is the fastest, is it the signal in shared memory, is it the filter in
    // shared memory, is it both?
    // Make sure to reduce the amount of branching (if-statements) in your code, and ensure
    // coalesced accessing when loading data into shared memory.
    // What happens when you set the block size to different multiples of 32? Why do you think that is?
    //
    // HINT - You need a run_compute_shader() call per type of compute shader.
    // Figure out what the arguments are supposed to be (see vector_add.rs) and
    // call the correct shader function in the correct shader file.
    //

    //
    // YOUR CODE HERE
    let data_naive = run_naive_convolution(handles, signal.clone(), filter.clone());
    let data_shared: Vec<f32> = ground_truth.clone(); // Remove this and replace with your own data
    let data_padded: Vec<f32> = ground_truth.clone(); // Remove this and replace with your own data
                                                      //

    // Naive
    println!(
        "convolution naive MSE: {}",
        mean_square_error(&ground_truth, &data_naive)
    );
    let success: bool = are_vectors_equivalent(&ground_truth, &data_naive);
    println!("convolution naive success: {}!", success);

    // Tiled
    println!(
        "convolution shared MSE: {}",
        mean_square_error(&ground_truth, &data_shared)
    );
    let success: bool = are_vectors_equivalent(&ground_truth, &data_shared);
    println!("convolution shared success: {}!", success);

    // Padded
    println!(
        "convolution padded MSE: {}",
        mean_square_error(&ground_truth, &data_padded)
    );
    let success: bool = are_vectors_equivalent(&ground_truth, &data_padded);
    println!("convolution padded success: {}!", success);

    success
}
