

// warnings to ignore
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// might use these later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

// create any modules i need
pub mod MainGameLoop; // where i create the window and renderer and the main loop
pub mod Renderer;
pub mod GPUKernels; // where i write all gpu kernels
pub mod Kernels;    // where i store the compiled kernels
pub mod Objects; // where i create my basic objects like spheres and squares
pub mod World; // this is where all of the objects in the world are stored
pub mod Settings; // this is a const settings file for things like chunk size, ray bounces ect.

fn main() {


    MainGameLoop::RunMainGameLoop();
}
