
extern crate rust_craft;
use rust_craft::chunk::chunk_functions::*;
use rust_craft::world::World;

// these all assume chunk sizes of (32 x 256 x 32)
fn test_get_chunk_id(pos_x: i32, pos_y: i32, correct_chunk_x: i32, correct_chunk_y: i32) {
    let world: World = World::new("James's World".to_string(), 1, 5, (32, 256, 32));
    let ids: (i32, i32) = get_chunk_id(pos_x, pos_y, world.chunk_sizes);

    assert_eq!(
        ids,
        (correct_chunk_x, correct_chunk_y),
        "input Position ({}, {}) - Result Ids ({}, {}) - Expected ({}, {})",
        pos_x,
        pos_y,
        ids.0,
        ids.1,
        correct_chunk_x,
        correct_chunk_y
    );
}

#[test]
fn test_get_chunk_id_1() {
    test_get_chunk_id(0, 0, 0, 0);
}

#[test]
fn test_get_chunk_id_2() {
    test_get_chunk_id(10, 10, 0, 0);
}

#[test]
fn test_get_chunk_id_3() {
    test_get_chunk_id(-32, 31, -1, 0);
}

#[test]
fn test_get_chunk_id_4() {
    test_get_chunk_id(-100, 68, -3, 2);
}




fn test_get_world_block_pos(chunk_x: i32, chunk_z: i32, relative_block_x: i32, relative_block_y: i16, relative_block_z: i32, answer: (i32, i16, i32)) {
    let result: (i32, i16, i32) = get_world_block_pos(chunk_x, chunk_z, relative_block_x, relative_block_y, relative_block_z, (32, 256, 32));

    assert_eq!(
        result, answer,
        "Expected ({:?}) - Got ({:?})",
        answer, result
    );
}

#[test]
fn test_get_world_block_pos_1() {
    test_get_world_block_pos(0, 0, 0, 0, 0, (0, 0, 0));
}

#[test]
fn test_get_world_block_pos_2() {
    test_get_world_block_pos(1, 1, 0, 0, 0, (32, 0, 32));
}

#[test]
fn test_get_world_block_pos_3() {
    test_get_world_block_pos(-2, 2, 5, 5, 5, (-59, 5, 69));
}

#[test]
fn test_get_world_block_pos_4() {
    test_get_world_block_pos(-2, -1, 5, 7, 18, (-59, 7, -14));
}


fn test_get_relative_block_pos(world_block_x: i32, world_block_y: i16, world_block_z: i32, answer: (i32, i16, i32)) {
    let world: World = World::new("James's World".to_string(), 1, 5, (32, 256, 32));
    let result: (i32, i16, i32) = get_relative_block_pos(
        world_block_x, 
        world_block_y, 
        world_block_z, 
        world.chunk_sizes
    );

    assert_eq!(
        result, answer,
        "Expected ({:?}) - Got ({:?})",
        answer, result
    );
}

#[test]
fn test_get_relative_block_pos_1() {
    test_get_relative_block_pos(0, 0, 0, (0, 0, 0));
}

#[test]
fn test_get_relative_block_pos_2() {
    test_get_relative_block_pos(5, 9, 2, (5, 9, 2));
}

#[test]
fn test_get_relative_block_pos_3() {
    test_get_relative_block_pos(50, 50, 50, (18, 50, 18));
}

#[test]
fn test_get_relative_block_pos_4() {
    test_get_relative_block_pos(-40, 50, 40, (24, 50, 8));
}

#[test]
fn test_get_relative_block_pos_5() {
    test_get_relative_block_pos(-12, 50, -65, (20, 50, 31));
}

#[test]
fn test_get_relative_block_pos_6() {
    test_get_relative_block_pos(-32, 0, -31, (0, 0, 1));
}
