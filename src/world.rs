
use crate::{
    chunk::*,
    file_system::*,
};

use crate::renderer::*;

use std::{
    {collections::{HashMap, HashSet}, path::PathBuf},
    io::{self, BufRead},
    fs::File,
};


// this struct will hold all of the Chunks as well as arrays of mobs
pub struct World {

    // TODO: #25 Use a hashmap to store currently loaded chunks
    pub chunks: HashMap<(i32, i32), Chunk>,

    // stores all of the chunks that have been created before
    pub created_chunks: HashSet<(i32, i32)>,
}


impl World {
    pub fn new() -> World {

        // stores all alive chunks in this so they can be rendered and used
        let chunks: HashMap<(i32, i32), Chunk> = HashMap::new();

        // a table of all of the chunks that have been calculated before, Key: (chunkIDx, chunkIDy)
        // the order the hashset is printed changes every run
        let created_chunks: HashSet<(i32, i32)> = HashSet::new();


        // create and return the world
        World { 
            chunks, 
            created_chunks,
        }

    }

    
    pub fn add_test_chunks(&mut self, file_system: &mut FileSystem, renderer: &Renderer) {
        // create the new chunk
        
        /*
        for x in -1..=1 {
            for z in -1..=1 {
                let k: (i32, i32) = (x, z);
                let mut c: Chunk = Chunk::new(k.0, k.1, -1);

                // check the file doesnt exist already 
                if !self.chunks.contains_key(&k) {
                    c.LoadChunk(filesystem, &mut self.createdChunks);
                    self.chunks.insert(k, c);
                } else {
                    println!("Chunk ({}, {}) already exists", k.0, k.1);
                }
            }
        }
        */
        

        // create the chunks id key
        let k: (i32, i32) = (0, 0);
        
        // if the loaded chunks doesnt contain this chunk ill load it
        if !self.chunks.contains_key(&k) {

            let mut c: Chunk = Chunk::new(k.0, k.1, -1, &renderer);
            c.load_chunk(file_system, &mut self.created_chunks, &renderer);

            self.chunks.insert(k, c);

            

        } else {
            // keep this debug so i know how many times it trys to reinsert the same chunk
            println!("Chunk ({}, {}) already exists", k.0, k.1);
        }

    }



    // takes in the filesystem, loads the file where all of the chunks that have been created live and writes them to the hashmap
    pub fn load_created_chunks_file(&mut self, my_file_system: &mut FileSystem) {

        // get the path to the ChunksCreated.txt file
        let mut chunks_created_path: PathBuf = my_file_system.my_world_directory.clone();
        chunks_created_path.push("ChunksCreated.txt");

        let chunks_created_file: File = File::open(chunks_created_path).unwrap();

        let reader: io::BufReader<File> = io::BufReader::new(chunks_created_file);

        let mut lines = reader.lines();


        // read the first line to get the total created chunks
        let line1: String = lines.next()
            .expect("Failed to get next line in ChunksCreated.txt, as it is at EOF")
            .unwrap();

        let total_created_chunks: i32 = line1.split_whitespace()
            .last()
            .expect("Failed to get last element in split whitespace string (ChunksCreated.txt)")
            .parse::<i32>()
            .unwrap();
        
        // skip the next 2 lines
        lines.next();
        lines.next();

        // now read the next totalCreatedChunks lines and insert them into the hashmap
        let mut line: String;
        let mut x: i32 = 0;
        let mut z: i32 = 0;
        for _ in 0..total_created_chunks {
            line = lines.next()
                .expect("Failed to get next line in ChunksCreated.txt, as it is at EOF")
                .unwrap();

            let mut split_line = line.split_whitespace();
            x = split_line.next()
            .expect("Failed to get next element of split whitespace line while reading ChunksCreated.txt")
            .parse::<i32>()
            .unwrap();


            z = split_line.next()
            .expect("Failed to get next element of split whitespace line while reading ChunksCreated.txt")
            .parse::<i32>()
            .unwrap();



            // insert these into the hashset and check if it is a dupe
            if !self.created_chunks.insert((x, z)) {
                // if insert returns false then it was already in the hashmap
                eprintln!("Duplicate key found when reading chunk ids from ChunksCreated.txt: ({}, {})", x, z);
            }
        }
    }

    // TODO: #63 set world block from world coords

    // TODO: #64 Get world block from world coords

    // TODO: #65 Place block function

    // TODO: #66 Break block function
}