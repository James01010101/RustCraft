use crate::Objects::{Block};
use crate::Settings::*;

// this is a tree which will store my spacial partitioned data for all of the objects.
// the purpose of this is to pass to the gpu so that when it is calculating rays it only needs to check around its current location
// and it doesnt need to check every object in the world
// TODO: #21 implement the spatial partitioning struct and calculations
struct PartitionedObjects {

}


// This will store all of the blocks and objects within a chunk
pub struct Chunk {
    // Blocks, These are all of the mostly static blocks that will be in the world, that fit into a single block
    pub chunkBlocks: [[[Option<Block>; chunkSizeZ]; chunkSizeY]; chunkSizeX],


    // other objects like mobs that dont fit into a single block
    //pub chunkObjects: Vec

    // Chunk ID (eg. posx / chuckSizex) is the chunk
    pub chunkIDX: i32,
    pub chunkIDY: i32,

}


// TODO: #20 Implement Creating chunks if they havent been created before 
// if i havent created this chunk before then i create it, by creating a new chunk object and filling it with all the data it needs
pub fn CreateChunk() {

}


// TODO: #19 Implement loading chunks from file
// if this chunk has beenc created before then i create a Chunk obj, and fill it from wherever
pub fn LoadChunk() {

}

// TODO: #23 Implement saving chunks back to a file
// Once a chunk has been loaded and is in play, and then goes out of range it is unloaded and saved back to a file at certain time periods
pub fn SaveChunk() {

}