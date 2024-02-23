/*
This file will be all of the rules for creating chunks and all the blocks within them
 */
use crate::{block::*, block_type::*};

impl super::Chunk {
    

    
}

/*
if i havent created this chunk before then i create it, by creating a new chunk object and filling it with all the data it needs

all of the world generation logic goes here

i can potentially create a 3d vector so initially write everything to so it is easier to see what has been made and where it is and also easier to edit it by moving things around
then i can just write it to the hashmap at the end, skipping air

remember the origin of the chunk (index 000) is the front bottom right
*/
pub fn generate_chunk(temp_chunk_vec: &mut Vec<Vec<Vec<Block>>>, chunk_sizes: (usize, usize, usize), half_chunk_y: usize) {
    /*println!(
        "Generating Chunk: ({}, {})",
        self.chunk_id_x, self.chunk_id_z
    );*/

    // start with the bottom level being bedrock
    for x in 0..chunk_sizes.0 {
        for z in 0..chunk_sizes.2 {
            temp_chunk_vec[x][0][z].block_type = BlockType::Bedrock;
        }
    }

    // stone up until the halfway point - 3
    for x in 0..chunk_sizes.0 {
        for y in 1..chunk_sizes.1 - 3 {
            for z in 0..chunk_sizes.2 {
                temp_chunk_vec[x][y][z].block_type = BlockType::Stone;
            }
        }
    }

    // then 2 layers of dirt
    for x in 0..chunk_sizes.0 {
        for y in half_chunk_y - 3..half_chunk_y - 1 {
            for z in 0..chunk_sizes.2 {
                temp_chunk_vec[x][y][z].block_type = BlockType::Dirt;
            }
        }
    }

    // then one layer of grass
    for x in 0..chunk_sizes.0 {
        for y in half_chunk_y - 1..half_chunk_y {
            for z in 0..chunk_sizes.2 {
                temp_chunk_vec[x][y][z].block_type = BlockType::Grass;
            }
        }
    }
}
