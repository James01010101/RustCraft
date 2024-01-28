
use crate::Chunk::*;
use crate::Objects::*;

use std::collections::HashMap;


// this struct will hold all of the Chunks as well as arrays of mobs
pub struct World {

    // TODO: #25 Use a hashmap to store currently loaded chunks
    pub chunks: HashMap<i32, Chunk>,

    pub testBlocks: Vec<Block>,
}


// TODO: #22 Make the create world function
pub fn CreateWorld() -> World {

    let mut world: World = World { chunks: HashMap::new(), testBlocks: Vec::new() };


    // Create some blocks to put into the testBlocks array
    let block1: Block = Block::new(BlockType::Dirt, 0, 0, 0, 255.0, 0.0, 255.0);
    world.testBlocks.push(block1);

    let block2: Block = Block::new(BlockType::Grass, 2, 0, 0, 255.0, 0.0, 0.0);
    world.testBlocks.push(block2);

    let block3: Block = Block::new(BlockType::Sand, -1, 1, 3, 0.0, 255.0, 0.0);
    world.testBlocks.push(block3);

    let block4: Block = Block::new(BlockType::Grass, -1, 0, 1, 0.0, 0.0, 255.0);
    world.testBlocks.push(block4);

    let block5: Block = Block::new(BlockType::Grass, -1, 0, -1, 255.0, 255.0, 0.0);
    world.testBlocks.push(block5);

    
    return world;
}


// other important structs
// a basic 3 int position position struct to store the xyz position of a block
// its position is the bottom left back part of the square
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}   

// TODO: make sure all positions use f32 for now since all gpu calculations use f32
impl Position {
    pub fn new(x: i32, y: i16, z: i32) -> Position {
        Position {
            x,
            y,
            z,
        }
    }
}


// same but a float position
pub struct FPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}   

// TODO: make sure all positions use f32 for now since all gpu calculations use f32
impl FPosition {
    pub fn new(x: f32, y: f32, z: f32) -> FPosition {
        FPosition {
            x,
            y,
            z,
        }
    }
}