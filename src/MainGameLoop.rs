
use crate::FileSystem::FileSystem;
use crate::Renderer::*;
use crate::Settings::{screenFOV, screenHeight, screenWidth};
use crate::WindowWrapper::*;
use crate::World::*;
use crate::GPUData::GPUData;
use crate::Block::*;
use crate::Chunk::*;
use crate::Camera::*;

use std::time::Instant;
use std::mem;
use async_std::task;

use winit::event::{Event, WindowEvent};



pub fn RunMainGameLoop() {

    let dontStartScreen: bool = false;


    println!("Size of Block: {} bytes", mem::size_of::<Block>());
    println!("Size of Chunk: {} bytes", mem::size_of::<Chunk>());
    println!("Size of u16: {} bytes", mem::size_of::<u16>());
    println!("Max value of u16: {}", std::u16::MAX);
    println!("Size of bool: {} bytes", mem::size_of::<bool>());
    println!(); // for spacing
    

    // Create the window wrapper
    let mut windowWrapper: WindowWrapper = WindowWrapper::new("RustCraft", screenWidth as u32, screenHeight as u32);

    let mut camera: Camera = Camera::new(
        screenFOV,
        screenWidth as u32,
        screenHeight as u32
    );

    // create Renderer and window
    let mut renderer: Renderer = task::block_on(Renderer::new(&windowWrapper, &camera));


    // create MY file system struct
    let mut fileSystem: FileSystem = FileSystem::new();
    fileSystem.CheckFileSystem();

    // create my world
    let mut world: World = World::new();
    // temp, add some blocks for testing
    world.LoadCreatedChunksFile(&mut fileSystem);
    //world.AddTestBlocks();
    world.AddTestChunks(&mut fileSystem, &renderer);


    // create the gpudata buffers
    let mut gpuData: GPUData = GPUData::new(&renderer);
    

   
    
    let mut angle: f32 = 0.0; // Current angle of rotation
    let rotation_speed: f32 = 0.008; // Speed of rotation
    let radius: f32 = 3.0; // Distance from the center


 
    // stats before starting
    let mut frameNumber: u64 = 0;
    let windowStartTime: Instant = Instant::now();

    // event loop
    windowWrapper.eventLoop
        .run(move |event, target| {


            // check if the event is a window event, if it use get the event from inside the window event
            if let Event::WindowEvent {
                window_id: _, // ignore this variable
                event,
            } = event
            {
                match event {
                    WindowEvent::Resized(new_size) => {

                        let new_width = new_size.width.max(1);
                        let new_height = new_size.height.max(1);

                        renderer.surfaceConfig.width = new_width;
                        renderer.surfaceConfig.height = new_height;
                        renderer.surface.configure(&renderer.device, &renderer.surfaceConfig);
                        

                        // so it always generates a new frame
                        windowWrapper.window.request_redraw();

                        // update the cameras width height and aspect ratio
                        camera.aspectRatio = new_width as f32 / new_height as f32;
                        camera.calculate_projection_matrix();


                    }
                    WindowEvent::RedrawRequested => {

                        // do any game logic each frame
                        // move the camera first so it can start copying
                        // rotate the camera for testing
                        angle += rotation_speed;
                        camera.position.x = radius * angle.cos();
                        camera.position.z = radius * angle.sin();

                        // Calculate the new view and combined matrices
                        camera.update(&mut renderer, &gpuData);

                        // update all chunks instances if needed
                        for chunk in world.chunks.values() {
                            chunk.update_instance_buffer(&renderer);
                        }


                        // calculate the frame
                        renderer.render_frame(&gpuData);
                        
                        // so it always generates a new frame
                        windowWrapper.window.request_redraw();
                    }
                    WindowEvent::CloseRequested => target.exit(),
                    _ => {}
                };
            }
        })
        .unwrap();

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