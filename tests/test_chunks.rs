#![allow(non_snake_case)]

extern crate rust_craft;
use rust_craft::chunk::chunk_functions::*;
use rust_craft::world::World;

// these all assume chunk sizes of (32 x 256 x 32)
fn test_get_chunk_id(posX: i32, posY: i32, correctChunkX: i32, correctChunkY: i32) {
    let world: World = World::new("James's World".to_string(), 1, 5, (32, 256, 32));
    let ids: (i32, i32) = get_chunk_id(posX, posY, &world);

    assert_eq!(
        ids,
        (correctChunkX, correctChunkY),
        "input Position ({}, {}) - Result Ids ({}, {}) - Expected ({}, {})",
        posX,
        posY,
        ids.0,
        ids.1,
        correctChunkX,
        correctChunkY
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




fn test_get_world_block_pos(chunkX: i32, chunkZ: i32, relBlockX: i32, relBlockY: i16, relBlockZ: i32, answer: (i32, i16, i32)) {
    let result: (i32, i16, i32) = get_world_block_pos(chunkX, chunkZ, relBlockX, relBlockY, relBlockZ, (32, 256, 32));

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














fn test_get_relative_block_pos(worldBlockX: i32, worldBlockY: i16, worldBlockZ: i32, answer: (i32, i16, i32)) {
    let world: World = World::new("James's World".to_string(), 1, 5, (32, 256, 32));
    let result: (i32, i16, i32) = get_relative_block_pos(worldBlockX, worldBlockY, worldBlockZ, &world);

    assert_eq!(
        result, answer,
        "Expected ({:?}) - Got ({:?})",
        answer, result
    );
}

#[test]
fn Test_GetChunkBlockPos_1() {
    test_get_relative_block_pos(0, 0, 0, (0, 0, 0));
}

#[test]
fn Test_GetChunkBlockPos_2() {
    test_get_relative_block_pos(5, 9, 2, (5, 9, 2));
}

#[test]
fn Test_GetChunkBlockPos_3() {
    test_get_relative_block_pos(50, 50, 50, (18, 50, 18));
}

#[test]
fn Test_GetChunkBlockPos_4() {
    test_get_relative_block_pos(-40, 50, 40, (24, 50, 8));
}

#[test]
fn Test_GetChunkBlockPos_5() {
    test_get_relative_block_pos(-12, 50, -65, (20, 50, 31));
}

#[test]
fn Test_GetChunkBlockPos_6() {
    test_get_relative_block_pos(-32, 0, -31, (0, 0, 1));
}
