
use std::collections::HashSet;
use std::fs::File;

use gl::TEXTURE_COMPARE_FUNC;

use crate::Block::*;
use crate::Settings::*;




impl super::Chunk {
    
    
    // if this chunk has beenc created before then i create a Chunk obj, and fill it from wherever
    pub fn LoadChunk(&mut self, createdChunks: &mut HashSet<(i32, i32)>) {

        // TODO: #54 check that the chunk isnt loaded before creating another chunk of that id
        // create the temp chunk Vector, which creates all blocks
        let mut tempChunkVec: Vec<Vec<Vec<Block>>> = self.CreateTempChunkVector();

        // fill the temp vector with data
        // first check if the chunk has been created before if so load it
        if createdChunks.contains(&(self.chunkIDx, self.chunkIDz)) {
            // has been created before so load from file
            self.ReadChunkFromFile(&mut tempChunkVec);

        } else {
            // else create a new one
            self.GenerateChunk(&mut tempChunkVec);

            // add this chunk to created chunks
            createdChunks.insert((self.chunkIDx, self.chunkIDz));
        }

        // fill the chunkBlocks hashmap from the temp vector
        self.FillChunksHashMap(tempChunkVec);

    }


    

    // TODO: #19 Implement loading chunks from file
    pub fn ReadChunkFromFile(&mut self, tempChunkVec: &mut Vec<Vec<Vec<Block>>>) {
        // read the chunk from a file and fill the temp vector with the data
        println!("Reading Chunk from File: ({}, {}) -> NOT IMPLEMENTED YET", self.chunkIDx, self.chunkIDz);

    }


    // TODO: #52 convert the temp chunks vector into the hashmap
    pub fn FillChunksHashMap(&mut self, tempChunkVec: Vec<Vec<Vec<Block>>>) {

        // loop through the temp vector and fill the hashmap
        for x in 0..chunkSizeX {
            for y in 0..chunkSizeY {
                for z in 0..chunkSizeZ {
                    // if the block is not air then add it to the hashmap
                    if tempChunkVec[x][y][z].blockType != BlockType::Air {

                        self.chunkBlocks.insert(
                            (tempChunkVec[x][y][z].position.x, tempChunkVec[x][y][z].position.y, tempChunkVec[x][y][z].position.z), 
                            tempChunkVec[x][y][z]
                        );
                        self.aliveBlocks += 1;
                    }
                }
            }
        }
    }

}


// takes the world position and gives you the chunk id of the chunk that position is in
pub fn GetChunkId(posX: i32, posZ: i32) -> (i32, i32) {

    let chunkX: i32 = posX / chunkSizeX as i32;
    let chunkZ: i32 = posZ / chunkSizeZ as i32;

    return (chunkX, chunkZ);
}


// given a chunk x and z id and a block position relative to the origin of the chunk return the world coordinate of the block
pub fn GetWorldBlockPos(blockIDx: i32, blockIDz: i32, relBlockX: i32, relBlockY: i16, relBlockZ: i32) -> (i32, i16, i32) {
    let worldX: i32 = (blockIDx * chunkSizeX as i32) + relBlockX;
    let worldY: i16 = relBlockY;
    let worldZ: i32 = (blockIDz * chunkSizeZ as i32) + relBlockZ;

    return (worldX, worldY, worldZ);
}


// go from blocks world position to chunk relative position
pub fn GetRelativeBlockPos(worldX: i32, worldY: i16, worldZ: i32) -> (i32, i16, i32) {

    /* this does the same thing, as below, below is just obviously more efficient
    let mut chunkRelativeX: i32 = 0;
    let mut chunkRelativeZ: i32 = 0;
    let maxX: i32 = chunkSizeX as i32;
    let maxZ: i32 = chunkSizeZ as i32;

    if worldX >= 0 {
        chunkRelativeX = worldX % chunkSizeX as i32;
    } else {
        chunkRelativeX = (maxX - (worldX % maxX).abs()) % maxX;
    }

    if worldZ >= -(chunkRelativeZ as i32) {
        chunkRelativeZ = worldZ % chunkSizeZ as i32;
    } else {
        chunkRelativeZ = (maxZ - (worldZ % maxZ).abs()) % maxZ;
    }
    */
    
    let chunkRelativeX: i32 = worldX.rem_euclid(chunkSizeX as i32);
    let chunkRelativeZ: i32 = worldZ.rem_euclid(chunkSizeZ as i32);


    return (chunkRelativeX, worldY, chunkRelativeZ);
}