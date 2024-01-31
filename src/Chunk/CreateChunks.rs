
/*
This file will be all of the rules for creating chunks and all the blocks within them
 */

use super::chunkSizeX;
use super::chunkSizeY;
use super::chunkSizeZ;
use super::Block;
use super::BlockType;

impl super::Chunk {

    /* 
    create a temporary 3d vector which will hold all of the blocks including air
    this will create all the blocks give them their position and will calculate their model matrix
    this will be returned and this is what the generate chunk will work on to calculate and modify the chunk as it is generated
    im using this temp vector so i can easily change the values of the blocks and also change blocks from air to block and vice versa
    then later ill convert from the vec to a hashmap which will only store blocks that arnt air to save space.
    */
    pub fn CreateTempChunkVector(&mut self) -> Vec<Vec<Vec<Block>>> {
        // initial xz values are defined by the chunks id, 
        let mut defaultBlock: Block = Block::new(BlockType::Air, 0, 0, 0);

        let mut tempChunkVec: Vec<Vec<Vec<Block>>> = Vec::with_capacity(chunkSizeX);
        let halfY: i16 = chunkSizeY as i16 / 2;

        let mut xPos: i32 = 0;
        let mut yPos: i16 = 0;
        let mut zPos: i32 = 0;

        println!("Printing XYZ Values: ");

        for x in 0..chunkSizeX as i32 {
            let mut temp2d: Vec<Vec<Block>> = Vec::with_capacity(chunkSizeY);

            for y in 0..chunkSizeY as i16 {
                let mut temp1d: Vec<Block> = Vec::with_capacity(chunkSizeZ);

                for z in 0..chunkSizeZ as i32 {
                    xPos = (self.chunkIDx * chunkSizeX as i32) + x as i32;
                    yPos = y - halfY;
                    zPos = (self.chunkIDz * chunkSizeZ as i32) + z as i32;

                    temp1d.push(Block::new(
                        BlockType::Air, 
                        xPos, 
                        yPos, 
                        zPos,
                        
                    ));

                    print!("({} {} {}) ", xPos, yPos, zPos);
                }

                temp2d.push(temp1d);
                println!("");
            }

            tempChunkVec.push(temp2d);
            println!("");
        }

        // now check vec
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut z: usize = 0;
        println!("Vector[{}][{}][{}] = {:?}", x, y, z, tempChunkVec[x][y][z].position);

        x = 1;
        y = 0;
        z = 0;
        println!("Vector[{}][{}][{}] = {:?}", x, y, z, tempChunkVec[x][y][z].position);

        x = 1;
        y = 0;
        z = 1;
        println!("Vector[{}][{}][{}] = {:?}", x, y, z, tempChunkVec[x][y][z].position);

        x = 1;
        y = 1;
        z = 1;
        println!("Vector[{}][{}][{}] = {:?}", x, y, z, tempChunkVec[x][y][z].position);

        x = 7;
        y = 1;
        z = 1;
        println!("Vector[{}][{}][{}] = {:?}", x, y, z, tempChunkVec[x][y][z].position);


        return tempChunkVec;

    }



    // TODO: #20 Implement Creating chunks if they havent been created before 
    /*
    if i havent created this chunk before then i create it, by creating a new chunk object and filling it with all the data it needs

    all of the world generation logic goes here

    i can potentially create a 3d vector so initially write everything to so it is easier to see what has been made and where it is and also easier to edit it by moving things around
    then i can just write it to the hashmap at the end, skipping air

    */
    pub fn GenerateChunk(&mut self, tempChunkVec: &mut Vec<Vec<Vec<Block>>>) {

    }

}