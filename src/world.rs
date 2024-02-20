use crate::{character::*, chunk::*, file_system::*, renderer::*};

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

// this struct will hold all of the Chunks as well as arrays of mobs
pub struct World {
    // Use a hashmap to store currently loaded chunks
    pub chunks: HashMap<(i32, i32), Chunk>,

    /* 
    once i create a chunk. it may not be vaid this frame. this can be if it create the instances buffer with less than the number of instance
    in this case there is no valid instance buffer which has enough room to render all the instance, there would be a new buffer being created 
    at this point and is in the middle of copying, but it isnt finished. so instead of having a check every single frame for every single buffer
    to check if it is valid, ill add each new chunk here and the once a frame ill check if it is valid and if it is ill remove it from here and 
    put it into chunks, so it can be rendered like normal
    */
    pub pending_chunks: Vec<Chunk>,

    // stores all of the chunks that have been created before
    pub created_chunks: HashSet<(i32, i32)>,

    // settings
    pub world_name: String,
    pub world_seed: u64,

    pub render_distance: usize,

    pub chunk_size_x: usize,
    pub chunk_size_y: usize,
    pub chunk_size_z: usize,
    pub half_chunk_y: usize,
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
        let created_chunks: HashSet<(i32, i32)> = HashSet::new();

        // create and return the world
        World {
            chunks,
            pending_chunks: Vec::new(),

            created_chunks,
            world_name,
            world_seed,

            render_distance,

            chunk_size_x: chunk_sizes.0,
            chunk_size_y: chunk_sizes.1,
            chunk_size_z: chunk_sizes.2,
            half_chunk_y: chunk_sizes.1 / 2,
        }
    }

    // if the player has changed chunks this frame update the chunks around them
    pub fn update_chunks_around_character(
        &mut self,
        character: &Character,
        renderer: &Renderer,
        file_system: &mut FileSystem,
    ) {
        // these are the chunk that should be currently loaded
        let mut chunks_to_load: Vec<(i32, i32)> = Vec::new();

        // ill have a funciton which gets all chunks which should be loaded here
        // start at my current position and go left and right render distance amount
        // then go up once and go render distance -1 left and right continue until the top
        let max_radius: i32 = self.render_distance as i32;
        let mut current_radius: i32 = max_radius;
        let current_chunk_x = character.chunk_position.0;
        let current_chunk_z = character.chunk_position.1;

        for chunk_z_diff in 0..max_radius + 1 {
            // go left and right all the way
            for chunk_x in
                (current_chunk_x - current_radius)..(current_chunk_x + current_radius + 1)
            {
                // z up
                chunks_to_load.push((chunk_x, current_chunk_z + chunk_z_diff));

                // z down
                chunks_to_load.push((chunk_x, current_chunk_z - chunk_z_diff));
            }

            // now decrease current radius
            current_radius -= 1;
        }

        // now go through the chunks loaded and match them to this array.
        // if something is in the array but not the hashmap ill add it
        // if something is in the hashmap but not the array ill remove it
        // if something is in both ill do nothing
        for i in 0..chunks_to_load.len() {
            let x: i32 = chunks_to_load[i].0;
            let z: i32 = chunks_to_load[i].1;

            // if the loaded chunks doesnt contain this chunk ill load it
            if !self.chunks.contains_key(&(x, z)) {

                // also check that it isnt in the pending chunks
                let mut is_in_pending: bool = false;
                for chunk in self.pending_chunks.iter() {
                    if chunk.chunk_id_x == x && chunk.chunk_id_z == z {
                        is_in_pending = true;
                        break;
                    }
                }  

                if !is_in_pending {
                    // load this chunk (i know for sure it isnt contained in the hashmap so i can just insert it)
                    let mut c: Chunk = Chunk::new(x, z, -1, renderer);
                    c.load_chunk(file_system, self, renderer);
                    self.pending_chunks.push(c);
                }


            }
        }

        // now go through the chunks loaded and match them to this array.
        // if something is in the array but not the hashmap ill add ià
        // if something is in the hashmap but not the array ill remove it
        // if something is in both ill do nothing
        let chunk_keys: Vec<(i32, i32)> = self.chunks.keys().cloned().collect();
        for (x, z) in chunk_keys {
            // if the chunk is not in the chunks to load array then remove it
            if !chunks_to_load.contains(&(x, z)) {
                // remove this chunk and save it to a file
                self.remove_chunk((x, z), file_system);
            }
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

            // insert these into the hashset and check if it is a dupe
            if !self.created_chunks.insert((x, z)) {
                // if insert returns false then it was already in the hashmap
                eprintln!(
                    "Duplicate key found when reading chunk ids from ChunksCreated.txt: ({}, {})",
                    x, z
                );
            }
        }
    }

    // TODO: #63 set world block from world coords

    // TODO: #64 Get world block from world coords

    // TODO: #65 Place block function

    // TODO: #66 Break block function

    // universal remove chunk function so that i remove it correctly and save it to a file without needing to do this myself
    pub fn remove_chunk(&mut self, chunk_id: (i32, i32), file_system: &mut FileSystem) {
        // remove the chunk from the hashmap and return it
        if let Some(chunk) = self.chunks.remove(&chunk_id) {
            file_system.save_chunk_to_file(chunk, self);
            //println!("Removed Chunk ({}, {})", chunk_id.0, chunk_id.1);
        } else {
            // if the key doesnt match a value ill print this but not panic so i can save the rest
            eprintln!("Failed to remove chunk with key {:?}", chunk_id);
        }
    }
}
