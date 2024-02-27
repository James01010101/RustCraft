use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::collections::HashMap;
use rust_craft::{
    optimisations::chunk_fill_chunk_hashmap::*,
    block::*, 
    chunk::{chunk_functions::*, create_chunks::generate_chunk},
    types::*, 
};



fn bench_fill_chunk_hashmap(c: &mut Criterion) {
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

    let mut group = c.benchmark_group("fill_chunk_hashmap");

    for (name, func) in [
        ("old", fill_chunk_hashmap_old as fn(&mut _, &mut _, _, _)),
        ("new_1", fill_chunk_hashmap_new_1 as fn(&mut _, &mut _, _, _)),
        ("new_2", fill_chunk_hashmap_new_2 as fn(&mut _, &mut _, _, _)),
        ("new_3", fill_chunk_hashmap_new_3 as fn(&mut _, &mut _, _, _)),
        ("new_4", fill_chunk_hashmap_new_4 as fn(&mut _, &mut _, _, _)),
        ("new_5", fill_chunk_hashmap_new_5 as fn(&mut _, &mut _, _, _)),
        ("new_6", fill_chunk_hashmap_new_6 as fn(&mut _, &mut _, _, _)),
        ("new_7", fill_chunk_hashmap_new_7 as fn(&mut _, &mut _, _, _)),
        // add more function versions here...
    ].iter() {
        group.bench_function(BenchmarkId::new("fill_chunk_hashmap", name), |b| {
            b.iter(|| {
                let temp_chunk_vector = temp_chunk_vector_global.clone();
                let mut chunk_blocks = chunk_blocks_global.clone();
                let mut instances_to_render = instances_to_render_global.clone();

                func(&mut chunk_blocks, &mut instances_to_render, temp_chunk_vector, chunk_sizes);
            });
        });
    }

    group.finish();
}

criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(1000); //.measurement_time(std::time::Duration::from_secs(10));
    targets = bench_fill_chunk_hashmap
}
criterion_main!(benches);