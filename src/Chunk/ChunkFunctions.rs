
use crate::Block::*;
use crate::Settings::*;




impl super::Chunk {
    
    // TODO: #19 Implement loading chunks from file
    // if this chunk has beenc created before then i create a Chunk obj, and fill it from wherever
    pub fn LoadChunk(&mut self) {

        // create the temp chunk Vector, which creates all blocks
        let mut tempChunkVec: Vec<Vec<Vec<Block>>> = self.CreateTempChunkVector();

        // fill the temp vector with data
        // first check if the chunk has been created before if so load it
        
        // else create a new one
        

        // fill the chunkBlocks hashmap from the temp vector
        self.FillChunksHashMap(tempChunkVec);

    }


    // TODO: #23 Implement saving chunks back to a file
    // Once a chunk has been loaded and is in play, and then goes out of range it is unloaded and saved back to a file at certain time periods
    pub fn SaveChunkToFile(&mut self) {
        // save the chunk to a file then free it
        println!("Saving Chunk to File: ({}, {})", self.chunkIDx, self.chunkIDz);
    }


    // TODO: #52 convert the temp chunks vector into the hashmap
    pub fn FillChunksHashMap(&mut self, tempChunkVec: Vec<Vec<Vec<Block>>>) {

    }

}


// takes the world position and gives you the chunk id of the chunk that position is in
pub fn GetChunkId(posX: i32, posZ: i32) -> [i32; 2]{

    let chunkX: i32 = posX / chunkSizeX as i32;
    let chunkZ: i32 = posZ / chunkSizeZ as i32;

    return [chunkX, chunkZ];
}