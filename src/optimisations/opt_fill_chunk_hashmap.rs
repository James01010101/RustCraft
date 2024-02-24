
use std::collections::HashMap;

use crate::{
    block::*, 
    block_type::*, 
    types::*, 
};


// origional function
pub fn fill_chunk_hashmap_old( 
    chunk_blocks: &mut HashMap<(i32, i16, i32), Block>, 
    instances_to_render: &mut HashMap<(i32, i16, i32), InstanceData>,
    temp_chunk_vec: Vec<Vec<Vec<Block>>>, 
    chunk_sizes: (usize, usize, usize)
) {
    // loop through the temp vector and fill the hashmap
    for x in 0..chunk_sizes.0 {
        for y in 0..chunk_sizes.1 {
            for z in 0..chunk_sizes.2 {
                // if the block is not air then add it to the hashmap
                if temp_chunk_vec[x][y][z].block_type != BlockType::Air {
                    chunk_blocks.insert(
                        (
                            temp_chunk_vec[x][y][z].position.x,
                            temp_chunk_vec[x][y][z].position.y,
                            temp_chunk_vec[x][y][z].position.z,
                        ),
                        temp_chunk_vec[x][y][z],
                    );

                    // also if it is touching air then add it to the instances to render hashmap
                    if temp_chunk_vec[x][y][z].touching_air {
                        instances_to_render.insert(
                            (
                                temp_chunk_vec[x][y][z].position.x,
                                temp_chunk_vec[x][y][z].position.y,
                                temp_chunk_vec[x][y][z].position.z,
                            ),
                            InstanceData {
                                model_matrix: temp_chunk_vec[x][y][z].model_matrix.clone(),
                                colour: temp_chunk_vec[x][y][z].block_type.block_colour(),
                            },
                        );
                    }
                }
            }
        }
    }
}


// reserve space for the number of elements i need to add to the chunk blocks array
// i dont know the exact number of elements that will be in chunk blocks so ill over estimate here
pub fn fill_chunk_hashmap_new_1( 
    chunk_blocks: &mut HashMap<(i32, i16, i32), Block>, 
    instances_to_render: &mut HashMap<(i32, i16, i32), InstanceData>,
    temp_chunk_vec: Vec<Vec<Vec<Block>>>, 
    chunk_sizes: (usize, usize, usize)
) {

    // since i know how many elements i can reserve this amount so it only reallocs once (most of the time)
    chunk_blocks.reserve(chunk_sizes.0 * chunk_sizes.1 * chunk_sizes.2);

    // loop through the temp vector and fill the hashmap
    for x in 0..chunk_sizes.0 {
        for y in 0..chunk_sizes.1 {
            for z in 0..chunk_sizes.2 {
                // if the block is not air then add it to the hashmap
                if temp_chunk_vec[x][y][z].block_type != BlockType::Air {

                    // and if it is touching air then add it to the instances to render hashmap
                    if temp_chunk_vec[x][y][z].touching_air {
                        instances_to_render.insert(
                            (
                                temp_chunk_vec[x][y][z].position.x,
                                temp_chunk_vec[x][y][z].position.y,
                                temp_chunk_vec[x][y][z].position.z,
                            ),
                            InstanceData {
                                model_matrix: temp_chunk_vec[x][y][z].model_matrix.clone(),
                                colour: temp_chunk_vec[x][y][z].block_type.block_colour(),
                            },
                        );
                    }

                    // always insert the block into the chunk_blocks hashmap
                    chunk_blocks.insert(
                        (
                            temp_chunk_vec[x][y][z].position.x,
                            temp_chunk_vec[x][y][z].position.y,
                            temp_chunk_vec[x][y][z].position.z,
                        ),
                        temp_chunk_vec[x][y][z],
                    );
                }
            }
        }
    }

    // so i dont waste any memory but dont do many reallocs during runtime. just one at the start to reserve the space
    // and one at the end to cleanup unneeded memory
    chunk_blocks.shrink_to_fit();
}


