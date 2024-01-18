// This file will be for all rendering to windows

use std::ptr;

use opencl3::device::{Device, CL_DEVICE_TYPE_GPU, get_all_devices};
use opencl3::context::{Context};
use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::memory::{Buffer, CL_MEM_READ_WRITE};
use opencl3::types::{cl_uint};

use minifb::{Window, WindowOptions};

pub struct Renderer {
    pub screenWidth: u32,
    pub screenHeight: u32,
    pub totalPixels: u32,
    pub window: Window,

    pub device: Device,
    pub context: Context,
    pub queue: CommandQueue,

    pub pixelBuffer1: Vec<[u32]>,
    pub pixelBuffer2: Vec<[u32]>,
    pub pixelBuffer3: Vec<[u32]>,

    pub gpuPixelBuffer1: Buffer::<cl_uint>,
    pub gpuPixelBuffer2: Buffer::<cl_uint>,
    pub gpuPixelBuffer3: Buffer::<cl_uint>,


}

// this is where i write the functions for the Renderer Struct
pub fn CreateRenderer(width: u32, height: u32) -> Renderer {

    // Find a usable GPU device for this application
    let device_id = *get_all_devices(CL_DEVICE_TYPE_GPU)?
        .first()
        .expect("no device found in platform");
    let device = Device::new(device_id);

    // Create a Context on an OpenCL device
    let context = Context::from_device(&device).expect("Context::from_device failed");

    // Create a command queue with the specified properties
    let queue = CommandQueue::create_default_with_properties(&context, CL_QUEUE_PROFILING_ENABLE, 0)
        .expect("Failed to create command queue with properties");


    // create window
    let mut window = Window::new(
        "RustCraft",
        width,
        height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });


    // create pixel buffers (CPU)
    let totalPixels: u32 = width * height;
    let mut buffer1 = vec![0u32; totalPixels];
    let mut buffer2 = vec![0u32; totalPixels];
    let mut buffer3 = vec![0u32; totalPixels];

    // create the gpu pixel buffers
    let mut gpuPixelBuffer1 = unsafe { Buffer::<cl_uint>::create(&context, CL_MEM_READ_WRITE, totalPixels, ptr::null_mut())? };
    let mut gpuPixelBuffer2 = unsafe { Buffer::<cl_uint>::create(&context, CL_MEM_READ_WRITE, totalPixels, ptr::null_mut())? };
    let mut gpuPixelBuffer3 = unsafe { Buffer::<cl_uint>::create(&context, CL_MEM_READ_WRITE, totalPixels, ptr::null_mut())? };



    let r: Renderer = Renderer {
        screenWidth: width,
        screenHeight: height,
        totalPixels: totalPixels,
        window: window,

        device: device,
        context: context,
        queue: queue,

        pixelBuffer2: buffer2,
        pixelBuffer2: buffer2,
        pixelBuffer3: buffer3,

        gpuPixelBuffer1: gpuPixelBuffer1,
        gpuPixelBuffer2: gpuPixelBuffer2,
        gpuPixelBuffer3: gpuPixelBuffer3,
    };
    return r;
}


