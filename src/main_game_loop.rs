
use crate::{
    file_system::*,
    renderer::*,
    window_wrapper::*,
    world::*,
    gpu_data::*,
    block::*,
    chunk::*,
    camera::*,
    character::*,
    my_keyboard::*,
    calculate_frame::*,
};

use std::{
    borrow::BorrowMut,
    mem, 
    sync::{Arc, Mutex}, 
    time::Instant,
};

use async_std::task;

use winit::{
    event::{ElementState, Event, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

pub fn run_main_game_loop() {

    //let dontStartScreen: bool = false;


    println!("Size of Block: {} bytes", mem::size_of::<Block>());
    println!("Size of Chunk: {} bytes", mem::size_of::<Chunk>());
    println!("Size of u16: {} bytes", mem::size_of::<u16>());
    println!("Max value of u16: {}", std::u16::MAX);
    println!("Size of bool: {} bytes", mem::size_of::<bool>());
    println!(); // for spacing
    

    let mut camera: Camera = Camera::new(
        90.0,
        1920,
        1080,
    );

    // Create the window wrapper
    let mut window_wrapper: WindowWrapper = WindowWrapper::new("RustCraft", camera.screen_width as u32, camera.screen_height as u32);

    // create Renderer and window
    let mut renderer: Renderer = task::block_on(Renderer::new(&window_wrapper, &camera));


    // create MY file system struct
    let mut file_system: FileSystem = FileSystem::new();
    

    // create my world
    let mut world: World = World::new(
        "James's World".to_string(),
        1,
        5,
        (8, 16, 8),
    );

    // create the gpudata buffers
    let mut gpu_data: GPUData = GPUData::new(&renderer);

    // create keyboard
    let mut keyboard: MyKeyboard = MyKeyboard::new(
        (camera.screen_width as f32 / 2.0, camera.screen_height as f32 / 2.0),
        (0.002, 0.003),
    );

    // load character
    let mut character: Character = Character::new(0.1);



    // validate the file system and add files and folders if needed
    file_system.check_file_system(&world);

    // temp, add some blocks for testing
    world.load_created_chunks_file(&mut file_system);

 



    
 
    // stats before starting
    let frame_number_outside: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
    let frame_number_inside: Arc<Mutex<u64>> = frame_number_outside.clone(); // use inside the run loop

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

                        keyboard.update_screen_center(new_width as f32 / 2.0, new_height as f32 / 2.0);

                        println!("Resized to: {} x {}", new_width, new_height);
                    }

                    WindowEvent::RedrawRequested => {

                        let window_locked: &Arc<Window> = window_wrapper.window.borrow_mut();
                        calculate_frame(&mut renderer, &mut gpu_data, &mut world, &mut character, &mut keyboard, &mut camera, &window_locked, &mut file_system);

                        

                        // calculate the frame
                        renderer.render_frame(&gpu_data, &world);
                        
                        // so it always generates a new frame
                        window_wrapper.window.request_redraw();

                        let mut frame_number = frame_number_inside.lock().unwrap();
                        *frame_number += 1;
                    }

                    WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                        match event.physical_key {
                            PhysicalKey::Code(KeyCode::Escape) => {
                                // request a close so the cleanup can happen
                                // cleanup which saves all chunks to files
                                clean_up(&mut world, &mut file_system);
                                target.exit();
                            }
                            PhysicalKey::Code(KeyCode::KeyW) => {
                                match event.state {
                                    ElementState::Pressed => { keyboard.pressed_w(); }
                                    ElementState::Released => { keyboard.released_w(); }
                                };
                            }
                            PhysicalKey::Code(KeyCode::KeyA) => {
                                match event.state {
                                    ElementState::Pressed => { keyboard.pressed_a(); }
                                    ElementState::Released => { keyboard.released_a(); }
                                };
                            }
                            PhysicalKey::Code(KeyCode::KeyS) => {
                                match event.state {
                                    ElementState::Pressed => { keyboard.pressed_s(); }
                                    ElementState::Released => { keyboard.released_s(); }
                                };
                            }
                            PhysicalKey::Code(KeyCode::KeyD) => {
                                match event.state {
                                    ElementState::Pressed => { keyboard.pressed_d(); }
                                    ElementState::Released => { keyboard.released_d(); }
                                };
                            }
                            _ => {} // default
                        };
                    }

                    WindowEvent::CursorMoved { device_id: _, position } => {
                        keyboard.update_mouse_position(position.x as f32, position.y as f32);
                        //println!("Mouse Position: ({}, {})", position.x, position.y);

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

    let frame_number = *frame_number_outside.lock().unwrap();
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
        world.remove_chunk(key, file_system);
    }

    file_system.save_created_chunks_file(world);
}