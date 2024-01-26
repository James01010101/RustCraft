
use crate::Renderer::*;
use crate::World::*;
use crate::GPUData::GPUData;

extern crate gl;
extern crate glfw;

use glfw::Context;

use std::time::Instant;

use nalgebra::{Point3, Vector3};


pub fn RunMainGameLoop() {

    // create Renderer and window
    let mut renderer: Renderer = Renderer::new(1920, 1080, 80.0);

    // create my kernels objects which will compile all my kernels
    //let kernels: Kernels = CreateKernels(&renderer);

    // create my world
    let world: World = CreateWorld();

    // create the gpudata (vao, vbo, ebo)
    let gpuData: GPUData = GPUData::new();
    

    

    let mut angle: f32 = 0.0; // Current angle of rotation
    let rotation_speed: f32 = 0.0001; // Speed of rotation
    let radius: f32 = 2.0; // Distance from the center
    
    // stats before starting
    let mut frameNumber: u64 = 0;
    let windowStartTime = Instant::now();
    while !renderer.window.should_close() {
        frameNumber += 1;

        // ... event handling ...

        // Update the angle
        angle += rotation_speed;

        // Calculate the camera's new position
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
        
        // Create a model matrix (for translation) once per object, this is where i put its actual world location eg (30, 20, -12)
        let modelMatrix = nalgebra::Translation3::new(
            0.0,
            0.0,
            0.0
        ).to_homogeneous();

        
        unsafe {
            // upload these to the gpu
            gl::UniformMatrix4fv(renderer.viewMatrixLocation, 1, gl::FALSE, viewMatrix.as_ptr());
            gl::UniformMatrix4fv(renderer.projectionMatrixLocation, 1, gl::FALSE, projectionMatrix.as_ptr());
            gl::UniformMatrix4fv(renderer.modelMatrixLocation, 1, gl::FALSE, modelMatrix.as_ptr()); 
            
            // draw
            gl::DrawElements(
                gl::TRIANGLES,        // Mode
                36,                   // Count of indices
                gl::UNSIGNED_SHORT,   // Type of the indices
                std::ptr::null()      // Offset to the EBO
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
