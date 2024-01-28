
use crate::Renderer::*;
use crate::Settings::maxBlocksRendered;
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
    let mut renderer: Renderer = Renderer::new(1920, 1080, 80.0);

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
        gpuData.cubeColours[i] = world.testBlocks[i].colour;

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
    let radius: f32 = 2.0; // Distance from the center
    
    // stats before starting
    let mut frameNumber: u64 = 0;
    let windowStartTime = Instant::now();
    while !renderer.window.should_close() {
        frameNumber += 1;

        // ... event handling ...

        // rotate the camera for testing
        angle += rotation_speed;
        renderer.camera.position.x = radius * angle.cos();
        renderer.camera.position.z = radius * angle.sin();

        // 5. Drawing
        unsafe {
            gl::UseProgram(renderer.openGLProgram);
            
            gl::ClearColor(0.0, 0.0, 0.0, 1.0); // Set clear color (black in this case)
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // Clear the screen

            // bind the specific vao for this object
            gl::BindVertexArray(gpuData.cubeVao);
        }

        // create all of the camera matricies which dont change for each object
        // move the camera
        // Create a perspective projection matrix
        let projectionMatrix = nalgebra::Perspective3::new(
            renderer.camera.aspectRatio, 
            renderer.camera.fov, 
            renderer.camera.nearPlane, 
            renderer.camera.farPlane
        ).to_homogeneous();

        // Create a view matrix
        let eye = Point3::new(renderer.camera.position.x, renderer.camera.position.y, renderer.camera.position.z);
        let target = Point3::new(renderer.camera.target.x, renderer.camera.target.y, renderer.camera.target.z);
        let viewMatrix = nalgebra::Isometry3::look_at_rh(&eye, &target, &-Vector3::y()).to_homogeneous();
        
       
        unsafe {
            // upload these to the gpu
            gl::UniformMatrix4fv(renderer.viewMatrixLocation, 1, gl::FALSE, viewMatrix.as_ptr());
            gl::UniformMatrix4fv(renderer.projectionMatrixLocation, 1, gl::FALSE, projectionMatrix.as_ptr());
            
            // draw
            /*
            gl::DrawElements(
                gl::TRIANGLES,        // Mode
                36,                   // Count of indices
                gl::UNSIGNED_SHORT,   // Type of the indices
                std::ptr::null()      // Offset to the EBO
            );
            */
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                gpuData.cubeTrisIndices.len() as i32, // Assuming cube_indices is defined
                gl::UNSIGNED_SHORT,
                std::ptr::null(),
                gpuData.instancesUsed as i32,
            );
        }
            

        renderer.window.swap_buffers();
        renderer.glfw.poll_events();


    }


    let totalWindowDuration_ms = windowStartTime.elapsed().as_millis();
    let AvgFPS: f32 = frameNumber as f32 / (totalWindowDuration_ms as f32 / 1000.0);
    println!("Total Window Time (ms): {:?}", totalWindowDuration_ms);
    println!("Total Frames Rendered: {}", frameNumber);
    println!("Average Frame Rate: {}", AvgFPS);
}
