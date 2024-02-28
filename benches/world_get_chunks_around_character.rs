use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rust_craft::{
    optimisations::world_get_chunks_around_character::*,
    world::*, 
    character::*,
};



fn bench_get_chunks_around_character(c: &mut Criterion) {

    let mut world = World::new("test_world".to_string(), 0, 5, (32, 256, 32));
    let mut character: Character = Character::new(0.1);

    // update the players chunk
    character.chunk_position = (0, 0);  

    
    let mut group = c.benchmark_group("get_chunks_around_character");

    for (name, func) in [
        ("old", get_chunks_around_character_old as fn(&mut _, & _,) -> Vec<(i32, i32)>),
        ("new_1", get_chunks_around_character_new_1 as fn(&mut _, & _,) -> Vec<(i32, i32)>),
        ("new_2", get_chunks_around_character_new_2 as fn(&mut _, & _,) -> Vec<(i32, i32)>),


    ].iter() {
        group.bench_function(BenchmarkId::new("get_chunks_around_character", name), |b| {
            b.iter(|| {
                let _: Vec<(i32, i32)> = func(&mut world, &character);
            });
        });
    }

    group.finish();
}

criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(1500).measurement_time(std::time::Duration::from_secs(10));
    targets = bench_get_chunks_around_character
}
criterion_main!(benches);