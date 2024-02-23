#![feature(test)]

extern crate test;
use std::collections::HashMap;

use test::Bencher;

use rust_craft::{
    block::*, 
    block_type::*, 
    chunk::{self, chunk_functions::*, create_chunks::generate_chunk},
    types::*, 
};

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


#[bench]
fn bench_fill_chunk_hashmap_old(b: &mut Bencher) {
    let chunk_sizes: (usize, usize, usize) = (8, 16, 8);

    let mut temp_chunk_vector_global: Vec<Vec<Vec<Block>>> = create_temp_chunk_vector((0, 0), (8, 16, 8));
    generate_chunk(&mut temp_chunk_vector_global, chunk_sizes, chunk_sizes.1/2);

    let chunk_blocks_global: HashMap<(i32, i16, i32), Block> = HashMap::new();
    let instances_to_render_global: HashMap<(i32, i16, i32), InstanceData> = HashMap::new();


    // set some as touching air
    for x in 0..chunk_sizes.0 {
        for z in 0..chunk_sizes.2 {
            temp_chunk_vector_global[x][chunk_sizes.1 / 2][z].touching_air = true;
        }
    }



    // testing
    b.iter(|| {

        // clone the temp chunk vector each time
        let temp_chunk_vector = temp_chunk_vector_global.clone();
        let mut chunk_blocks = chunk_blocks_global.clone();
        let mut instances_to_render = instances_to_render_global.clone();


        fill_chunk_hashmap_old(&mut chunk_blocks, &mut instances_to_render, temp_chunk_vector, chunk_sizes); // end iterations
    });
}




pub fn fill_chunk_hashmap_new( 
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
}


#[bench]
fn bench_fill_chunk_hashmap_new(b: &mut Bencher) {
    let chunk_sizes: (usize, usize, usize) = (8, 16, 8);

    let mut temp_chunk_vector_global: Vec<Vec<Vec<Block>>> = create_temp_chunk_vector((0, 0), (8, 16, 8));
    generate_chunk(&mut temp_chunk_vector_global, chunk_sizes, chunk_sizes.1/2);

    let chunk_blocks_global: HashMap<(i32, i16, i32), Block> = HashMap::new();
    let instances_to_render_global: HashMap<(i32, i16, i32), InstanceData> = HashMap::new();

    // testing
    b.iter(|| {

        // clone the temp chunk vector each time
        let temp_chunk_vector = temp_chunk_vector_global.clone();
        let mut chunk_blocks = chunk_blocks_global.clone();
        let mut instances_to_render = instances_to_render_global.clone();


        fill_chunk_hashmap_new(&mut chunk_blocks, &mut instances_to_render, temp_chunk_vector, chunk_sizes); // end iterations
    });
}