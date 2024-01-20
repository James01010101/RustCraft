
use crate::Renderer::*;
use crate::Kernels::{Kernels, CreateKernels};
use crate::GPUKernels::PixelGradient::RunPixelGradientKernel;
use crate::GPUKernels::PixelShift::RunPixelShiftKernel;
use crate::World::*;
use crate::Chunk::*;

use minifb::{Key};

use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::mem;

pub fn RunMainGameLoop() {

    // create Renderer and window
    let mut renderer: Renderer = CreateRenderer(1920, 1080);

    // create my kernels objects which will compile all my kernels
    let kernels: Kernels = CreateKernels(&renderer);

    // create my world
    let world: World = CreateWorld();

    
    // stats before starting
    let mut frameNumber: u64 = 0;
    let windowStartTime = Instant::now();

    // Loop until the user closes the window
    while renderer.window.is_open() && !renderer.window.is_key_down(Key::Escape) {
        frameNumber += 1;

        
        // Switch buffers if needed, e.g., based on user input or timing
        // Example: Switch buffer on spacebar press
        if renderer.window.is_key_down(Key::Space) {
    
        }

        // render the image to the screen
        RenderToScreen(&mut renderer);
    }   
    let totalWindowDuration_ms = windowStartTime.elapsed().as_millis();
    let AvgFPS: f32 = frameNumber as f32 / (totalWindowDuration_ms as f32 / 1000.0);
    println!("Total Window Time (ms): {:?}", totalWindowDuration_ms);
    println!("Total Frames Rendered: {}", frameNumber);
    println!("Average Frame Rate: {}", AvgFPS);
}
