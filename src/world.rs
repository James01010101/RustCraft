use crate::{character::*, chunk::*, file_system::*};

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use std::collections::VecDeque;

// this struct will hold all of the Chunks as well as arrays of mobs
pub struct World {
    // Use a hashmap to store currently loaded chunks
    pub chunks: Arc<Mutex<HashMap<(i32, i32), Chunk>>>,

    // stores all of the chunks that have been created before
    pub created_chunks: Arc<Mutex<HashSet<(i32, i32)>>>,

    // settings
    pub world_name: String,
    pub world_seed: u64,

    pub render_distance: usize,

    pub chunk_sizes: (usize, usize, usize),
}

impl World {
    pub fn new(
        world_name: String,
        world_seed: u64,
        render_distance: usize,
        chunk_sizes: (usize, usize, usize),
    ) -> World {
        // stores all alive chunks in this so they can be rendered and used
        let chunks: HashMap<(i32, i32), Chunk> = HashMap::new();

        // a table of all of the chunks that have been calculated before, Key: (chunkIDx, chunkIDy)
        // the order the hashset is printed changes every run
        let created_chunks: Arc<Mutex<HashSet<(i32, i32)>>> = Arc::new(Mutex::new(HashSet::new()));

        let chunks: Arc<Mutex<HashMap<(i32, i32), Chunk>>> = Arc::new(Mutex::new(chunks));
        // create and return the world
        World {
            chunks,

            created_chunks,
            world_name,
            world_seed,

            render_distance,

            chunk_sizes,
        }
    }

    // TODO: #143 match a load with an unload and add it to the generation thread queue
    // if the player has changed chunks this frame update the chunks around them
    pub fn get_chunks_around_character(
        &mut self,
        character: &Character,
    ) -> HashSet<(i32, i32)> {
        // these are the chunk that should be currently loaded
        let mut chunks_to_load: HashSet<(i32, i32)> = HashSet::new();

        // ill have a funciton which gets all chunks which should be loaded here
        // start at my current position and go left and right render distance amount
        // then go up once and go render distance -1 left and right continue until the top
        let mut current_radius: i32 = self.render_distance as i32;
        let current_chunk_x = character.chunk_position.0;
        let current_chunk_z = character.chunk_position.1;

        for chunk_z_diff in 0..(self.render_distance as i32 + 1) {
            // go left and right all the way
            for chunk_x in (current_chunk_x - current_radius)..(current_chunk_x + current_radius + 1) {

                if chunk_z_diff == 0 {
                    // just do this layer
                    chunks_to_load.insert((chunk_x, current_chunk_z));
                } else {
                    // z up
                    chunks_to_load.insert((chunk_x, current_chunk_z + chunk_z_diff));

                    // z down
                    chunks_to_load.insert((chunk_x, current_chunk_z - chunk_z_diff));
                }
            }

            // now decrease current radius
            current_radius -= 1;
        }

        chunks_to_load
    }


    pub fn update_chunks_around_character(
        &mut self,
        chunks_to_load: HashSet<(i32, i32)>,
        loading_chunks_queue: Arc<Mutex<VecDeque<((i32, i32), Option<(i32, i32)>)>>>,
    ) {

        println!("Loading Chunks Around Character");
        let chunks_locked = self.chunks.lock().unwrap();
        let chunks_hashset: HashSet<(i32, i32)> = chunks_locked.keys().cloned().collect();
        drop(chunks_locked);

        // get the chunks that are in chunks_to_load but not in the chunks
        // these will need to be loaded
        let load: Vec<(i32, i32)> = chunks_to_load.difference(&chunks_hashset).cloned().collect();

        // now get the chunks that are in chunks but not in chunks to load
        // these need to be unloaded
        let unload: Vec<(i32, i32)> = chunks_hashset.difference(&chunks_to_load).cloned().collect();

        // just to check
        if unload.len() > load.len() {
            panic!("Im trying to unload more chunks than im loading, i havent taken this into consideration yet");
        }

        // pair up loads and unloads (if unload doesnt exist then none)
        let mut loading_chunks_queue_locked = loading_chunks_queue.lock().unwrap();
        let mut load_element: ((i32, i32), Option<(i32, i32)>);
        for i in 0..load.len() {
            if i < unload.len() {
                load_element = (load[i], Some(unload[i]));
            } else {
                load_element = (load[i], None);
            }
            
            loading_chunks_queue_locked.push_back(load_element);
            println!("Sending to loading queue: {:?}", load_element);
        }
        drop(loading_chunks_queue_locked);
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
        let line1: String = lines
            .next()
            .expect("Failed to get next line in ChunksCreated.txt, as it is at EOF")
            .unwrap();

        let total_created_chunks: i32 = line1
            .split_whitespace()
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
            line = lines
                .next()
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

            // insert these into the hashset and check if it is a dupe (just a info message so i can fix something if i start getting dupes, which shouldnt happen)
            let mut created_chunks_locked = self.created_chunks.lock().unwrap();
            if !created_chunks_locked.insert((x, z)) {
                // if insert returns false then it was already in the hashmap
                eprintln!(
                    "Duplicate key found when reading chunk ids from ChunksCreated.txt: ({}, {})",
                    x, z
                );
            }
            drop(created_chunks_locked);
        }
    }

    // TODO: #63 set world block from world coords

    // TODO: #64 Get world block from world coords

    // TODO: #65 Place block function

    // TODO: #66 Break block function

}


// universal remove chunk function so that i remove it correctly and save it to a file without needing to do this myself
pub fn remove_chunk(chunks: &mut HashMap<(i32, i32), Chunk>, chunk_id: (i32, i32), file_system: &mut FileSystem, chunk_sizes: (usize, usize, usize)) {
    // remove the chunk from the hashmap and return it
    if let Some(chunk) = chunks.remove(&chunk_id) {
        file_system.save_chunk_to_file(chunk, chunk_sizes);
        //println!("Removed Chunk ({}, {})", chunk_id.0, chunk_id.1);
    } else {
        // if the key doesnt match a value ill print this but not panic so i can save the rest
        eprintln!("Failed to remove chunk with key {:?}", chunk_id);
    }
}
