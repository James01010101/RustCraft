#![allow(non_snake_case)]

extern crate rust_craft;
use rust_craft::chunk::chunk_functions::*;




#[test]
fn Test_GetChunkId() {

    let posX: i32 = 0;
    let posY: i32 = 0;

    let correctChunkX: i32 = 0;
    let correctChunkY: i32 = 0;

    let ids: (i32, i32) = get_chunk_id(posX, posY);

    assert_eq!(ids, (correctChunkX, correctChunkY), 
        "input Position ({}, {}) - Result Ids ({}, {}) - Expected ({}, {})", posX, posY, ids.0, ids.1, correctChunkX, correctChunkY);
}

#[test]
fn Test_GetChunkId_2() {

    let posX: i32 = 10;
    let posZ: i32 = 10;

    let correctChunkX: i32 = 0;
    let correctChunkZ: i32 = 0;

    let ids: (i32, i32) = get_chunk_id(posX, posZ);

    assert_eq!(ids, (correctChunkX, correctChunkZ), 
        "input Position ({}, {}) - Result Ids ({}, {}) - Expected ({}, {})", posX, posZ, ids.0, ids.1, correctChunkX, correctChunkZ);
}

#[test]
fn Test_GetChunkId_3() {

    let posX: i32 = -32;
    let posZ: i32 = 31;

    let correctChunkX: i32 = -1;
    let correctChunkZ: i32 = 0;

    let ids: (i32, i32) = get_chunk_id(posX, posZ);

    assert_eq!(ids, (correctChunkX, correctChunkZ), 
        "input Position ({}, {}) - Result Ids ({}, {}) - Expected ({}, {})", posX, posZ, ids.0, ids.1, correctChunkX, correctChunkZ);
}


#[test]
fn Test_GetChunkId_4() {

    let posX: i32 = -100;
    let posZ: i32 = 68;

    let correctChunkX: i32 = -3;
    let correctChunkZ: i32 = 2;

    let ids: (i32, i32) = get_chunk_id(posX, posZ);

    assert_eq!(ids, (correctChunkX, correctChunkZ), 
        "input Position ({}, {}) - Result Ids ({}, {}) - Expected ({}, {})", posX, posZ, ids.0, ids.1, correctChunkX, correctChunkZ);
}



#[test]
fn Test_GetWorldBlockPos_1() {

    let chunkX: i32 = 0;
    let chunkZ: i32 = 0;

    let relBlockX: i32 = 0;
    let relBlockY: i16 = 0;
    let relBlockZ: i32 = 0;

    let answer: (i32, i16, i32) = (0, 0, 0);

    let result: (i32, i16, i32) = get_world_block_pos(chunkX, chunkZ, relBlockX, relBlockY, relBlockZ);
    
    assert_eq!(result, answer, "Expected ({:?}) - Got ({:?})", answer, result);
}

#[test]
fn Test_GetWorldBlockPos_2() {

    let chunkX: i32 = 1;
    let chunkZ: i32 = 1;

    let relBlockX: i32 = 0;
    let relBlockY: i16 = 0;
    let relBlockZ: i32 = 0;

    let answer: (i32, i16, i32) = (32, 0, 32);

    let result: (i32, i16, i32) = get_world_block_pos(chunkX, chunkZ, relBlockX, relBlockY, relBlockZ);
    
    assert_eq!(result, answer, "Expected ({:?}) - Got ({:?})", answer, result);
}

#[test]
fn Test_GetWorldBlockPos_3() {

    let chunkX: i32 = -2;
    let chunkZ: i32 = 2;

    let relBlockX: i32 = 5;
    let relBlockY: i16 = 5;
    let relBlockZ: i32 = 5;

    let answer: (i32, i16, i32) = (-59, 5, 69);

    let result: (i32, i16, i32) = get_world_block_pos(chunkX, chunkZ, relBlockX, relBlockY, relBlockZ);
    
    assert_eq!(result, answer, "Expected ({:?}) - Got ({:?})", answer, result);
}

#[test]
fn Test_GetWorldBlockPos_4() {

    let chunkX: i32 = -2;
    let chunkZ: i32 = -1;

    let relBlockX: i32 = 5;
    let relBlockY: i16 = 7;
    let relBlockZ: i32 = 18;

    let answer: (i32, i16, i32) = (-59, 7, -14);

    let result: (i32, i16, i32) = get_world_block_pos(chunkX, chunkZ, relBlockX, relBlockY, relBlockZ);
    
    assert_eq!(result, answer, "Expected ({:?}) - Got ({:?})", answer, result);
}


#[test]
fn Test_GetChunkBlockPos_1() {

    let worldBlockX: i32 = 0;
    let worldBlockY: i16 = 0;
    let worldBlockZ: i32 = 0;

    let answer: (i32, i16, i32) = (0, 0, 0);

    let result: (i32, i16, i32) = get_relative_block_pos(worldBlockX, worldBlockY, worldBlockZ);
    
    assert_eq!(result, answer, "Expected ({:?}) - Got ({:?})", answer, result);
}


#[test]
fn Test_GetChunkBlockPos_2() {
    let worldBlockX: i32 = 5;
    let worldBlockY: i16 = 9;
    let worldBlockZ: i32 = 2;

    let answer: (i32, i16, i32) = (5, 9, 2);

    let result: (i32, i16, i32) = get_relative_block_pos(worldBlockX, worldBlockY, worldBlockZ);
    
    assert_eq!(result, answer, "Expected ({:?}) - Got ({:?})", answer, result);
}


#[test]
fn Test_GetChunkBlockPos_3() {
    let worldBlockX: i32 = 50;
    let worldBlockY: i16 = 50;
    let worldBlockZ: i32 = 50;

    let answer: (i32, i16, i32) = (18, 50, 18);

    let result: (i32, i16, i32) = get_relative_block_pos(worldBlockX, worldBlockY, worldBlockZ);
    
    assert_eq!(result, answer, "Expected ({:?}) - Got ({:?})", answer, result);
}


#[test]
fn Test_GetChunkBlockPos_4() {
    let worldBlockX: i32 = -40;
    let worldBlockY: i16 = 50;
    let worldBlockZ: i32 = 40;

    let answer: (i32, i16, i32) = (24, 50, 8);

    let result: (i32, i16, i32) = get_relative_block_pos(worldBlockX, worldBlockY, worldBlockZ);
    
    assert_eq!(result, answer, "Expected ({:?}) - Got ({:?})", answer, result);
}


#[test]
fn Test_GetChunkBlockPos_5() {
    let worldBlockX: i32 = -12;
    let worldBlockY: i16 = 50;
    let worldBlockZ: i32 = -65;

    let answer: (i32, i16, i32) = (20, 50, 31);

    let result: (i32, i16, i32) = get_relative_block_pos(worldBlockX, worldBlockY, worldBlockZ);
    
    assert_eq!(result, answer, "Expected ({:?}) - Got ({:?})", answer, result);
}

#[test]
fn Test_GetChunkBlockPos_6() {
    let worldBlockX: i32 = -32;
    let worldBlockY: i16 = 0;
    let worldBlockZ: i32 = -31;

    let answer: (i32, i16, i32) = (0, 0, 1);

    let result: (i32, i16, i32) = get_relative_block_pos(worldBlockX, worldBlockY, worldBlockZ);
    
    assert_eq!(result, answer, "Expected ({:?}) - Got ({:?})", answer, result);
}

