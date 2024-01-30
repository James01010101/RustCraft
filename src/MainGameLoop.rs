
use crate::FileSystem::FileSystem;
use crate::Renderer::*;
use crate::Settings::{maxBlocksRendered, screenFOV, screenHeight, screenWidth};
use crate::World::*;
use crate::GPUData::GPUData;
use crate::Objects::*;

extern crate gl;
extern crate glfw;

use glfw::Context;

use std::time::Instant;
use std::mem;

use nalgebra::{Point3, Vector3};


pub fn RunMainGameLoop() {

    let dontStartScreen: bool = true;


    let sizeOfBlock: usize = mem::size_of::<Block>();
    //println!("Size of Block: {} bytes", sizeOfBlock);


    // create Renderer and window
    let mut renderer: Renderer = Renderer::new(screenWidth as u32, screenHeight as u32, screenFOV);

    // create my kernels objects which will compile all my kernels
    //let kernels: Kernels = CreateKernels(&renderer);


    // create MY file system struct
    let mut fileSystem: FileSystem = FileSystem::new();

    // check the filesystem has folders and structure i expect
    fileSystem.CheckFileSystem();



    // create my world
    let mut world: World = World::new();

    // temp, add some blocks for testing
    world.AddTestBlocks();
    world.LoadCreatedChunksFile(&mut fileSystem);



    // create the gpudata (vao, vbo, ebo)
    let mut gpuData: GPUData = GPUData::new();

    // update the instances buffer with the blocks model matricies
    gpuData.UpdateCubeInstances(&mut world);
    
    
    let mut angle: f32 = 0.0; // Current angle of rotation
    let rotation_speed: f32 = 0.0001; // Speed of rotation
    let radius: f32 = 3.0; // Distance from the center


    // so i dont need to load the screen all the time if im just debugging
    if dontStartScreen { return; }
    
    // stats before starting
    let mut frameNumber: u64 = 0;
    let windowStartTime: Instant = Instant::now();
    while !renderer.window.should_close() {
        frameNumber += 1; // keep increasing frame number

        // ... event handling ...

        // rotate the camera for testing
        angle += rotation_speed;
        renderer.camera.position.x = radius * angle.cos();
        renderer.camera.position.z = radius * angle.sin();
    

        // Render the frame
        gpuData.RenderFrame(&mut renderer);
        
        // TODO: #35 deal with events
        renderer.glfw.poll_events();

    }

    // TODO: #44 implement clean up when window exit


    let totalWindowDuration_ms = windowStartTime.elapsed().as_millis();
    let AvgFPS: f32 = frameNumber as f32 / (totalWindowDuration_ms as f32 / 1000.0);
    println!("\nTotal Window Time (ms): {:?}", totalWindowDuration_ms);
    println!("Total Frames Rendered: {}", frameNumber);
    println!("Average Frame Rate: {}", AvgFPS);
}
