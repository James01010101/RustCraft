
use std::path::{Path, PathBuf};
use std::env;

use std::ffi::OsStr;
use std::fs::{create_dir_all, File};
use std::io::Write;

use crate::Settings::*;
use crate::Chunk::*;
use crate::Block::*;

pub struct FileSystem {
    pub dataDirectory: PathBuf,
    pub myWorldDirectory: PathBuf, // the directory of blah/james's World/

}


impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem {
            dataDirectory: PathBuf::new(),
            myWorldDirectory: PathBuf::new(),
        }
    }


    // will check if the files have been created for this world and if not it will create them
    pub fn CheckFileSystem(&mut self) {

        // first check that the data folder exists
        self.dataDirectory = match self.CheckDataFolder() {
            Ok(path) => { path },
            Err(e) => { panic!("Failed to find data directory: {:?}", e); }
        };


        // now check if this game world has a folder and files, if it doesnt ill make them
        self.CheckGameFiles();
    }


    pub fn CheckDataFolder(&self) -> Result<PathBuf, PathBuf> {

        let mut dataDirectory: PathBuf = PathBuf::new();

        // Get a path from the executable to the datya folder
        match env::current_exe() {
            Ok(exePath) => {
                // Create a PathBuf from the executable's path
                dataDirectory = PathBuf::from(exePath);

                // Navigate back out until you find the data folder
                dataDirectory.pop(); // Remove the executable name
                dataDirectory.pop(); // Remove release dir
                dataDirectory.pop(); // Remove target dir

                // Append the relative path to the 'data' directory
                dataDirectory.push("data");
            },
            Err(e) => {
                eprintln!("Failed to get current executable path: {}", e);
            }
        }

        if dataDirectory.exists() && dataDirectory.is_dir() {
            Ok(dataDirectory)
        } else {
            Err(dataDirectory)
        }

    }


    pub fn CheckGameFiles(&mut self) {

        // clone this so i can work on it without changing the data dir
        let mut path: PathBuf = self.dataDirectory.clone();

        // check if the world folder exists
        path.push("Worlds");
        if !path.exists() {
            // Create the directory if it does not exist
            match create_dir_all(&path) {
                Ok(_) => {
                    println!("Created Worlds Directory");
                }
                Err(e) => {
                    eprintln!("Failed to create World Directory at path: {:?}", path);
                    panic!("Error: {}", e);
                }
            }
        }

        // now check if there is a folder for this game world
        path.push(worldName);
        if !path.exists() {
            // Create the directory if it does not exist
            match create_dir_all(&path) {
                Ok(_) => {
                    println!("Created new game world directory: {:?}", worldName);
                }
                Err(e) => {
                    eprintln!("Failed to create new game world directory at path: {:?}", path);
                    panic!("Error: {}", e);
                }
            }
        }
        
        self.myWorldDirectory = path.clone();


        path.push("Chunks");
        if !path.exists() {
            // Create the directory if it does not exist
            match create_dir_all(&path) {
                Ok(_) => {
                    println!("Created chunks directory for world: {:?}", worldName);
                }
                Err(e) => {
                    eprintln!("Failed to create chunks directory at path: {:?}", path);
                    panic!("Error: {}", e);
                }
            }
        }

        // chunks created file
        path.pop();
        path.push("ChunksCreated.txt");

        // if the file doesnt exist make it
        let mut file: File;

        if !path.exists() {
            file = match File::create(&path) {
                Ok(file) => {
                    println!("Created ChunksCreated.txt file for world: {:?}", worldName);
                    file
                }
                Err(e) => {
                    eprintln!("Failed to create ChunksCreated.txt at path: {:?}", path);
                    panic!("Error: {}", e);
                }
            };

            // now write the headings to the file
            let mut data: String = String::new();
            data.push_str("Total Chunks Created : 0\n");
            data.push_str(&format!("Chunk Sizes: ({}, {}, {})\n", chunkSizeX, chunkSizeY, chunkSizeZ));
            // any other data i might want to save can go here
            
            data.push_str(&format!("Created Chunks:\n",));

            // Write data to the file
            match file.write_all(data.as_bytes()) {
                Ok(_) => {}
                Err(e) => { 
                    eprintln!("Failed to write to ChunksCreated.txt");
                    panic!("Error: {}", e);
                }
            }
        }
            


    }


    // TODO: #23 Implement saving chunks back to a file
    // Once a chunk has been loaded and is in play, and then goes out of range it is unloaded and saved back to a file
    // the chunk is not borrowed here so after this call it goes out of scope and is dropped
    pub fn SaveChunkToFile(&mut self, chunk: Chunk) {
        // save the chunk to a file then free it
        println!("Saving Chunk to File: ({}, {})", chunk.chunkIDx, chunk.chunkIDz);
        let mut filePath: PathBuf = self.myWorldDirectory.clone();
        filePath.push("Chunks");
        filePath.push(format!("{}_{}.txt", chunk.chunkIDx, chunk.chunkIDz));

        // first create the file, and overwrite it if it already exists
        let mut file: File = match File::create(filePath) {
            Ok(file) => { file },
            Err(e) => { panic!("Failed to create chunk file: {:?}", e); }
        };

        /* 
        now write the chunk data to the file
        since the data is stored in a hashmap it is in a random order
        i will make a vector, load all of the hashmap into the vector into its correct positions, 
        then i can go through the vector in the correct order and write the blocks to a file
        this way there is an order to the blocks in the file so i dont have to store there exact position as well for each block
        so it takes less space to store them.
        */

        // make the 3d vector
        let mut tempChunkVec: Vec<Vec<Vec<BlockType>>> = vec![vec![vec![BlockType::Air; chunkSizeZ as usize]; chunkSizeY as usize]; chunkSizeX as usize]; 
        
        // now go through the hashmap
        for (key, block) in chunk.chunkBlocks.iter() {
            // get the position of the block relative to the chunk
            let chunkRelativeX: usize = key.0.rem_euclid(chunkSizeX as i32) as usize;
            let chunkRelativeY: usize = (key.1 + (chunkSizeY as i16 / 2)) as usize;
            let chunkRelativeZ: usize = key.2.rem_euclid(chunkSizeZ as i32) as usize;

    
            // put this block into the temp vector, but just its block type
            tempChunkVec[chunkRelativeX][chunkRelativeY][chunkRelativeZ] = block.blockType;
        }

        /*
        now write the blocks to the file, starting at 0,0,0, and increasing in x, then y, then z
        once x is max make a new line and increase z, once z is max make two new lines and increase y
        */
        let mut data: String = String::new();
        for y in 0..chunkSizeY as usize {
            for z in 0..chunkSizeZ as usize {
                for x in 0..chunkSizeX as usize {
                    data.push_str(&format!("{:?} ", tempChunkVec[x][y][z].ToInt()));
                }
                data.push_str("\n");
            }
            data.push_str("\n");
        }

        // now write the data string to the file
        match file.write_all(data.as_bytes()) {
            Ok(_) => {}
            Err(e) => { 
                eprintln!("Failed to write to chunk file: {:?}", e);
                panic!("Error: {}", e);
            }
        }
        
        

    }

}