// just use cachine of vectors
pub fn fill_chunk_hashmap_new_2( 
    chunk_blocks: &mut HashMap<(i32, i16, i32), Block>, 
    instances_to_render: &mut HashMap<(i32, i16, i32), InstanceData>,
    temp_chunk_vec: Vec<Vec<Vec<Block>>>, 
    chunk_sizes: (usize, usize, usize)
) {

    // since i know how many elements i can reserve this amount so it only reallocs once (most of the time)
    //chunk_blocks.reserve(chunk_sizes.0 * chunk_sizes.1 * chunk_sizes.2);

    // since ill have atleast 1 block showing ins xz for the ground at minimum i can alloc that much 
    // ill * by 2 so this should be more than i need in most cases and ill shrink after
    // and it can realloc more later if it needs
    //instances_to_render.reserve(chunk_sizes.0 * chunk_sizes.2 * 2);

    // cache each vector so i dont have to do 3 indexes each time
    let mut cached_vec_x: &Vec<Vec<Block>>;
    let mut cached_vec_y: &Vec<Block>;
    let mut cached_block: &Block;
    let mut cached_position: &Position;
    let mut cached_key: (i32, i16, i32);

    // loop through the temp vector and fill the hashmap
    for x in 0..chunk_sizes.0 {
        cached_vec_x = &temp_chunk_vec[x];

        for y in 0..chunk_sizes.1 {
            cached_vec_y = &cached_vec_x[y];

            for z in 0..chunk_sizes.2 {
                cached_block = &cached_vec_y[z];

                // if the block is not air then add it to the hashmap
                if cached_block.block_type != BlockType::Air {
                    cached_position = &cached_block.position;
                    cached_key = (cached_position.x, cached_position.y, cached_position.z);

                    // and if it is touching air then add it to the instances to render hashmap
                    if cached_block.touching_air {
                        instances_to_render.insert(
                            cached_key,
                            InstanceData {
                                model_matrix: cached_block.model_matrix.clone(),
                                colour: cached_block.block_type.block_colour(),
                            },
                        );
                    }

                    // always insert the block into the chunk_blocks hashmap
                    chunk_blocks.insert(
                        cached_key,
                        *cached_block,
                    );
                }
            }
        }
    }
    //chunk_blocks.shrink_to_fit();
    //instances_to_render.shrink_to_fit();
}


// instead of going straight into a hashmap, push into a vector first. 
// then at the end of all the loops i know exactly how mush memory the hashmaps need 
// and i can reserve that amount and then hash them all into it
pub fn fill_chunk_hashmap_new_3( 
    chunk_blocks: &mut HashMap<(i32, i16, i32), Block>, 
    instances_to_render: &mut HashMap<(i32, i16, i32), InstanceData>,
    temp_chunk_vec: Vec<Vec<Vec<Block>>>, 
    chunk_sizes: (usize, usize, usize)
) {

    // cache each vector so i dont have to do 3 indexes each time
    let mut cached_vec_x: &Vec<Vec<Block>>;
    let mut cached_vec_y: &Vec<Block>;
    let mut cached_block: &Block;
    let mut cached_position: &Position;
    let mut cached_key: (i32, i16, i32);

    let max_elements: usize = chunk_sizes.0 * chunk_sizes.1 * chunk_sizes.2;

    let mut blocks_to_insert: Vec<Block> = Vec::with_capacity(max_elements);
    let mut blocks_keys_to_insert: Vec<(i32, i16, i32)> = Vec::with_capacity(max_elements);
    let mut instances_to_insert: Vec<InstanceData> = Vec::with_capacity(max_elements);
    let mut instances_keys_to_insert: Vec<(i32, i16, i32)> = Vec::with_capacity(max_elements);

    // loop through the temp vector and fill the hashmap
    for x in 0..chunk_sizes.0 {
        cached_vec_x = &temp_chunk_vec[x];

        for y in 0..chunk_sizes.1 {
            cached_vec_y = &cached_vec_x[y];

            for z in 0..chunk_sizes.2 {
                cached_block = &cached_vec_y[z];

                // if the block is not air then add it to the hashmap
                if cached_block.block_type != BlockType::Air {
                    cached_position = &cached_block.position;
                    cached_key = (cached_position.x, cached_position.y, cached_position.z);

                    // and if it is touching air then add it to the instances to render hashmap
                    if cached_block.touching_air {
                        instances_to_insert.push(InstanceData {
                            model_matrix: cached_block.model_matrix.clone(),
                            colour: cached_block.block_type.block_colour(),
                        });
                        instances_keys_to_insert.push(cached_key);
                    }

                    blocks_to_insert.push(*cached_block);
                    blocks_keys_to_insert.push(cached_key);
                }
            }
        }
    }

    // now reserve exactly the amount of memory needed and fill the hashmaps
    chunk_blocks.reserve(blocks_to_insert.len());
    for i in 0..blocks_to_insert.len() {
        chunk_blocks.insert(blocks_keys_to_insert[i], blocks_to_insert[i]);
    }

    instances_to_render.reserve(instances_to_insert.len());
    for i in 0..instances_to_insert.len() {
        instances_to_render.insert(instances_keys_to_insert[i], instances_to_insert[i]);
    }
}


