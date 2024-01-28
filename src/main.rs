

// warnings to ignore
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// might use these later
#![allow(unused_imports)]

#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(temporary_cstring_as_ptr)] // when i do .as_ptr() as a param to a func it will dealloc after

// create any modules i need
pub mod MainGameLoop; // where i create the window and renderer and the main loop
pub mod Renderer;
pub mod Camera; // anything to do with camera
pub mod GPUKernels; // where i write all gpu kernels
pub mod Kernels;    // where i store the compiled kernels
pub mod Objects; // where i create my basic objects like spheres and squares
pub mod World; // this is where all of the objects in the world are stored
pub mod Settings; // this is a const settings file for things like chunk size, ray bounces ect.\
pub mod Chunk; // where the blocks and chunks are stored
pub mod GPUData; // where the vbo vao ebo, and vertex and index buffers are as well as textures



fn main() {

    MainGameLoop::RunMainGameLoop();

}
