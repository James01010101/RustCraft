// THis will be all the main code to do all calculations for the frame before it is rendered
use crate::{
    camera::*, character::*, file_system::*, gpu_data::*, my_keyboard::*, renderer::*, world::*,
};

use winit::{dpi::PhysicalPosition, window::Window};

// do any game logic each frame
pub fn calculate_frame(
    renderer: &mut Renderer,
    gpu_data: &mut GPUData,
    world: &mut World,
    character: &mut Character,
    keyboard: &mut MyKeyboard,
    camera: &mut Camera,
    window: &Window,
    file_system: &mut FileSystem,
    use_cursor: bool,
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

    // go through the pending chunks vec and any that are valid now are put into chunks
    let mut i = 0;
    while i < world.pending_chunks.len() {
        // call update so it can finish off its copy when ready
        world.pending_chunks[i].update(renderer);
        if world.pending_chunks[i].instance_capacity > world.pending_chunks[i].instance_size {
            let chunk = world.pending_chunks.remove(i);
            world.chunks.insert((chunk.chunk_id_x, chunk.chunk_id_z), chunk);
        } else {
            i += 1;
        }
    }


    // update the chunks that are loaded in the world around the player only if the chunk position changed
    if character.chunk_changed {
        character.chunk_changed = false;
        world.update_chunks_around_character(character, renderer, file_system);
    }

    // Calculate the new view and combined matrices
    camera.update(renderer, gpu_data, character);

    // poll the gpu to finish and call any callbacks functions
    renderer.device.poll(wgpu::Maintain::Poll);

    // update all chunks instances if needed
    for chunk in world.chunks.values_mut() {
        chunk.update(renderer);
    }
}
