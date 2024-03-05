// THis will be all the main code to do all calculations for the frame before it is rendered
use crate::{
    camera::*, character::*, gpu_data::*, my_keyboard::*, renderer::*, world::*,
};
use winit::{dpi::PhysicalPosition, window::Window};

use std::collections::{HashSet, VecDeque};

use std::sync::{Arc, Mutex};

// do any game logic each frame
pub fn calculate_frame(
    renderer: &mut Renderer,
    gpu_data: &mut GPUData,
    world: &mut World,
    character: &mut Character,
    keyboard: &mut MyKeyboard,
    camera: &mut Camera,
    window: &Window,
    use_cursor: bool,
    loading_chunks_queue: Arc<Mutex<VecDeque<((i32, i32), Option<(i32, i32)>)>>>,
) {
    // check the keyboard for any key presses
    // create a movement vector (to see what direction i need to move in)
    character.update_movement(keyboard);
    
    // set the cursors position back to 0,0
    if !use_cursor {
        window.set_cursor_position(PhysicalPosition::new(
                keyboard.mouse_center_position.0 as i32,
                keyboard.mouse_center_position.1 as i32,
            )).unwrap();

            keyboard.update_mouse_position(
            keyboard.mouse_center_position.0,
            keyboard.mouse_center_position.1,
        ); // need this so if no more cursor events come in it wont keep the last position
    }
    

    // update characters chunk position
    character.update_chunk_position(world.chunk_sizes);

    // update the chunks that are loaded in the world around the player only if the chunk position changed
    if character.chunk_changed {
        character.chunk_changed = false;
        let chunks_to_load: HashSet<(i32, i32)> = world.get_chunks_around_character(character);
        world.update_chunks_around_character(chunks_to_load, loading_chunks_queue);
    }

    // Calculate the new view and combined matrices
    { //mutex lock scope
        let device_locked = renderer.device.lock().unwrap();
        let queue_locked = renderer.queue.lock().unwrap();
        
        camera.update(&queue_locked, &mut renderer.vertex_uniforms, gpu_data, character);
    
        // poll the gpu to finish and call any callbacks functions
        device_locked.poll(wgpu::Maintain::Poll);

        // update all chunks instances if needed
        for chunk in world.chunks.lock().unwrap().values_mut() {
            chunk.update(&device_locked, &queue_locked);
        }
    }
}
