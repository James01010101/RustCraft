#![allow(non_snake_case)]

extern crate RustCraft;
use RustCraft::Chunk::ChunkFunctions::*;




#[test]
fn Test_GetChunkId() {

    let posX: i32 = 0;
    let posY: i32 = 0;

    let correctChunkX: i32 = 0;
    let correctChunkY: i32 = 0;

    let ids: [i32; 2] = GetChunkId(posX, posY);

    assert_eq!(ids, [correctChunkX, correctChunkY], 
        "input Position ({}, {}) - Result Ids ({}, {}) - Expected ({}, {})", posX, posY, ids[0], ids[1], correctChunkX, correctChunkY);
}

#[test]
fn Test_GetChunkId_2() {

    let posX: i32 = 10;
    let posY: i32 = 10;

    let correctChunkX: i32 = 0;
    let correctChunkY: i32 = 0;

    let ids: [i32; 2] = GetChunkId(posX, posY);

    assert_eq!(ids, [correctChunkX, correctChunkY], 
        "input Position ({}, {}) - Result Ids ({}, {}) - Expected ({}, {})", posX, posY, ids[0], ids[1], correctChunkX, correctChunkY);
}

#[test]
fn Test_GetChunkId_3() {

    let posX: i32 = -32;
    let posY: i32 = 31;

    let correctChunkX: i32 = -1;
    let correctChunkY: i32 = 0;

    let ids: [i32; 2] = GetChunkId(posX, posY);

    assert_eq!(ids, [correctChunkX, correctChunkY], 
        "input Position ({}, {}) - Result Ids ({}, {}) - Expected ({}, {})", posX, posY, ids[0], ids[1], correctChunkX, correctChunkY);
}


#[test]
fn Test_GetChunkId_4() {

    let posX: i32 = -100;
    let posY: i32 = 68;

    let correctChunkX: i32 = -3;
    let correctChunkY: i32 = 2;

    let ids: [i32; 2] = GetChunkId(posX, posY);

    assert_eq!(ids, [correctChunkX, correctChunkY], 
        "input Position ({}, {}) - Result Ids ({}, {}) - Expected ({}, {})", posX, posY, ids[0], ids[1], correctChunkX, correctChunkY);
}