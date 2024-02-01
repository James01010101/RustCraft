

use std::mem;

use std::collections::HashMap;

use crate::Block::*;
use crate::Settings::*;
use crate::World::*;

pub mod ChunkFunctions;
pub mod CreateChunks;


// This will store all of the blocks and objects within a chunk
pub struct Chunk {
    // Blocks, These are all of the mostly static blocks that will be in the world
    // using a tuple as key so i can access it without needing to create a position struct just to get it
    pub chunkBlocks: HashMap<(i32, i16, i32), Block>,

    // other objects like mobs that dont fit into a single block
    //pub chunkObjects: Vec

    // Chunk ID eg. floor(posx / chuckSizex) is the chunk
    pub chunkIDx: i32,
    pub chunkIDz: i32,

    // total number of non air blocks in the chunk
    pub aliveBlocks: i32,

}


impl Chunk {
    pub fn new(idx: i32, idz: i32, numBlocks: i32) -> Chunk {

        // if a numBlocks was passed in ill allocate the hashmap of that size
        if numBlocks != -1 {
            Chunk {
                chunkBlocks: HashMap::with_capacity(numBlocks as usize),
                chunkIDx: idx,
                chunkIDz: idz,
                aliveBlocks: numBlocks,
            }
        } else { // if the number of blocks needed is unknown ill just let it do its thing
            Chunk {
                chunkBlocks: HashMap::new(),
                chunkIDx: idx,
                chunkIDz: idz,
                aliveBlocks: 0,
            }
        }
    }
}
