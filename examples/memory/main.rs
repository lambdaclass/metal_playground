use metal::{ComputePipelineDescriptor, Device, DeviceRef, MTLResourceOptions, MTLResourceUsage};

const LIB_DATA: &[u8] = include_bytes!("metal/memory.metallib");

fn main() {
    let device: &DeviceRef = &Device::system_default().expect("No device found");
    let lib = device.new_library_with_data(LIB_DATA).unwrap();
    let kernel = lib.get_function("assign", None).unwrap();

    let pipeline_state_descriptor = ComputePipelineDescriptor::new();
    pipeline_state_descriptor.set_compute_function(Some(&kernel));

    let pipeline = device
        .new_compute_pipeline_state_with_function(
            pipeline_state_descriptor.compute_function().unwrap(),
        )
        .unwrap();

    let mut data = vec![0_u32; 16];

    let buffer = device.new_buffer_with_bytes_no_copy(
        data.as_mut_ptr() as *mut core::ffi::c_void,
        (data.capacity() * core::mem::size_of::<u32>()) as u64,
        MTLResourceOptions::StorageModeShared,
        None,
    );

    let command_queue = device.new_command_queue();
    let command_buffer = command_queue.new_command_buffer();
    let compute_encoder = command_buffer.new_compute_command_encoder();
    compute_encoder.set_compute_pipeline_state(&pipeline);
    compute_encoder.set_buffer(0, Some(&buffer), 0);

    compute_encoder.use_resource(&buffer, MTLResourceUsage::Write);

    // specify thread count and organization
    let grid_size = metal::MTLSize::new(data.capacity() as u64, 1, 1);
    let threadgroup_size = metal::MTLSize::new(data.capacity() as u64, 1, 1);

    compute_encoder.dispatch_threads(grid_size, threadgroup_size);
    compute_encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    unsafe {
        println!("{:?}", *(buffer.contents() as *mut [u32; 16])); 
    }
    println!("{:?}", data);
}
