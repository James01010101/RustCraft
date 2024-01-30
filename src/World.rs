
use crate::Chunk::*;
use crate::FileSystem::FileSystem;
use crate::Objects::*;

use std::collections::{HashMap, HashSet};





// this struct will hold all of the Chunks as well as arrays of mobs
pub struct World {

    // TODO: #25 Use a hashmap to store currently loaded chunks
    pub chunks: HashMap<[i32; 2], Chunk>,

    // stores all of the chunks that have been created before
    pub createdChunks: HashSet<(i32, i32)>,

    pub testBlocks: Vec<Block>,
}


impl World {
    pub fn new() -> World {

        // stores all alive chunks in this so they can be rendered and used
        let mut chunks: HashMap<[i32; 2], Chunk> = HashMap::new();

        // a table of all of the chunks that have been calculated before
        let mut createdChunks: HashSet<(i32, i32)> = HashSet::new();

        // a temp vec of blocks to put into the world without world gen
        let mut testBlocks: Vec<Block> = Vec::new();

        // create and return the world
        World { 
            chunks, 
            createdChunks,
            testBlocks, 
        }

    }

    pub fn AddTestBlocks(&mut self) {

        // Create some blocks to put into the testBlocks array
        self.testBlocks.push(Block::new(
            BlockType::Air, 
            1, 0, 0)
        );

        self.testBlocks.push(Block::new(
            BlockType::Dirt, 
            0, 0, 0)
        );

        self.testBlocks.push(Block::new(
            BlockType::Grass, 
            -1, 0, 0)
        );

        self.testBlocks.push(Block::new(
            BlockType::Sand, 
            -2, 0, 0)
        );

        self.testBlocks.push(Block::new(
            BlockType::Stone, 
            -3, 0, 0)
        );

        self.testBlocks.push(Block::new(
            BlockType::Cobblestone, 
            -4, 0, 0)
        );
    }



    


    // takes in the filesystem, loads the file where all of the chunks that have been created live and writes them to the hashmap
    pub fn LoadCreatedChunksFile(&mut self, myFileSystem: &mut FileSystem) {

        // open the file with this data and load it all into the hashmap
    }

}


// other important structs
// a basic 3 int position position struct to store the xyz position of a block
// its position is the bottom left back part of the square
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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