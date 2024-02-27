

use crate::{character::Character, world::World};

// first try
pub fn get_chunks_around_character_old(
    world: &mut World,
    character: &Character,
) -> Vec<(i32, i32)> {
    // these are the chunk that should be currently loaded
    let mut chunks_to_load: Vec<(i32, i32)> = Vec::new();

    // ill have a funciton which gets all chunks which should be loaded here
    // start at my current position and go left and right render distance amount
    // then go up once and go render distance -1 left and right continue until the top
    let max_radius: i32 = world.render_distance as i32;
    let mut current_radius: i32 = max_radius;
    let current_chunk_x = character.chunk_position.0;
    let current_chunk_z = character.chunk_position.1;

    for chunk_z_diff in 0..max_radius + 1 {
        // go left and right all the way
        for chunk_x in (current_chunk_x - current_radius)..(current_chunk_x + current_radius + 1) {

            if chunk_z_diff == 0 {
                // just do this layer
                chunks_to_load.push((chunk_x, current_chunk_z));
            } else {
                // z up
                chunks_to_load.push((chunk_x, current_chunk_z + chunk_z_diff));

                // z down
                chunks_to_load.push((chunk_x, current_chunk_z - chunk_z_diff));
            }
        }

        // now decrease current radius
        current_radius -= 1;
    }

    chunks_to_load
}



// optimised version (removed max radius)
pub fn get_chunks_around_character_new_1(
    world: &mut World,
    character: &Character,
) -> Vec<(i32, i32)> {
    // these are the chunk that should be currently loaded
    let mut chunks_to_load: Vec<(i32, i32)> = Vec::new();

    // ill have a funciton which gets all chunks which should be loaded here
    // start at my current position and go left and right render distance amount
    // then go up once and go render distance -1 left and right continue until the top
    let mut current_radius: i32 = world.render_distance as i32;
    let current_chunk_x = character.chunk_position.0;
    let current_chunk_z = character.chunk_position.1;

    for chunk_z_diff in 0..(world.render_distance as i32 + 1) {
        // go left and right all the way
        for chunk_x in (current_chunk_x - current_radius)..(current_chunk_x + current_radius + 1) {

            if chunk_z_diff == 0 {
                // just do this layer
                chunks_to_load.push((chunk_x, current_chunk_z));
            } else {
                // z up
                chunks_to_load.push((chunk_x, current_chunk_z + chunk_z_diff));

                // z down
                chunks_to_load.push((chunk_x, current_chunk_z - chunk_z_diff));
            }
        }

        // now decrease current radius
        current_radius -= 1;
    }

    chunks_to_load
}

// reorganising the loop so it does z=0 outside the loop
pub fn get_chunks_around_character_new_2(
    world: &mut World,
    character: &Character,
) -> Vec<(i32, i32)> {
    // these are the chunk that should be currently loaded
    let mut chunks_to_load: Vec<(i32, i32)> = Vec::new();

    // ill have a funciton which gets all chunks which should be loaded here
    // start at my current position and go left and right render distance amount
    // then go up once and go render distance -1 left and right continue until the top
    let mut current_radius: i32 = world.render_distance as i32;
    let current_chunk_x = character.chunk_position.0;
    let current_chunk_z = character.chunk_position.1;

    // for z diff = 0
    for chunk_x in (current_chunk_x - current_radius)..(current_chunk_x + current_radius + 1) {
        chunks_to_load.push((chunk_x, current_chunk_z));
    }
    current_radius -= 1;


    // for all other z values
    for chunk_z_diff in 1..(world.render_distance as i32 + 1) {
        // go left and right all the way
        for chunk_x in (current_chunk_x - current_radius)..(current_chunk_x + current_radius + 1) {
                // z up
                chunks_to_load.push((chunk_x, current_chunk_z + chunk_z_diff));

                // z down
                chunks_to_load.push((chunk_x, current_chunk_z - chunk_z_diff));
        }

        // now decrease current radius
        current_radius -= 1;
    }

    chunks_to_load
}