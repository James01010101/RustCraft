
// this is a practise kernel 

use opencl3::types::{CL_NON_BLOCKING, CL_BLOCKING};
use opencl3::kernel::{ExecuteKernel};
use crate::Renderer::*;
use crate::Kernels::Kernels;

// test kernel, gradient colours
pub const raytraceKernelName: &str = "raytrace";
pub const raytraceKernel: &str = r#"
kernel void raytrace (global uint* pixels, uint height, uint width)
{
    const size_t id = get_global_id(0);
    const size_t idy = id / width; // pixel row
    const size_t idx = id % width; // pixel col

}"#;


pub fn RunraytraceKernel(renderer: &mut Renderer, kernels: &Kernels) {
    let gpuPixelBuff2_write_event = unsafe { 
        renderer.queue.enqueue_write_buffer(&mut renderer.gpuPixelBuffer2, CL_NON_BLOCKING, 0, &renderer.pixelBuffer2, &[])
        .expect("Failed to enqueue_write_buffer") 
    };
    
    // make and run the kernel
    let kernel_event = unsafe {
        ExecuteKernel::new(&kernels.raytraceKernel)
            .set_arg(&renderer.gpuPixelBuffer2)
            .set_arg(&(renderer.screenHeight as u32))
            .set_arg(&(renderer.screenWidth as u32))
            .set_global_work_size(renderer.totalPixels)
            .set_wait_event(&gpuPixelBuff2_write_event)
            .enqueue_nd_range(&renderer.queue)
            .expect("Failed to run Kernel")
    };

    kernel_event.wait().expect("Failed to wait for kernel to finish"); // wait for kernel to finish
    let read_event = unsafe { 
        renderer.queue.enqueue_read_buffer(&renderer.gpuPixelBuffer2, CL_BLOCKING, 0, &mut renderer.pixelBuffer2, &[])
            .expect("Failed to enqueue_read_buffer") 
    };

    // Calculate the kernel duration, from the kernel_event
    let start_time = kernel_event.profiling_command_start().expect("Failed GPU profiling");
    let end_time = kernel_event.profiling_command_end().expect("Failed GPU profiling");
    let duration = end_time - start_time;
    println!("Pixel Gradient Kernel execution duration (ns): {}", duration);
}