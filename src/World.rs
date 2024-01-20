
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
    let block1: Block = CreateNewBlock(BlockType::Dirt, false, 0, 0, 0);
    world.testBlocks.push(block1);

    let block2: Block = CreateNewBlock(BlockType::Grass, false, 2, 0, 0);
    world.testBlocks.push(block2);

    return world;
}