
use crate::Renderer::*;
use crate::Kernels::{Kernels, CreateKernels};
use crate::GPUKernels::PixelGradient::RunPixelGradientKernel;
use crate::GPUKernels::PixelShift::RunPixelShiftKernel;

use minifb::{Key};
use std::mem;
use std::thread;
use std::time::Duration;

pub fn RunMainGameLoop() {

    // create Renderer and window
    let mut renderer: Renderer = CreateRenderer(800, 600);

    // create my kernels objects which will compile all my kernels
    let kernels: Kernels = CreateKernels(&renderer);

    // Blocking write
    RunPixelGradientKernel(&mut renderer, &kernels);
    mem::swap(&mut renderer.pixelBuffer1, &mut renderer.pixelBuffer2);
    
    let mut frameNumber: u64 = 0;
    

    // Loop until the user closes the window
    while renderer.window.is_open() && !renderer.window.is_key_down(Key::Escape) {
        frameNumber += 1;

        //RunPixelShiftKernel(&mut renderer, &kernels);
        
        // Switch buffers if needed, e.g., based on user input or timing
        // Example: Switch buffer on spacebar press
        if renderer.window.is_key_down(Key::Space) {
            mem::swap(&mut renderer.pixelBuffer1, &mut renderer.pixelBuffer2);
        }

        // render the image
        // always render buffer 1, but swap buffer 1 & 2 as ill be working on 3 and 2 is the newest finished frame
        //mem::swap(&mut renderer.pixelBuffer1, &mut renderer.pixelBuffer2);
        renderer.window.update_with_buffer(&renderer.pixelBuffer1, renderer.screenWidth, renderer.screenHeight).unwrap();

        //thread::sleep(Duration::from_secs(1));
    }   
}
