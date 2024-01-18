
use opencl3::types::{CL_NON_BLOCKING, CL_BLOCKING};
use opencl3::kernel::{ExecuteKernel};
use crate::Renderer::*;
use crate::Kernels::Kernels;

// test kernel, gradient colours
pub const pixelShiftKernelName: &str = "PixelShift";
pub const pixelShiftKernel: &str = r#"
kernel void PixelShift (global uint* pixels, uint height, uint width)
{
    const size_t id = get_global_id(0);
    const size_t idy = id / width; // pixel row
    const size_t idx = id % width; // pixel col

    pixels[id] = pixels[id] + 1;
}"#;
/*
    // 0xAARRGGBB
    // shift left so i get rid of front, then all the way right so its in the last 2 hex 
    uint a = pixel >> 24;
    uint r = (pixel << 8) >> 24;
    uint g = (pixel << 16) >> 24;
    uint b = (pixel << 32) >> 24;

    // increase rgb by 1
    r = (r + 1) % 256;
    g = (g + 1) % 256;
    b = (b + 1) % 256;
    uint pixelColour = (a << 24) | (r << 16) | (g << 8) | b;
*/


pub fn RunPixelShiftKernel(renderer: &mut Renderer, kernels: &Kernels) {
    let gpuPixelBuff1_write_event = unsafe { 
        renderer.queue.enqueue_write_buffer(&mut renderer.gpuPixelBuffer1, CL_NON_BLOCKING, 0, &renderer.pixelBuffer1, &[])
        .expect("Failed to enqueue_write_buffer") 
    };
    
    // make and run the kernel
    let kernel_event = unsafe {
        ExecuteKernel::new(&kernels.pixelShiftKernel)
            .set_arg(&renderer.gpuPixelBuffer1)
            .set_arg(&(renderer.screenHeight as u32))
            .set_arg(&(renderer.screenWidth as u32))
            .set_global_work_size(renderer.totalPixels)
            .set_wait_event(&gpuPixelBuff1_write_event)
            .enqueue_nd_range(&renderer.queue)
            .expect("Failed to run Kernel")
    };

    kernel_event.wait().expect("Failed to wait for kernel to finish"); // wait for kernel to finish
    let read_event = unsafe { 
        renderer.queue.enqueue_read_buffer(&renderer.gpuPixelBuffer1, CL_BLOCKING, 0, &mut renderer.pixelBuffer1, &[])
            .expect("Failed to enqueue_read_buffer") 
    };

    // Calculate the kernel duration, from the kernel_event
    let start_time = kernel_event.profiling_command_start().expect("Failed GPU profiling");
    let end_time = kernel_event.profiling_command_end().expect("Failed GPU profiling");
    let duration = end_time - start_time;
    println!("Pixel Gradient Kernel execution duration (ns): {}", duration);
}