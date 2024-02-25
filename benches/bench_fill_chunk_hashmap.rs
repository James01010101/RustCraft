#![feature(test)]

extern crate test;
use std::collections::HashMap;

use test::Bencher;

use rust_craft::{
    optimisations::opt_fill_chunk_hashmap::*,
    block::*, 
    chunk::{chunk_functions::*, create_chunks::generate_chunk},
    types::*, 
};


// takes in the function as a param so i dont need to rewrite the whole test each time
fn bench_fill_chunk_hashmap<F>(b: &mut Bencher, fill_chunk_hashmap: F)
where
    F: Fn(&mut HashMap<(i32, i16, i32), Block>, &mut HashMap<(i32, i16, i32), InstanceData>, Vec<Vec<Vec<Block>>>, (usize, usize, usize)),
{
    let chunk_sizes: (usize, usize, usize) = (32, 256, 32);

    let mut temp_chunk_vector_global: Vec<Vec<Vec<Block>>> = create_temp_chunk_vector((0, 0), chunk_sizes);
    generate_chunk(&mut temp_chunk_vector_global, chunk_sizes);

    let chunk_blocks_global: HashMap<(i32, i16, i32), Block> = HashMap::new();
    let instances_to_render_global: HashMap<(i32, i16, i32), InstanceData> = HashMap::new();

    // set some as touching air
    for x in 0..chunk_sizes.0 {
        for z in 0..chunk_sizes.2 {
            temp_chunk_vector_global[x][(chunk_sizes.1 / 2) - 1][z].is_touching_air = true;
        }
    }

    b.iter(|| {
        let temp_chunk_vector = temp_chunk_vector_global.clone();
        let mut chunk_blocks = chunk_blocks_global.clone();
        let mut instances_to_render = instances_to_render_global.clone();

        fill_chunk_hashmap(&mut chunk_blocks, &mut instances_to_render, temp_chunk_vector, chunk_sizes);
    });
}

#[bench]
fn bench_fill_chunk_hashmap_old(b: &mut Bencher) {
    bench_fill_chunk_hashmap(b, fill_chunk_hashmap_old);
}

#[bench]
fn bench_fill_chunk_hashmap_new_1(b: &mut Bencher) {
    bench_fill_chunk_hashmap(b, fill_chunk_hashmap_new_1);
}

#[bench]
fn bench_fill_chunk_hashmap_new_2(b: &mut Bencher) {
    bench_fill_chunk_hashmap(b, fill_chunk_hashmap_new_2);
}

#[bench]
fn bench_fill_chunk_hashmap_new_3(b: &mut Bencher) {
    bench_fill_chunk_hashmap(b, fill_chunk_hashmap_new_3);
}

#[bench]
fn bench_fill_chunk_hashmap_new_4(b: &mut Bencher) {
    bench_fill_chunk_hashmap(b, fill_chunk_hashmap_new_4);
}

#[bench]
fn bench_fill_chunk_hashmap_new_5(b: &mut Bencher) {
    bench_fill_chunk_hashmap(b, fill_chunk_hashmap_new_5);
}

#[bench]
fn bench_fill_chunk_hashmap_new_6(b: &mut Bencher) {
    bench_fill_chunk_hashmap(b, fill_chunk_hashmap_new_6);
}









