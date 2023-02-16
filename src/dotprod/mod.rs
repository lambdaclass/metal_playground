use metal::{Device, DeviceRef, MTLResourceOptions};
use std::{fs, rc::Rc};

use crate::util;

pub fn dot(v: &[u32], w: &[u32]) -> *const [u32; 4] { // will return a raw pointer to the result
    // the system will assign a GPU to use.
    let device: &DeviceRef = &Device::system_default().expect("No device found");

    let lib_file = fs::read("metal/dot_product.metallib").unwrap();
    // represents the library which contains the kernel.
    let lib = device.new_library_with_data(&lib_file[..]).unwrap();

    // create function pipeline.
    // this compiles the function, so a pipline can't be created in performance sensitive code.
    let function = lib.get_function("dot_product", None).unwrap();
    let pipeline = lib
        .device()
        .new_compute_pipeline_state_with_function(&function)
        .unwrap();

    let length = v.len() as u64;
    let size = length * core::mem::size_of::<i32>() as u64;
    assert_eq!(v.len(), w.len());

    let buffer_a = device.new_buffer_with_data(
        util::void_ptr(&v),
        size,
        MTLResourceOptions::StorageModeShared,
    );
    let buffer_b = device.new_buffer_with_data(
        util::void_ptr(&w),
        size,
        MTLResourceOptions::StorageModeShared,
    );
    let buffer_result = device.new_buffer(
        size, // the operation will return an array with the same size.
        MTLResourceOptions::StorageModeShared,
    );

    // a command queue for sending instructions to the device.
    let command_queue = Rc::new(device.new_command_queue());
    // for sending commands, a command buffer is needed.
    let command_buffer = command_queue.new_command_buffer();
    // to write commands into a buffer an encoder is needed, in our case a compute encoder.
    let compute_encoder = command_buffer.new_compute_command_encoder();
    compute_encoder.set_compute_pipeline_state(&pipeline);
    compute_encoder.set_buffers(
        0,
        &[Some(&buffer_a), Some(&buffer_b), Some(&buffer_result)],
        &[0; 3],
    );

    // specify thread count and organization
    let grid_size = metal::MTLSize::new(length, 1, 1);
    let threadgroup_size = metal::MTLSize::new(length, 1, 1);
    compute_encoder.dispatch_threads(grid_size, threadgroup_size);

    // end encoding and execute commands
    compute_encoder.end_encoding();
    command_buffer.commit();

    command_buffer.wait_until_completed();

    util::deref_void_ptr(buffer_result.contents())
}
