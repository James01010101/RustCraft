

// warnings to ignore
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// might use these later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

// create any modules i need
pub mod MainGameLoop;
pub mod Renderer;
pub mod GPUKernels; // where i write all gpu kernels
pub mod Kernels;    // where i store the compiled kernels


fn main() {

    MainGameLoop::RunMainGameLoop();
}
