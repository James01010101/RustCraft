
use crate::FileSystem::FileSystem;
use crate::Renderer::*;
use crate::Settings::{maxBlocksRendered, screenFOV, screenHeight, screenWidth};
use crate::World::*;
use crate::GPUData::GPUData;
use crate::Block::*;
use crate::Chunk::*;


extern crate gl;
extern crate glfw;

use glfw::{Context, Key, Action};

use std::time::Instant;
use std::mem;

use nalgebra::{Point3, Vector3};


pub fn RunMainGameLoop() {

    let dontStartScreen: bool = false;


    println!("Size of Block: {} bytes", mem::size_of::<Block>());
    println!("Size of Chunk: {} bytes\n", mem::size_of::<Chunk>());



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
    world.LoadCreatedChunksFile(&mut fileSystem);
    world.AddTestBlocks();
    world.AddTestChunks(&mut fileSystem);
    



    // create the gpudata (vao, vbo, ebo)
    let mut gpuData: GPUData = GPUData::new();

    // update the instances buffer with the blocks model matricies
    gpuData.UpdateCubeInstances(&mut world);
    
    
    let mut angle: f32 = 0.0; // Current angle of rotation
    let rotation_speed: f32 = 0.0001; // Speed of rotation
    let radius: f32 = 3.0; // Distance from the center


 
    // stats before starting
    let mut frameNumber: u64 = 0;
    let windowStartTime: Instant = Instant::now();
    while !renderer.window.should_close() {
        frameNumber += 1; // keep increasing frame number

        if dontStartScreen { break; }

        // rotate the camera for testing
        angle += rotation_speed;
        renderer.camera.position.x = radius * angle.cos();
        renderer.camera.position.z = radius * angle.sin();


        // TODO: #35 deal with events
        renderer.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&renderer.events) {
            match event {

                // any key event
                glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
                    match (key, action) {
                        (Key::Escape, Action::Press) => {
                            renderer.window.set_should_close(true);
                        },
                        (Key::Space, Action::Press) => {
                            println!("Space Pressed")
                        },
                        _ => {} // default
                    }
                },

                // cursor moved
                glfw::WindowEvent::CursorPos(newX, newY) => {
                    println!("cursor moved to: {:?}", (newX, newY));
                },

                // mouse click
                glfw::WindowEvent::MouseButton(button, action, modifiers) => {
                    match (button, action) {
                        (glfw::MouseButton::Button1, glfw::Action::Press) => {
                            println!("Left Mouse Button Pressed");
                        },
                        (glfw::MouseButton::Button2, glfw::Action::Press) => {
                            println!("Right Mouse Button Pressed");
                        },
                        _ => {} // default
                    }
                },

                // deal with resize events here
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    renderer.camera.aspectRatio = width as f32 / height as f32;
                    unsafe { gl::Viewport(0, 0, width, height); }
                },

                // deal with user typing characters
                glfw::WindowEvent::Char(character) => {
                    println!("User Typed Character: {:?}", character);

                    // TODO: #69 deal with user typing input
                },

                // default
                _ => {}
            }
        }

        // other calculations for this frame
    

        // Render the frame
        gpuData.RenderFrame(&mut renderer);
        
        

    }

    CleanUp(&mut world, &mut fileSystem);


    let totalWindowDuration_ms = windowStartTime.elapsed().as_millis();
    let AvgFPS: f32 = frameNumber as f32 / (totalWindowDuration_ms as f32 / 1000.0);
    println!("\nTotal Window Time (ms): {:?}", totalWindowDuration_ms);
    println!("Total Frames Rendered: {}", frameNumber);
    println!("Average Frame Rate: {}", AvgFPS);
}



// this will clean up all data before the program ends
pub fn CleanUp(world: &mut World, fileSystem: &mut FileSystem) {

    let hashmapChunkKeys: Vec<(i32, i32)> = world.chunks.keys().cloned().collect();

    // go through each chunk and call unload on it
    //let mut chunk: &Chunk;

    for key in  hashmapChunkKeys {
        // remove the chunk from the hashmap and return it
        if let Some(mut chunk) = world.chunks.remove(&key) {
            fileSystem.SaveChunkToFile(chunk);
        } else {
            // if the key doesnt match a value ill print this but not panic so i can save the rest
            eprintln!("Failed CleanUp: cannot to find value with key {:?}", key);
        }
    }

    fileSystem.SaveCreatedChunksFile(world);

}