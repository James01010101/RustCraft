

// warnings to ignore
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

// grab any modules i need
pub mod Renderer;
pub mod MainGameLoop;
pub mod Kernels; // this is the object that will store all of the kernels
pub mod GPUKernels; // this is where all of the kernels are written

fn main() -> Result<()> {

    MainGameLoop::RunMainGameLoop();

    Ok(())
}
