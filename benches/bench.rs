#![feature(test)]

extern crate test;

use test::Bencher;
use std::collections::HashMap;
use std::vec::Vec;

#[bench]
fn bench_hashmap_insert(b: &mut Bencher) {
    b.iter(|| {
        // see how long to insert elements into this
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0..1000 {
            map.insert(i, i);
        }
    });
}

#[bench]
fn bench_hashmap_remove(b: &mut Bencher) {
    b.iter(|| {
        // create and insert into hashmap
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..1000 {
            map.insert(i, i);
        }

        // bench how long to remove
        for i in 0..1000 {
            map.remove(&i);
        }        
    });
}

#[bench]
fn bench_vector_insert(b: &mut Bencher) {
    b.iter(|| {
        // bench how long to insert

        let mut vector: Vec<i32> = Vec::new();

        for i in 0..1000 {
            vector.push(i);
        }
    });
}

#[bench]
fn bench_vector_remove(b: &mut Bencher) {
    b.iter(|| {
        // create the vector
        let mut vector: Vec<i32> = Vec::new();
        for i in 0..1000 {
            vector.push(i);
        }

        // bench how long to remove
        for i in 0..1000 {
            // here i want to remove from the vector the element i
            let index = vector.iter().position(|&x| x == i).unwrap();
            vector.remove(index);
        }
    });
}