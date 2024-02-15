
use crate::file_system::FileSystem;
use crate::renderer::*;
use crate::settings::*;
use crate::window_wrapper::*;
use crate::world::*;
use crate::gpu_data::GPUData;
use crate::block::*;
use crate::chunk::*;
use crate::camera::*;

use std::time::Instant;
use std::mem;
use async_std::task;

use winit::{
    event::{ElementState, Event, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub fn run_main_game_loop() {

    //let dontStartScreen: bool = false;


    println!("Size of Block: {} bytes", mem::size_of::<Block>());
    println!("Size of Chunk: {} bytes", mem::size_of::<Chunk>());
    println!("Size of u16: {} bytes", mem::size_of::<u16>());
    println!("Max value of u16: {}", std::u16::MAX);
    println!("Size of bool: {} bytes", mem::size_of::<bool>());
    println!(); // for spacing
    

    // Create the window wrapper
    let window_wrapper: WindowWrapper = WindowWrapper::new("RustCraft", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);

    let mut camera: Camera = Camera::new(
        SCREEN_FOV,
        SCREEN_WIDTH as u32,
        SCREEN_HEIGHT as u32
    );

    // create Renderer and window
    let mut renderer: Renderer = task::block_on(Renderer::new(&window_wrapper, &camera));


    // create MY file system struct
    let mut file_system: FileSystem = FileSystem::new();
    file_system.check_file_system();

    // create my world
    let mut world: World = World::new();
    // temp, add some blocks for testing
    world.load_created_chunks_file(&mut file_system);
    //world.AddTestBlocks();
    world.add_test_chunks(&mut file_system, &renderer);

    // create the gpudata buffers
    let gpu_data: GPUData = GPUData::new(&renderer);
    
    // camera stuff for testing
    let mut angle: f32 = 0.0; // Current angle of rotation
    let rotation_speed: f32 = 0.008; // Speed of rotation
    let radius: f32 = 3.0; // Distance from the center

 
    // stats before starting
    let mut frame_number: u64 = 0;
    let window_start_time: Instant = Instant::now();

    // event loop
    window_wrapper.event_loop
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

                        renderer.surface_config.width = new_width;
                        renderer.surface_config.height = new_height;
                        renderer.surface.configure(&renderer.device, &renderer.surface_config);
                        
                        // so it always generates a new frame
                        window_wrapper.window.request_redraw();

                        // update the cameras width height and aspect ratio
                        camera.aspect_ratio = new_width as f32 / new_height as f32;
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
                        camera.update(&mut renderer, &gpu_data);

                        // update all chunks instances if needed
                        for chunk in world.chunks.values_mut() {
                            chunk.update_instance_buffer(&renderer);
                        }

                        // calculate the frame
                        renderer.render_frame(&gpu_data, &world);
                        
                        // so it always generates a new frame
                        window_wrapper.window.request_redraw();

                        // TODO: #103 fix frame number counting
                        frame_number += 1;
                    }
                    // take keyboard input for the esc key
                    // ...

                    WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                        match event.physical_key {
                            PhysicalKey::Code(KeyCode::Escape) => {
                                
                                println!("Escape key pressed");
                                // request a close so the cleanup can happen
                                // cleanup which saves all chunks to files
                                clean_up(&mut world, &mut file_system);
                                target.exit();
                            }
                            PhysicalKey::Code(KeyCode::KeyW) => {
                                
                                match event.state {
                                    ElementState::Pressed => {
                                        println!("W key pressed");
                                    }
                                    ElementState::Released => {
                                        println!("W key released");
                                    }
                                };

                            }
                            PhysicalKey::Code(KeyCode::KeyA) => {
                                println!("A key pressed");

                            }
                            PhysicalKey::Code(KeyCode::KeyS) => {
                                println!("S key pressed");

                            }
                            PhysicalKey::Code(KeyCode::KeyD) => {
                                println!("D key pressed");

                            }
                            _ => {} // default

                            
                        };
                        
                    }


                    // if i close the window
                    WindowEvent::CloseRequested => {

                        // cleanup which saves all chunks to files
                        clean_up(&mut world, &mut file_system);

                        // finally exit the program
                        target.exit();
                    }
                    _ => {}
                };
            }
        })
        .unwrap();

    let total_window_duration_ms = window_start_time.elapsed().as_millis();
    let avg_fps: f32 = frame_number as f32 / (total_window_duration_ms as f32 / 1000.0);
    println!("\nTotal Window Time (ms): {:?}", total_window_duration_ms);
    println!("Total Frames Rendered: {}", frame_number);
    println!("Average Frame Rate: {}", avg_fps);
}


// this will clean up all data before the program ends
pub fn clean_up(world: &mut World, file_system: &mut FileSystem) {

    let hashmap_chunk_keys: Vec<(i32, i32)> = world.chunks.keys().cloned().collect();

    // go through each chunk and call unload on it
    //let mut chunk: &Chunk;

    for key in  hashmap_chunk_keys {
        // remove the chunk from the hashmap and return it
        if let Some(chunk) = world.chunks.remove(&key) {
            file_system.save_chunk_to_file(chunk);
        } else {
            // if the key doesnt match a value ill print this but not panic so i can save the rest
            eprintln!("Failed CleanUp: cannot to find value with key {:?}", key);
        }
    }

    file_system.save_created_chunks_file(world);
}