use crate::{
    calculate_frame::*, 
    camera::*, 
    character::*,
    file_system::*, 
    gpu_data::*,
    my_keyboard::*, 
    renderer::*, 
    window_wrapper::*, 
    world::*,
    chunk_generation_thread::*,
    chunk::*,
};

use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
    time::Instant,
    thread,
    collections::VecDeque,
};

use async_std::task;

use winit::{
    event::{ElementState, Event, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

pub fn run_main_game_loop() {

    let mut camera: Camera = Camera::new(90.0, 1920, 1080);

    // Create the window wrapper
    let mut window_wrapper: WindowWrapper = WindowWrapper::new(
        "RustCraft",
        camera.screen_width as u32,
        camera.screen_height as u32,
    );

    // create Renderer and window
    let mut renderer: Renderer = task::block_on(Renderer::new(&window_wrapper, &camera));

    // create MY file system struct
    let mut file_system: Arc<Mutex<FileSystem>> = Arc::new(Mutex::new(FileSystem::new()));

    // create my world
    let mut world: World = World::new(
        "James's World".to_string(), 
        1, 
        5, 
        (8, 16, 8)
    );

    // create the gpudata buffers
    let mut gpu_data: GPUData = { // mutex lock scope
        GPUData::new(&renderer.device.lock().unwrap(), &renderer.vertex_uniforms)
    };

    // create keyboard
    let mut keyboard: MyKeyboard = MyKeyboard::new(
        (
            camera.screen_width as f32 / 2.0,
            camera.screen_height as f32 / 2.0,
        ),
        (0.002, 0.003),
    );

    // load character
    let mut character: Character = Character::new(0.1);

    // validate the file system and add files and folders if needed
    let mut file_system_locked = file_system.lock().unwrap();
    file_system_locked.check_file_system(world.chunk_sizes, &world.world_name);

    // temp, add some blocks for testing
    world.load_created_chunks_file(&mut file_system_locked);
    drop(file_system_locked);

    let mut use_cursor: bool = false;

    

    
    // create the queue that i will use to load chunks on the chunk generation thread
    let mut loading_chunks_queue: Arc<Mutex<VecDeque<((i32, i32), Option<(i32, i32)>)>>> = Arc::new(Mutex::new(VecDeque::new()));

    // make a continue running variable so i can tell the chunk thread to stop
    let mut continue_running: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));

    // clone all variabled needed for the generation thread
    let loading_chunks_queue_thread_clone: Arc<Mutex<VecDeque<((i32, i32), Option<(i32, i32)>)>>> = loading_chunks_queue.clone();
    let chunk_sizes: (usize, usize, usize) = world.chunk_sizes;
    let created_chunks_clone: Arc<Mutex<std::collections::HashSet<(i32, i32)>>> = world.created_chunks.clone();
    let device_clone: Arc<Mutex<wgpu::Device>> = renderer.device.clone();
    let queue_clone: Arc<Mutex<wgpu::Queue>> = renderer.queue.clone();
    let shader_code: Arc<Mutex<wgpu::ShaderModule>> = renderer.check_air_compute_shader_code.clone();
    let file_system_clone: Arc<Mutex<FileSystem>> = file_system.clone();
    let chunks_clone: Arc<Mutex<std::collections::HashMap<(i32, i32), Chunk>>> = world.chunks.clone();
    let continue_running_thread_clone: Arc<Mutex<bool>> = continue_running.clone();

    // start up the chunk generation thread
    let chunk_generation_thread = thread::spawn(move || run_chunk_generation_thread(
            loading_chunks_queue_thread_clone,
            chunk_sizes,
            created_chunks_clone,
            device_clone,
            queue_clone,
            shader_code,
            file_system_clone,
            chunks_clone,
            continue_running_thread_clone,
        )
    );



    // move any variables i need to into the closure
    let loading_chunks_queue_clone: Arc<Mutex<VecDeque<((i32, i32), Option<(i32, i32)>)>>> = loading_chunks_queue.clone();
    let continue_running_clone: Arc<Mutex<bool>> = continue_running.clone();

    // stats before starting
    let frame_number_outside: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
    let frame_number_inside: Arc<Mutex<u64>> = frame_number_outside.clone(); // use inside the run loop

    let window_start_time: Instant = Instant::now();

    // event loop
    window_wrapper
        .event_loop
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

                        let device_locked = renderer.device.lock().unwrap();
                        renderer.surface.configure(&device_locked, &renderer.surface_config);
                        drop(device_locked);

                        // so it always generates a new frame
                        window_wrapper.window.request_redraw();

                        // update the cameras width height and aspect ratio
                        camera.aspect_ratio = new_width as f32 / new_height as f32;
                        camera.calculate_projection_matrix();

                        keyboard.update_screen_center(new_width as f32 / 2.0, new_height as f32 / 2.0);

                        println!("Resized screen to: {} x {}", new_width, new_height);
                    }

                    WindowEvent::RedrawRequested => {
                        let window_locked: &Arc<Window> = window_wrapper.window.borrow_mut();
                        calculate_frame(
                            &mut renderer,
                            &mut gpu_data,
                            &mut world,
                            &mut character,
                            &mut keyboard,
                            &mut camera,
                            &window_locked,
                            use_cursor,
                            loading_chunks_queue_clone,
                        );

                        // calculate the frame
                        { // to drop the mutex
                            renderer.render_frame(&gpu_data, &world.chunks.lock().unwrap());
                        }

                        // so it always generates a new frame
                        window_wrapper.window.request_redraw();

                        let mut frame_number = frame_number_inside.lock().unwrap();
                        *frame_number += 1;
                        //println!("Frame Number: {}", *frame_number);
                    }

                    WindowEvent::KeyboardInput {
                        device_id: _,
                        event,
                        is_synthetic: _,
                    } => {
                        match event.physical_key {
                            PhysicalKey::Code(KeyCode::Escape) => {
                                // request a close so the cleanup can happen
                                // cleanup which saves all chunks to files
                                clean_up(continue_running_clone);
                                target.exit();
                            }
                            PhysicalKey::Code(KeyCode::KeyW) => {
                                match event.state {
                                    ElementState::Pressed => {
                                        keyboard.pressed_w();
                                    }
                                    ElementState::Released => {
                                        keyboard.released_w();
                                    }
                                };
                            }
                            PhysicalKey::Code(KeyCode::KeyA) => {
                                match event.state {
                                    ElementState::Pressed => {
                                        keyboard.pressed_a();
                                    }
                                    ElementState::Released => {
                                        keyboard.released_a();
                                    }
                                };
                            }
                            PhysicalKey::Code(KeyCode::KeyS) => {
                                match event.state {
                                    ElementState::Pressed => {
                                        keyboard.pressed_s();
                                    }
                                    ElementState::Released => {
                                        keyboard.released_s();
                                    }
                                };
                            }
                            PhysicalKey::Code(KeyCode::KeyD) => {
                                match event.state {
                                    ElementState::Pressed => {
                                        keyboard.pressed_d();
                                    }
                                    ElementState::Released => {
                                        keyboard.released_d();
                                    }
                                };
                            }
                            PhysicalKey::Code(KeyCode::KeyP) => {
                                if event.state == ElementState::Pressed {
                                    use_cursor = !use_cursor;
                                    let window_locked: &Arc<Window> = window_wrapper.window.borrow_mut();
                                    window_locked.set_cursor_visible(use_cursor);
                                    println!("use cursor: {}", use_cursor);
                                }

                            }
                            _ => {} // default
                        };
                    }

                    WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                    } => {
                        keyboard.update_mouse_position(position.x as f32, position.y as f32);
                        //println!("Mouse Position: ({}, {})", position.x, position.y);
                    }

                    // if i close the window
                    WindowEvent::CloseRequested => {
                        // cleanup which saves all chunks to files
                        clean_up(continue_running_clone);

                        // finally exit the program
                        target.exit();
                    }
                    _ => {}
                };
            }
        })
        .unwrap();

    // finally join the threads
    chunk_generation_thread.join().unwrap();

    let frame_number = *frame_number_outside.lock().unwrap();
    let total_window_duration_ms = window_start_time.elapsed().as_millis();
    let avg_fps: f32 = frame_number as f32 / (total_window_duration_ms as f32 / 1000.0);
    println!("\nTotal Window Time (ms): {:?}", total_window_duration_ms);
    println!("Total Frames Rendered: {}", frame_number);
    println!("Average Frame Rate: {}", avg_fps);

}

// this will clean up all data before the program ends
pub fn clean_up(continue_running: Arc<Mutex<bool>>) {

    // send the stop signal to the generation thread
    let mut continue_running_locked = continue_running.lock().unwrap();
    *continue_running_locked = false;
    drop(continue_running_locked);
}
