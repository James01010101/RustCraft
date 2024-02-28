extern crate rust_craft;
use rust_craft::{
    character::*, 
    optimisations::world_get_chunks_around_character::*,
    world::World
};

use std::collections::HashSet;

// these all assume chunk sizes of (32 x 256 x 32)
fn test_get_chunks_around_character(render_distance: usize, correct_chunks: Vec<(i32, i32)>) {

    let mut world = World::new("test_world".to_string(), 0, render_distance, (32, 256, 32));
    let mut character: Character = Character::new(0.1);

    // update the players chunk
    character.chunk_position = (0, 0);  

    // run the function
    let result: HashSet<(i32, i32)> = world.get_chunks_around_character(&character);

    //println!("result: {:?}", result);

    assert_eq!(result.len(), correct_chunks.len(), "Length of result and correct chunks vectors are different");
    // go throuch the correct chunks and check that they are in the result
    for chunk in correct_chunks {
        assert_eq!(result.contains(&chunk), true);
    }
}


// actual tests for the function
#[test]
fn test_get_chunks_around_character_render_distance_1() {
    let render_distance: usize = 1;
    let correct_chunks: Vec<(i32, i32)> = vec![
        (0, 0),
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
    ];

    test_get_chunks_around_character(render_distance, correct_chunks);
}

#[test]
fn test_get_chunks_around_character_render_distance_2() {
    let render_distance: usize = 2;
    let correct_chunks: Vec<(i32, i32)> = vec![
        (0, 2),

        (1, 1),
        (0, 1),
        (-1, 1),

        (2, 0),
        (1, 0),
        (0, 0),
        (-1, 0),
        (-2, 0),

        (1, -1),
        (0, -1),
        (-1, -1),

        (0, -2),
    ];

    test_get_chunks_around_character(render_distance, correct_chunks);
}


#[test]
fn test_get_chunks_around_character_render_distance_3() {
    let render_distance: usize = 3;
    let correct_chunks: Vec<(i32, i32)> = vec![
        (0, 3),

        (1, 2),
        (0, 2),
        (-1, 2),

        (2, 1),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-2, 1),

        (3, 0),
        (2, 0),
        (1, 0),
        (0, 0),
        (-1, 0),
        (-2, 0),
        (-3, 0),

        (2, -1),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-2, -1),

        (1, -2),
        (0, -2),
        (-1, -2),

        (0, -3),
    ];

    test_get_chunks_around_character(render_distance, correct_chunks);
}

// TODO: #137 write other tests for not chunk (0,0)


// optimisation tests
// since it takes a self i changed it for this to take a world
fn test_opt_get_chunks_around_character<F>(get_chunks_around_character: F)
where
    F: Fn(&mut World, &Character) -> Vec<(i32, i32)>,
{
    let mut world: World = World::new("test_world".to_string(), 0, 3, (32, 256, 32));
    let mut character: Character = Character::new(0.1);

    // update the players chunk
    character.chunk_position = (0, 0);  

    let correct_chunks: Vec<(i32, i32)> = vec![
        (0, 3),

        (1, 2),
        (0, 2),
        (-1, 2),

        (2, 1),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-2, 1),

        (3, 0),
        (2, 0),
        (1, 0),
        (0, 0),
        (-1, 0),
        (-2, 0),
        (-3, 0),

        (2, -1),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-2, -1),

        (1, -2),
        (0, -2),
        (-1, -2),

        (0, -3),
    ];

    // run the function
    let result: Vec<(i32, i32)> = get_chunks_around_character(&mut world, &character);

    assert_eq!(result.len(), correct_chunks.len(), "Length of result and correct chunks vectors are different");
    // go throuch the correct chunks and check that they are in the result
    for chunk in correct_chunks {
        assert_eq!(result.contains(&chunk), true);
    }
}

#[test]
fn test_get_chunks_around_character_old() {
    test_opt_get_chunks_around_character(get_chunks_around_character_old);
}

#[test]
fn test_get_chunks_around_character_new_1() {
    test_opt_get_chunks_around_character(get_chunks_around_character_new_1);
}

#[test]
fn test_get_chunks_around_character_new_2() {
    test_opt_get_chunks_around_character(get_chunks_around_character_new_2);
}
