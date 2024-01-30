use crate::Objects::{Block};
use crate::Settings::*;


// This will store all of the blocks and objects within a chunk
pub struct Chunk {
    // Blocks, These are all of the mostly static blocks that will be in the world, that fit into a single block
    pub chunkBlocks: [[[Option<Block>; chunkSizeZ]; chunkSizeY]; chunkSizeX],


    // other objects like mobs that dont fit into a single block
    //pub chunkObjects: Vec

    // Chunk ID eg. floor(posx / chuckSizex) is the chunk
    pub chunkIDX: i32,
    pub chunkIDY: i32,

}



impl Chunk {
    pub fn new(IDx: i32, IDy: i32) -> Chunk {
        Chunk {
            chunkBlocks: [[[None; chunkSizeZ]; chunkSizeY]; chunkSizeX],
            chunkIDX: IDx,
            chunkIDY: IDy,
        }
    }

    
    // TODO: #19 Implement loading chunks from file
    // if this chunk has beenc created before then i create a Chunk obj, and fill it from wherever
    pub fn LoadChunk(loadX: i32, loadY: i32, unloadX: i32, unloadY: i32) {
        // params are of the x and y id of the chunk i want to load and the chunk i want to unload 
    
        // first check if the chunk has been created before

    }


    // TODO: #20 Implement Creating chunks if they havent been created before 
    // if i havent created this chunk before then i create it, by creating a new chunk object and filling it with all the data it needs
    pub fn CreateChunk() {

    }




    // TODO: #23 Implement saving chunks back to a file
    // Once a chunk has been loaded and is in play, and then goes out of range it is unloaded and saved back to a file at certain time periods
    pub fn SaveChunk() {

    }
}


// takes the world position and gives you the chunk id of the chunk that position is in
pub fn GetChunkId(posX: i32, posY: i32) -> [i32; 2]{

    let chunkX: i32 = posX / chunkSizeX as i32;
    let chunkY: i32 = posY / chunkSizeY as i32;

    return [chunkX, chunkY];
}