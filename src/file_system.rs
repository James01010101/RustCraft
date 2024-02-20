use crate::{block::*, block_type::*, chunk::*, world::*};

use std::{
    env,
    fs::{create_dir_all, File},
    io::{self, BufRead, Write},
    path::PathBuf,
};

pub struct FileSystem {
    pub assets_directory: PathBuf,   // the directory of the assets folder
    pub my_world_directory: PathBuf, // the directory of blah/james's World/
}

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem {
            assets_directory: PathBuf::new(),
            my_world_directory: PathBuf::new(),
        }
    }

    // will check if the files have been created for this world and if not it will create them
    pub fn check_file_system(&mut self, world: &World) {
        // first check that the data folder exists
        self.check_data_folder();

        // now check if this game world has a folder and files, if it doesnt ill make them
        self.check_game_files(&world);
    }

    pub fn check_data_folder(&mut self) {
        let mut path: PathBuf = env::current_exe().unwrap();
        let exe_directory_level: usize = if cfg!(feature = "shipping") { 1 } else { 3 };
        for _ in 0..exe_directory_level {
            path.pop();
        }
        path.push("assets");

        // save this as the assets directory
        self.assets_directory = path.clone();

        // continue to check the data directory
        path.push("data");

        if !path.exists() || !path.is_dir() {
            panic!(
                "Data directory ({:?}) does not exist or is not a directory",
                path
            );
        }
    }

    pub fn check_game_files(&mut self, world: &World) {
        // get to the data dir
        let mut path: PathBuf = self.assets_directory.clone();
        path.push("data");

        // check if the world folder exists
        path.push("Worlds");
        if !path.exists() {
            // Create the directory if it does not exist
            create_dir_all(&path).unwrap();
        }

        // now check if there is a folder for this game world
        path.push(world.world_name.clone());
        if !path.exists() {
            // Create the directory if it does not exist
            match create_dir_all(&path) {
                Ok(_) => {
                    println!("Created new game world directory: {:?}", world.world_name);
                }
                Err(e) => {
                    eprintln!(
                        "Failed to create new game world directory at path: {:?}",
                        path
                    );
                    panic!("Error: {}", e);
                }
            }
        }

        self.my_world_directory = path.clone();

        path.push("Chunks");
        if !path.exists() {
            // Create the directory if it does not exist
            create_dir_all(&path).unwrap();
        }

        // chunks created file
        path.pop();
        path.push("ChunksCreated.txt");

        // if the file doesnt exist make it
        let mut file: File;

        if !path.exists() {
            file = File::create(&path).unwrap();

            // now write the headings to the file
            let mut data: String = String::new();
            data.push_str("Total Chunks Created : 0\n");
            data.push_str(&format!(
                "Chunk Sizes: ({}, {}, {})\n",
                world.chunk_size_x, world.chunk_size_y, world.chunk_size_z
            ));
            // any other data i might want to save can go here

            data.push_str(&format!("Created Chunks:\n",));

            // Write data to the file
            file.write_all(data.as_bytes()).unwrap()
        }

        // create a world info file
        path.pop();
        path.push("WorldInfo.txt");

        // if the file doesnt exist make it
        if !path.exists() {
            file = File::create(&path).unwrap();

            // now write the headings to the file
            let mut data: String = String::new();
            data.push_str("Important Info Goes Here: \n");
            file.write_all(data.as_bytes()).unwrap()
        }

        // create a stats info file
        path.pop();
        path.push("Stats.txt");

        // if the file doesnt exist make it
        if !path.exists() {
            file = File::create(&path).unwrap();

            // now write the headings to the file
            let mut data: String = String::new();
            data.push_str("TimeSpent: 0\n");
            data.push_str("DistanceTravelled: 0\n");
            data.push_str("BlocksPlaced: 0\n");
            data.push_str("BlocksDestroyed: 0\n");
            data.push_str("Deaths: 0\n");
            data.push_str("DamageTaken: 0\n");
            data.push_str("DamageDelt: 0\n");
            data.push_str("MobsKilled: 0\n");

            // Write data to the file
            file.write_all(data.as_bytes()).unwrap()
        }
    }

    // Once a chunk has been loaded and is in play, and then goes out of range it is unloaded and saved back to a file
    // the chunk is not borrowed here so after this call it goes out of scope and is dropped
    pub fn save_chunk_to_file(&mut self, chunk: Chunk, world: &World) {
        // save the chunk to a file then free it
        //println!("Saving Chunk to File: ({}, {})", chunk.chunk_id_x, chunk.chunk_id_z);
        let mut file_path: PathBuf = self.my_world_directory.clone();
        file_path.push("Chunks");
        file_path.push(format!("{}_{}.txt", chunk.chunk_id_x, chunk.chunk_id_z));

        // first create the file, and overwrite it if it already exists
        let mut file: File = File::create(file_path).unwrap();

        /*
        now write the chunk data to the file
        since the data is stored in a hashmap it is in a random order
        i will make a vector, load all of the hashmap into the vector into its correct positions,
        then i can go through the vector in the correct order and write the blocks to a file
        this way there is an order to the blocks in the file so i dont have to store there exact position as well for each block
        so it takes less space to store them.
        */

        // make the 3d vector
        let mut temp_chunk_vec: Vec<Vec<Vec<BlockType>>> =
            vec![
                vec![vec![BlockType::Air; world.chunk_size_x]; world.chunk_size_y];
                world.chunk_size_z
            ];

        // now go through the hashmap
        for (key, block) in chunk.chunk_blocks.iter() {
            // get the position of the block relative to the chunk
            let chunk_relative_x: usize = key.0.rem_euclid(world.chunk_size_x as i32) as usize;
            let chunk_relative_y: usize = (key.1 + (world.chunk_size_y as i16 / 2)) as usize;
            let chunk_relative_z: usize = key.2.rem_euclid(world.chunk_size_z as i32) as usize;

            // put this block into the temp vector, but just its block type
            temp_chunk_vec[chunk_relative_x][chunk_relative_y][chunk_relative_z] = block.block_type;
        }

        /*
        now write the blocks to the file, starting at 0,0,0, and increasing in x, then y, then z
        once x is max make a new line and increase z, once z is max make two new lines and increase y
        */
        let mut data: String = String::new();
        for y in 0..world.chunk_size_y as usize {
            for z in 0..world.chunk_size_z as usize {
                for x in 0..world.chunk_size_x as usize {
                    data.push_str(&format!("{:?} ", temp_chunk_vec[x][y][z].to_int()));
                }
                data.push_str("\n");
            }
            data.push_str("\n");
        }

        // now write the data string to the file
        file.write_all(data.as_bytes()).unwrap();
    }

    // save the created chunks file
    pub fn save_created_chunks_file(&mut self, world: &mut World) {
        let mut data: String = String::new();

        // write the header lines
        data.push_str(&format!(
            "Total Chunks Created : {:?}\n",
            world.created_chunks.len()
        ));
        data.push_str(&format!(
            "Chunk Sizes: ({}, {}, {})\n",
            world.chunk_size_x, world.chunk_size_y, world.chunk_size_z
        ));
        data.push_str("Created Chunks: \n");
        for key in world.created_chunks.drain() {
            data.push_str(&format!("{} {}\n", key.0, key.1));
        }

        // open the file
        let mut path: PathBuf = self.my_world_directory.clone();
        path.push("ChunksCreated.txt");
        let mut file: File = File::create(path).unwrap();

        // now write the data string to the file
        file.write_all(data.as_bytes()).unwrap();
    }

    pub fn read_chunks_from_file(
        &mut self,
        temp_chunk_vec: &mut Vec<Vec<Vec<Block>>>,
        chunk_id_x: i32,
        chunk_id_z: i32,
        world: &World,
    ) {
        // read the chunk from a file and fill the temp vector with the data
        println!("Reading Chunk from File: ({}, {})", chunk_id_x, chunk_id_z);

        // get the file path
        let mut file_path: PathBuf = self.my_world_directory.clone();
        file_path.push("Chunks");
        file_path.push(format!("{}_{}.txt", chunk_id_x, chunk_id_z));

        //check that this file exists
        if !file_path.exists() {
            panic!(
                "Trying to read chunk file that does not exist: {:?}",
                file_path
            );
        }

        // open the file
        let file: File = File::open(file_path).unwrap();

        // read the file line by line
        let reader: io::BufReader<File> = io::BufReader::new(file);
        let lines = reader.lines();

        // iterate through the lines and fill the temp vector with the data
        let mut x: usize;
        let mut y: usize = 0;
        let mut z: usize = 0;
        let mut new_line: String;
        let mut skip_line: bool = false;
        for line in lines {
            new_line = line.unwrap();

            // so i can skip empty lines
            if skip_line {
                skip_line = false;
                continue;
            }

            x = 0;
            for block in new_line.split_whitespace() {
                temp_chunk_vec[x][y][z].block_type =
                    BlockType::from_int(block.parse::<u16>().unwrap());
                x += 1;
            }
            z += 1;

            // if ive read the whole layer increase y
            if z == world.chunk_size_z {
                z = 0;
                y += 1;

                // i need to read in the next line here and discard it
                skip_line = true;

                // if im at the end then break
                if y == world.chunk_size_y {
                    break;
                }
            }
        }
    }
}
