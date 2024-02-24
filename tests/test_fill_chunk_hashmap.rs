
extern crate rust_craft;

use std::collections::HashMap;
use rust_craft::{
    optimisations::opt_fill_chunk_hashmap::*,
    block::*, 
    block_type::*,
    chunk::{chunk_functions::*, create_chunks::generate_chunk},
    types::*, 
};


// here is the helper function which takes my function as input
fn test_fill_chunk_hashmap<F>(fill_chunk_hashmap: F)
where
    F: Fn(&mut HashMap<(i32, i16, i32), Block>, &mut HashMap<(i32, i16, i32), InstanceData>, Vec<Vec<Vec<Block>>>, (usize, usize, usize)),
{
    let chunk_sizes: (usize, usize, usize) = (32, 256, 32);

    let mut temp_chunk_vector_global: Vec<Vec<Vec<Block>>> = create_temp_chunk_vector((0, 0), chunk_sizes);
    generate_chunk(&mut temp_chunk_vector_global, chunk_sizes, chunk_sizes.1/2);

    let chunk_blocks_global: HashMap<(i32, i16, i32), Block> = HashMap::new();
    let instances_to_render_global: HashMap<(i32, i16, i32), InstanceData> = HashMap::new();

    // set some as touching air
    for x in 0..chunk_sizes.0 {
        for z in 0..chunk_sizes.2 {
            temp_chunk_vector_global[x][chunk_sizes.1 / 2][z].touching_air = true;
        }
    }

    // clone the temp chunk vector each time
    let temp_chunk_vector = temp_chunk_vector_global.clone();
    let mut chunk_blocks = chunk_blocks_global.clone();
    let mut instances_to_render = instances_to_render_global.clone();

    fill_chunk_hashmap(&mut chunk_blocks, &mut instances_to_render, temp_chunk_vector, chunk_sizes); // end iterations

    // check the hashmap now
    for x in 0..chunk_sizes.0 {
        for y in 0..chunk_sizes.1 {
            for z in 0..chunk_sizes.2 {
                if temp_chunk_vector_global[x][y][z].block_type != BlockType::Air {
                    assert_eq!(chunk_blocks.contains_key(&(
                        temp_chunk_vector_global[x][y][z].position.x, 
                        temp_chunk_vector_global[x][y][z].position.y, 
                        temp_chunk_vector_global[x][y][z].position.z)), true);
                } else {
                    assert_eq!(chunk_blocks.contains_key(&(
                        temp_chunk_vector_global[x][y][z].position.x, 
                        temp_chunk_vector_global[x][y][z].position.y, 
                        temp_chunk_vector_global[x][y][z].position.z)), false);
                }
            }
        }
    }
}




#[test]
fn test_fill_chunk_hashmap_old() {
    test_fill_chunk_hashmap(fill_chunk_hashmap_old);
}

#[test]
fn test_fill_chunk_hashmap_new_1() {
    test_fill_chunk_hashmap(fill_chunk_hashmap_new_1);
}

#[test]
fn test_fill_chunk_hashmap_new_2() {
    test_fill_chunk_hashmap(fill_chunk_hashmap_new_2);
}

#[test]
fn test_fill_chunk_hashmap_new_3() {
    test_fill_chunk_hashmap(fill_chunk_hashmap_new_3);
}

#[test]
fn test_fill_chunk_hashmap_new_4() {
    test_fill_chunk_hashmap(fill_chunk_hashmap_new_4);
}

#[test]
fn test_fill_chunk_hashmap_new_5() {
    test_fill_chunk_hashmap(fill_chunk_hashmap_new_5);
}

#[test]
fn test_fill_chunk_hashmap_new_6() {
    test_fill_chunk_hashmap(fill_chunk_hashmap_new_6);
}

#[test]
fn test_fill_chunk_hashmap_new_7() {
    test_fill_chunk_hashmap(fill_chunk_hashmap_new_7);
}

