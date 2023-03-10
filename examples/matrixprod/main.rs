mod matrix;

use crate::matrix::Matrix;
use metal::{Device, MTLResourceOptions};
use metal_playground::utils;

struct MetalState<'a> {
    pub device: &'a metal::DeviceRef,
    pub queue: metal::CommandQueue,
    pub pipeline: metal::ComputePipelineState,
}

const LIB_DATA: &[u8] = include_bytes!("metal/matrixprod.metallib");

/// Parallel computation of a matrix multiplication.
/// Only admits square matrices.
fn prod<T: Copy>(ma: &Matrix<T>, mb: &Matrix<T>, state: MetalState) -> *mut std::ffi::c_void {
    assert!(ma.is_square());
    assert!(mb.is_square());
    assert_eq!(ma.rows, mb.rows);
    let size = ma.sizeof_entries();

    let buffer_a = state.device.new_buffer_with_data(
        utils::void_ptr(&ma.entries),
        size,
        MTLResourceOptions::StorageModeShared,
    );
    let buffer_b = state.device.new_buffer_with_data(
        utils::void_ptr(&mb.entries),
        size,
        MTLResourceOptions::StorageModeShared,
    );
    let buffer_result = state.device.new_buffer(
        size, // the result will be another suqare matrix of the same size
        MTLResourceOptions::StorageModeShared,
    );

    let command_buffer = state.queue.new_command_buffer();
    let compute_encoder = command_buffer.new_compute_command_encoder();
    compute_encoder.set_compute_pipeline_state(&state.pipeline);
    compute_encoder.set_buffers(
        0,
        &[Some(&buffer_a), Some(&buffer_b), Some(&buffer_result)],
        &[0; 3],
    );

    let n = ma.rows as u64;
    let w = state.pipeline.thread_execution_width();
    let h = state.pipeline.max_total_threads_per_threadgroup() / w;
    let grid_size = metal::MTLSize::new(n, n, 1);
    let threadgroup_size = metal::MTLSize::new(w, h, 1);
    compute_encoder.dispatch_threads(grid_size, threadgroup_size);

    // end encoding and execute commands
    compute_encoder.end_encoding();
    command_buffer.commit();

    command_buffer.wait_until_completed();

    buffer_result.contents()
}

fn main() {
    let device: &metal::DeviceRef = &Device::system_default().expect("No device found");
    let queue = device.new_command_queue();

    let lib = device.new_library_with_data(LIB_DATA).unwrap();

    let function = lib.get_function("mul_matrices", None).unwrap();
    let pipeline = device
        .new_compute_pipeline_state_with_function(&function)
        .unwrap();

    let state = MetalState {
        device,
        queue,
        pipeline,
    };

    let matrix_a = Matrix::new(4, 4, &[1.0; 16]);
    let matrix_b = Matrix::new(4, 4, &[2.0; 16]);

    let result = prod(&matrix_a, &matrix_b, state) as *const [f32; 16];

    unsafe {
        println!("{:?}", *result);
    };
}
