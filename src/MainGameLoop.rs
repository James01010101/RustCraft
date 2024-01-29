
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

    let sizeOfBlock: usize = mem::size_of::<Block>();
    println!("Size of Block: {} bytes", sizeOfBlock);


    // create Renderer and window
    let mut renderer: Renderer = Renderer::new(screenWidth as u32, screenHeight as u32, screenFOV);

    // create my kernels objects which will compile all my kernels
    //let kernels: Kernels = CreateKernels(&renderer);

    // create my world
    let mut world: World = CreateWorld();

    // create the gpudata (vao, vbo, ebo)
    let mut gpuData: GPUData = GPUData::new();


    // update the instances buffer with the blocks model matricies
    gpuData.instancesUsed = world.testBlocks.len() as u32;

    // Instance model matricies, each element is a model matrix of a block
    for i in 0..gpuData.instancesUsed {
        let i: usize = i as usize;
        gpuData.cubeInstanceModelMatricies[i] = world.testBlocks[i].modelMatrix;
        gpuData.cubeColours[i] = world.testBlocks[i].blockType.BlockColour();

        //println!("Instanced Model Matricies [{}]: {:?}", i, instanceModelMatricies[i]);
    }

    // update the data on the gpu
    unsafe {
        // model matrix
        gl::BindBuffer(gl::ARRAY_BUFFER, gpuData.cubeInstanceVbo);
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            0,
            (gpuData.instancesUsed as usize * std::mem::size_of::<[[f32; 4]; 4]>()) as isize,
            gpuData.cubeInstanceModelMatricies.as_ptr() as *const gl::types::GLvoid,
        );

        // colour
        gl::BindBuffer(gl::ARRAY_BUFFER, gpuData.cubeColoursVbo);
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            0,
            (gpuData.instancesUsed as usize * std::mem::size_of::<[f32; 4]>()) as isize,
            gpuData.cubeColours.as_ptr() as *const gl::types::GLvoid,
        );
    }
    

    
    
    let mut angle: f32 = 0.0; // Current angle of rotation
    let rotation_speed: f32 = 0.0001; // Speed of rotation
    let radius: f32 = 3.0; // Distance from the center
    
    // stats before starting
    let mut frameNumber: u64 = 0;
    let windowStartTime = Instant::now();
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


    let totalWindowDuration_ms = windowStartTime.elapsed().as_millis();
    let AvgFPS: f32 = frameNumber as f32 / (totalWindowDuration_ms as f32 / 1000.0);
    println!("Total Window Time (ms): {:?}", totalWindowDuration_ms);
    println!("Total Frames Rendered: {}", frameNumber);
    println!("Average Frame Rate: {}", AvgFPS);
}
