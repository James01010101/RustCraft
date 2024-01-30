

use crate::Objects::*;
use crate::Settings::*;

pub mod ChunkFunctions;
pub mod CreateChunks;


// This will store all of the blocks and objects within a chunk
pub struct Chunk {
    // Blocks, These are all of the mostly static blocks that will be in the world, that fit into a single block
    pub chunkBlocks: [[[Option<Block>; chunkSizeZ]; chunkSizeY]; chunkSizeX],


    // other objects like mobs that dont fit into a single block
    //pub chunkObjects: Vec

    // Chunk ID eg. floor(posx / chuckSizex) is the chunk
    pub chunkIDx: i32,
    pub chunkIDy: i32,

}


impl Chunk {
    pub fn new(idx: i32, idy: i32) -> Chunk {
        Chunk {
            chunkBlocks: [[[None; chunkSizeZ]; chunkSizeY]; chunkSizeX],
            chunkIDx: idx,
            chunkIDy: idy,
        }
    }
}