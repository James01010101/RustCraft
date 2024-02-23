#![feature(test)]

extern crate test;

use test::Bencher;
use std::collections::HashMap;
//use std::vec::Vec;

#[bench]
fn bench_hashmap_insert(b: &mut Bencher) {
    b.iter(|| {
        // create a temp vector with numbers from 0..1000 and then shuffle them
        let mut shuffled_vector: Vec<i32> = Vec::new();
        for i in 0..1000 {
            shuffled_vector.push(i);
        }

        // shuffle the vector
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        shuffled_vector.shuffle(&mut rng);


        // create and insert into hashmap
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..1000 {
            map.insert(shuffled_vector[i], i as i32);
        }
    });
}

#[bench]
fn bench_hashmap_insert_remove(b: &mut Bencher) {
    b.iter(|| {
        // create a temp vector with numbers from 0..1000 and then shuffle them
        let mut shuffled_vector: Vec<i32> = Vec::new();
        for i in 0..1000 {
            shuffled_vector.push(i);
        }

        // shuffle the vector
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        shuffled_vector.shuffle(&mut rng);


        // create and insert into hashmap
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..1000 {
            map.insert(shuffled_vector[i], i as i32);
        }


        // bench how long to remove
        for i in 0..1000 {
            map.remove(&shuffled_vector[i]);
        }         
    });
}

#[bench]
fn bench_hashmap_insert_get_remove(b: &mut Bencher) {
    b.iter(|| {
        // create a temp vector with numbers from 0..1000 and then shuffle them
        let mut shuffled_vector: Vec<i32> = Vec::new();
        for i in 0..1000 {
            shuffled_vector.push(i);
        }

        // shuffle the vector
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        shuffled_vector.shuffle(&mut rng);


        // create and insert into hashmap
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..1000 {
            map.insert(shuffled_vector[i], i as i32);
        }

        // get each item in the hashmap
        for i in 0..1000 {
            let _ = map.get(&shuffled_vector[i]);
        }

        // bench how long to remove
        for i in 0..1000 {
            map.remove(&shuffled_vector[i]);
        }        
    });
}

#[bench]
fn bench_vector_insert(b: &mut Bencher) {
    b.iter(|| {
        // create a temp vector with numbers from 0..1000 and then shuffle them
        let mut shuffled_vector: Vec<i32> = Vec::new();
        for i in 0..1000 {
            shuffled_vector.push(i);
        }

        // shuffle the vector
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        shuffled_vector.shuffle(&mut rng);

        // insert into the vector 
        let mut vector: Vec<i32> = Vec::new();
        for i in 0..1000 {
            vector.push(i);
        }

    });
}

#[bench]
fn bench_vector_insert_remove(b: &mut Bencher) {
    b.iter(|| {
        // create a temp vector with numbers from 0..1000 and then shuffle them
        let mut shuffled_vector: Vec<i32> = Vec::new();
        for i in 0..1000 {
            shuffled_vector.push(i);
        }

        // shuffle the vector
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        shuffled_vector.shuffle(&mut rng);


        // insert into the vector 
        let mut vector: Vec<i32> = Vec::new();
        for i in 0..1000 {
            vector.push(i);
        }

        // remove each number in the vector
        for i in 0..1000 {
            // here i want to remove from the vector the element i
            let index = vector.iter().position(|&x| x == shuffled_vector[i]).unwrap();
            vector.remove(index);
        }
    });
}

#[bench]
fn bench_vector_insert_get_remove(b: &mut Bencher) {
    b.iter(|| {
        // create a temp vector with numbers from 0..1000 and then shuffle them
        let mut shuffled_vector: Vec<i32> = Vec::new();
        for i in 0..1000 {
            shuffled_vector.push(i);
        }

        // shuffle the vector
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        shuffled_vector.shuffle(&mut rng);




        // insert into the vector 
        let mut vector: Vec<i32> = Vec::new();
        for i in 0..1000 {
            vector.push(i);
        }



        // get each number from the shuffled vector
        for i in 0..1000 {
            let index = vector.iter().position(|&x| x == shuffled_vector[i]).unwrap();
            let _ = vector.get(index);
        }



        // remove each number in the vector
        for i in 0..1000 {
            // here i want to remove from the vector the element i
            let index = vector.iter().position(|&x| x == shuffled_vector[i]).unwrap();
            vector.remove(index);
        }
    });
}