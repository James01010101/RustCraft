// This file will be for all rendering to windows

use std::ptr;
use std::mem;

use opencl3::device::{Device, CL_DEVICE_TYPE_GPU, get_all_devices};
use opencl3::context::{Context};
use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::memory::{Buffer, CL_MEM_READ_WRITE};
use opencl3::types::{cl_uint};

use minifb::{Window, WindowOptions};


pub struct Renderer {
    pub screenWidth: usize,
    pub screenHeight: usize,
    pub totalPixels: usize,
    pub window: Window,

    pub device: Device,
    pub context: Context,
    pub queue: CommandQueue,

    pub pixelBuffer1: Vec<u32>,
    pub pixelBuffer2: Vec<u32>,
    pub pixelBuffer3: Vec<u32>,

    pub gpuPixelBuffer1: Buffer::<cl_uint>,
    pub gpuPixelBuffer2: Buffer::<cl_uint>,
    pub gpuPixelBuffer3: Buffer::<cl_uint>,
}

// this is where i write the functions for the Renderer Struct
pub fn CreateRenderer(width: usize, height: usize) -> Renderer {

    // Find a usable GPU device for this application
    // calls the get all devices func, if ok then continues, else if it causes an error it is delt with
    let device_id = *get_all_devices(CL_DEVICE_TYPE_GPU)
        .expect("Failed to get all devices")
        .first()
        .expect("No device found in platform");

    let device = Device::new(device_id);

    // Create a Context on an OpenCL device
    let context = Context::from_device(&device).expect("Context::from_device failed");

    // Create a command queue with the specified properties
    let queue = CommandQueue::create_default_with_properties(&context, CL_QUEUE_PROFILING_ENABLE, 0)
        .expect("Failed to create command queue with properties");


    // create window
    let window = Window::new(
        "RustCraft",
        width,
        height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });


    // create pixel buffers (CPU)
    let totalPixels: usize = width * height;
    let mut buffer1 = vec![0u32; totalPixels];
    let mut buffer2 = vec![0u32; totalPixels];
    let mut buffer3 = vec![0u32; totalPixels];

    // create the gpu pixel buffers
    let mut gpuPixelBuffer1 = unsafe { Buffer::<cl_uint>::create(&context, CL_MEM_READ_WRITE, totalPixels, ptr::null_mut())
            .expect("Failed to create GPU buffer") };
    let mut gpuPixelBuffer2 = unsafe { Buffer::<cl_uint>::create(&context, CL_MEM_READ_WRITE, totalPixels, ptr::null_mut())
            .expect("Failed to create GPU buffer") };
    let mut gpuPixelBuffer3 = unsafe { Buffer::<cl_uint>::create(&context, CL_MEM_READ_WRITE, totalPixels, ptr::null_mut())
            .expect("Failed to create GPU buffer") };


    let r: Renderer = Renderer {
        screenWidth: width,
        screenHeight: height,
        totalPixels: totalPixels,
        window: window,

        device: device,
        context: context,
        queue: queue,

        pixelBuffer1: buffer1,
        pixelBuffer2: buffer2,
        pixelBuffer3: buffer3,

        gpuPixelBuffer1: gpuPixelBuffer1,
        gpuPixelBuffer2: gpuPixelBuffer2,
        gpuPixelBuffer3: gpuPixelBuffer3,
    };
    return r;
}


// will swap buffers 1 & 2 and then show the new buffer 1 to the screen
pub fn RenderToScreen(renderer: &mut Renderer) {
    mem::swap(&mut renderer.pixelBuffer1, &mut renderer.pixelBuffer2);
    renderer.window.update_with_buffer(&renderer.pixelBuffer1, renderer.screenWidth, renderer.screenHeight).unwrap();
}