// condense the 2 vectors into 1 for inserting
pub fn fill_chunk_hashmap_new_4( 
    chunk_blocks: &mut HashMap<(i32, i16, i32), Block>, 
    instances_to_render: &mut HashMap<(i32, i16, i32), InstanceData>,
    temp_chunk_vec: Vec<Vec<Vec<Block>>>, 
    chunk_sizes: (usize, usize, usize)
) {

    // cache each vector so i dont have to do 3 indexes each time
    let mut cached_vec_x: &Vec<Vec<Block>>;
    let mut cached_vec_y: &Vec<Block>;
    let mut cached_block: &Block;
    let mut cached_position: &Position;
    let mut cached_key: (i32, i16, i32);

    let max_elements: usize = chunk_sizes.0 * chunk_sizes.1 * chunk_sizes.2;

    let mut blocks_to_insert: Vec<((i32, i16, i32), Block)> = Vec::with_capacity(max_elements);
    let mut instances_to_insert: Vec<((i32, i16, i32), InstanceData)> = Vec::with_capacity(max_elements);

    // loop through the temp vector and fill the hashmap
    for x in 0..chunk_sizes.0 {
        cached_vec_x = &temp_chunk_vec[x];

        for y in 0..chunk_sizes.1 {
            cached_vec_y = &cached_vec_x[y];

            for z in 0..chunk_sizes.2 {
                cached_block = &cached_vec_y[z];

                // if the block is not air then add it to the hashmap
                if cached_block.block_type != BlockType::Air {
                    cached_position = &cached_block.position;
                    cached_key = (cached_position.x, cached_position.y, cached_position.z);

                    // and if it is touching air then add it to the instances to render hashmap
                    if cached_block.touching_air {
                        instances_to_insert.push((cached_key,
                            InstanceData {
                                model_matrix: cached_block.model_matrix.clone(),
                                colour: cached_block.block_type.block_colour(),
                            }
                        ));
                    }

                    blocks_to_insert.push((cached_key, *cached_block));
                }
            }
        }
    }

    // now reserve exactly the amount of memory needed and fill the hashmaps
    chunk_blocks.reserve(blocks_to_insert.len());
    for i in 0..blocks_to_insert.len() {
        chunk_blocks.insert(blocks_to_insert[i].0, blocks_to_insert[i].1);
    }

    instances_to_render.reserve(instances_to_insert.len());
    for i in 0..instances_to_insert.len() {
        instances_to_render.insert(instances_to_insert[i].0, instances_to_insert[i].1);
    }
}


