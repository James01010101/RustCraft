

// THis will be all the main code to do all calculations for the frame before it is rendered
use crate::{
    renderer::*,
    gpu_data::*,
    world::*,
    character::*,
    my_keyboard::*,
    camera::*,
};

use winit::{
    dpi::LogicalPosition,
    window::Window,
};


    // do any game logic each frame
pub fn calculate_frame(renderer: &mut Renderer, gpu_data: &mut GPUData, world: &mut World, character: &mut Character, keyboard: &mut MyKeyboard, camera: &mut Camera, window: &Window) {

    // check the keyboard for any key presses
    if keyboard.w_held {
        character.move_forward(0.1);
    }
    if keyboard.s_held {
        character.move_forward(-0.1);
    }

    if keyboard.d_held {
        character.move_sideways(-0.1);
    }
    if keyboard.a_held {
        character.move_sideways(0.1);
    }

    // mouse and camera movement
    character.update_view(keyboard);

    // set the cursors position back to 0,0 
    window.set_cursor_position(LogicalPosition::new(keyboard.mouse_center_position.0, keyboard.mouse_center_position.1)).unwrap();
    //window.set_cursor_position(LogicalPosition::new(700.0, 400.0)).unwrap();

    // update characters chunk position
    character.update_chunk_position();



    // Calculate the new view and combined matrices
    camera.update(renderer, gpu_data, character);

    // update all chunks instances if needed
    for chunk in world.chunks.values_mut() {
        chunk.update_instance_buffer(renderer);
    }
}