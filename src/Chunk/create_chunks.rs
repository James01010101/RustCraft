
/*
This file will be all of the rules for creating chunks and all the blocks within them
 */
use crate::{
    settings::*,
    block_type::*,
    block::*,
};

impl super::Chunk {
    /* 
    create a temporary 3d vector which will hold all of the blocks including air
    this will create all the blocks give them their position and will calculate their model matrix
    this will be returned and this is what the generate chunk will work on to calculate and modify the chunk as it is generated
    im using this temp vector so i can easily change the values of the blocks and also change blocks from air to block and vice versa
    then later ill convert from the vec to a hashmap which will only store blocks that arnt air to save space.
    */
    pub fn create_temp_chunk_vector(&mut self) -> Vec<Vec<Vec<Block>>> {
        // initial xz values are defined by the chunks id, 
        let mut temp_chunk_vec: Vec<Vec<Vec<Block>>> = Vec::with_capacity(CHUNK_SIZE_X);

        let mut pos_x: i32;
        let mut pos_y: i16;
        let mut pos_z: i32;

        for x in 0..CHUNK_SIZE_X as i32 {
            let mut temp2d: Vec<Vec<Block>> = Vec::with_capacity(CHUNK_SIZE_Y);

            for y in 0..CHUNK_SIZE_Y as i16 {
                let mut temp1d: Vec<Block> = Vec::with_capacity(CHUNK_SIZE_Z);

                for z in 0..CHUNK_SIZE_Z as i32 {
                    pos_x = (self.chunk_id_x * CHUNK_SIZE_X as i32) + x as i32;
                    pos_y = y - HALF_CHUNK_Y as i16;
                    pos_z = (self.chunk_id_z * CHUNK_SIZE_Z as i32) + z as i32;

                    temp1d.push(Block::new(
                        BlockType::Air, 
                        pos_x, 
                        pos_y, 
                        pos_z,
                        
                    ));
                }

                temp2d.push(temp1d);
            }

            temp_chunk_vec.push(temp2d);
        }

        return temp_chunk_vec;
    }


    /*
    if i havent created this chunk before then i create it, by creating a new chunk object and filling it with all the data it needs

    all of the world generation logic goes here

    i can potentially create a 3d vector so initially write everything to so it is easier to see what has been made and where it is and also easier to edit it by moving things around
    then i can just write it to the hashmap at the end, skipping air
    */
    pub fn generate_chunk(&mut self, temp_chunk_vec: &mut Vec<Vec<Vec<Block>>>) {
        
        println!("Generating Chunk: ({}, {})", self.chunk_id_x, self.chunk_id_z);

        // start with the bottom level being bedrock
        for x in 0..CHUNK_SIZE_X {
            for z in 0..CHUNK_SIZE_Z {
                temp_chunk_vec[x][0][z].block_type = BlockType::Bedrock;
            }
        }


        // stone up until the halfway point - 3
        for x in 0..CHUNK_SIZE_X {
            for y in 1..HALF_CHUNK_Y - 3 {
                for z in 0..CHUNK_SIZE_Z {
                    temp_chunk_vec[x][y][z].block_type = BlockType::Stone;
                }
            }
        }

        // then 2 layers of dirt
        for x in 0..CHUNK_SIZE_X {
            for y in HALF_CHUNK_Y - 3..HALF_CHUNK_Y - 1 {
                for z in 0..CHUNK_SIZE_Z {
                    temp_chunk_vec[x][y][z].block_type = BlockType::Dirt;
                }
            }
        }


        // then one layer of grass
        for x in 0..CHUNK_SIZE_X {
            for y in HALF_CHUNK_Y - 1..HALF_CHUNK_Y {
                for z in 0..CHUNK_SIZE_Z {
                    temp_chunk_vec[x][y][z].block_type = BlockType::Grass;
                }
            }
        }
    }
}