// use iter instead of loops over a range as they are slightly faster
pub fn fill_chunk_hashmap_new_5( 
    chunk_blocks: &mut HashMap<(i32, i16, i32), Block>, 
    instances_to_render: &mut HashMap<(i32, i16, i32), InstanceData>,
    temp_chunk_vec: Vec<Vec<Vec<Block>>>, 
    chunk_sizes: (usize, usize, usize)
) {

    // cache each vector so i dont have to do 3 indexes each time
    let mut cached_position: &Position;
    let mut cached_key: (i32, i16, i32);

    let max_elements: usize = chunk_sizes.0 * chunk_sizes.1 * chunk_sizes.2;

    let mut blocks_to_insert: Vec<((i32, i16, i32), Block)> = Vec::with_capacity(max_elements);
    let mut instances_to_insert: Vec<((i32, i16, i32), InstanceData)> = Vec::with_capacity(max_elements);

    // loop through the temp vector and fill the hashmap
    for vec_x in temp_chunk_vec.iter() {
        for vec_y in vec_x.iter() {
            for block in vec_y.iter() {

                // if the block is not air then add it to the hashmap
                if block.block_type != BlockType::Air {
                    cached_position = &block.position;
                    cached_key = (cached_position.x, cached_position.y, cached_position.z);

                    // and if it is touching air then add it to the instances to render hashmap
                    if block.touching_air {
                        instances_to_insert.push((cached_key,
                            InstanceData {
                                model_matrix: block.model_matrix.clone(),
                                colour: block.block_type.block_colour(),
                            }
                        ));
                    }

                    blocks_to_insert.push((cached_key, *block));
                }
            }
        }
    }

    // now reserve exactly the amount of memory needed and fill the hashmaps
    chunk_blocks.reserve(blocks_to_insert.len());
    chunk_blocks.extend(blocks_to_insert.into_iter());

    instances_to_render.reserve(instances_to_insert.len());
    instances_to_render.extend(instances_to_insert.into_iter());
}


// 
pub fn fill_chunk_hashmap_new_6( 
    chunk_blocks: &mut HashMap<(i32, i16, i32), Block>, 
    instances_to_render: &mut HashMap<(i32, i16, i32), InstanceData>,
    temp_chunk_vec: Vec<Vec<Vec<Block>>>, 
    chunk_sizes: (usize, usize, usize)
) {

    // cache each vector so i dont have to do 3 indexes each time
    let mut cached_key: (i32, i16, i32);

    let max_elements: usize = chunk_sizes.0 * chunk_sizes.1 * chunk_sizes.2;
    let mut blocks_to_insert: Vec<((i32, i16, i32), Block)> = Vec::with_capacity(max_elements);
    let mut instances_to_insert: Vec<((i32, i16, i32), InstanceData)> = Vec::with_capacity(max_elements);

    // loop through the temp vector and fill the hashmap
    for vec_x in temp_chunk_vec.iter() {
        for vec_y in vec_x.iter() {
            for block in vec_y.iter() {

                // if the block is not air then add it to the hashmap
                if block.block_type != BlockType::Air {
                    cached_key = (block.position.x, block.position.y, block.position.z);
                    blocks_to_insert.push((cached_key, *block));

                    // and if it is touching air then add it to the instances to render hashmap
                    if block.touching_air {
                        instances_to_insert.push((cached_key,
                            InstanceData {
                                model_matrix: block.model_matrix.clone(),
                                colour: block.block_type.block_colour(),
                            }
                        ));
                    }
                }
            }
        }
    }

    // now reserve exactly the amount of memory needed and fill the hashmaps
    chunk_blocks.reserve(blocks_to_insert.len());
    chunk_blocks.extend(blocks_to_insert.into_iter());

    instances_to_render.reserve(instances_to_insert.len());
    instances_to_render.extend(instances_to_insert.into_iter());
}