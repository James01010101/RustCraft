
extern crate rust_craft;
use rust_craft::{
    chunk::chunk_functions::create_temp_chunk_vector, 
    block::*,
    block_type::*,
};
    


// these all assume chunk sizes of (32 x 256 x 32)
fn test_create_temp_chunk_vector(chunk_ids: (i32, i32), chunk_sizes: (usize, usize, usize)) {

    let result: Vec<Vec<Vec<Block>>> = create_temp_chunk_vector(chunk_ids, chunk_sizes);

    assert_eq!(result.len(), chunk_sizes.0);
    assert_eq!(result[0].len(), chunk_sizes.1);
    assert_eq!(result[0][0].len(), chunk_sizes.2);

    // loop through the whole vector checking for the correct block types (should all be air)
    // and the correct position
    for x in 0..chunk_sizes.0 {
        for y in 0..chunk_sizes.1 {
            for z in 0..chunk_sizes.2 {
                assert_eq!(result[x][y][z].block_type, BlockType::Air);
                assert_eq!(result[x][y][z].position.x, (chunk_ids.0 * chunk_sizes.0 as i32) + x as i32);
                assert_eq!(result[x][y][z].position.y, y as i16 - (chunk_sizes.1 as i16 / 2));
                assert_eq!(result[x][y][z].position.z, (chunk_ids.1 * chunk_sizes.2 as i32) + z as i32);
            }
        }
    }
}

#[test]
fn test_create_temp_chunk_vector_1() {
    test_create_temp_chunk_vector((0, 0), (8, 16, 8));
}

#[test]
fn test_create_temp_chunk_vector_2() {
    test_create_temp_chunk_vector((1, 1), (8, 16, 8));
}

#[test]
fn test_create_temp_chunk_vector_3() {
    test_create_temp_chunk_vector((0, 0), (32, 256, 32));
}

#[test]
fn test_create_temp_chunk_vector_4() {
    test_create_temp_chunk_vector((-1, 1), (32, 256, 32));
}