
use crate::Chunk::*;
use crate::FileSystem::FileSystem;
use crate::Block::*;

use std::{collections::{HashMap, HashSet}, path::PathBuf};
use std::io::{self, BufRead};
use std::fs::File;



// this struct will hold all of the Chunks as well as arrays of mobs
pub struct World {

    // TODO: #25 Use a hashmap to store currently loaded chunks
    pub chunks: HashMap<(i32, i32), Chunk>,

    // stores all of the chunks that have been created before
    pub createdChunks: HashSet<(i32, i32)>,

    pub testBlocks: Vec<Block>,
}


impl World {
    pub fn new() -> World {

        // stores all alive chunks in this so they can be rendered and used
        let mut chunks: HashMap<(i32, i32), Chunk> = HashMap::new();

        // a table of all of the chunks that have been calculated before, Key: (chunkIDx, chunkIDy)
        // the order the hashset is printed changes every run
        let mut createdChunks: HashSet<(i32, i32)> = HashSet::new();

        // a temp vec of blocks to put into the world without world gen
        let mut testBlocks: Vec<Block> = Vec::new();

        // create and return the world
        World { 
            chunks, 
            createdChunks,
            testBlocks, 
        }

    }

    pub fn AddTestBlocks(&mut self) {

        // Create some blocks to put into the testBlocks array
        self.testBlocks.push(Block::new(
            BlockType::Air, 
            1, 0, 0)
        );

        self.testBlocks.push(Block::new(
            BlockType::Dirt, 
            0, 0, 0)
        );

        self.testBlocks.push(Block::new(
            BlockType::Grass, 
            -1, 0, 0)
        );

        self.testBlocks.push(Block::new(
            BlockType::Sand, 
            -2, 0, 0)
        );

        self.testBlocks.push(Block::new(
            BlockType::Stone, 
            -3, 0, 0)
        );

        self.testBlocks.push(Block::new(
            BlockType::Cobblestone, 
            -4, 0, 0)
        );
    }


    pub fn AddTestChunks(&mut self) {
        // create the new chunk
        for x in -3..=3 {
            for z in -3..=3 {
                let k: (i32, i32) = (x, z);
                let mut c: Chunk = Chunk::new(k.0, k.1, -1);
                c.LoadChunk(&mut self.createdChunks);
                self.chunks.insert(k, c);
            }
        }

        // add 0,0 again just to check for duplicated
        let k: (i32, i32) = (0, 0);
        let mut c: Chunk = Chunk::new(k.0, k.1, -1);
        c.LoadChunk(&mut self.createdChunks);
        self.chunks.insert(k, c);

    }



    // takes in the filesystem, loads the file where all of the chunks that have been created live and writes them to the hashmap
    pub fn LoadCreatedChunksFile(&mut self, myFileSystem: &mut FileSystem) {

        // get the path to the ChunksCreated.txt file
        let mut chunksCreatedPath: PathBuf = myFileSystem.myWorldDirectory.clone();
        chunksCreatedPath.push("ChunksCreated.txt");

        let chunksCreatedFile: File = File::open(chunksCreatedPath).unwrap();

        let reader: io::BufReader<File> = io::BufReader::new(chunksCreatedFile);

        let mut lines = reader.lines();


        // read the first line to get the total created chunks
        let line1: String = lines.next()
            .expect("Failed to get next line in ChunksCreated.txt, as it is at EOF")
            .unwrap();

        let totalCreatedChunks: i32 = line1.split_whitespace()
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
        for i in 0..totalCreatedChunks {
            line = lines.next()
                .expect("Failed to get next line in ChunksCreated.txt, as it is at EOF")
                .unwrap();

            let mut splitLine = line.split_whitespace();
            x = splitLine.next()
            .expect("Failed to get next element of split whitespace line while reading ChunksCreated.txt")
            .parse::<i32>()
            .unwrap();


            z = splitLine.next()
            .expect("Failed to get next element of split whitespace line while reading ChunksCreated.txt")
            .parse::<i32>()
            .unwrap();


            // insert these into the hashset and check if it is a dupe
            if !self.createdChunks.insert((x, z)) {
                // if insert returns false then it was already in the hashmap
                eprintln!("Duplicate key found when reading chunk ids from ChunksCreated.txt: ({}, {})", x, z);
            }
        }
    }

}


// other important structs
// a basic 3 int position position struct to store the xyz position of a block
// its position is the bottom left back part of the square
#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}   

// TODO: make sure all positions use f32 for now since all gpu calculations use f32
impl Position {
    pub fn new(x: i32, y: i16, z: i32) -> Position {
        Position {
            x,
            y,
            z,
        }
    }
}


// same but a float position
#[derive(Clone, Copy, Debug)]
pub struct FPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}   

// TODO: make sure all positions use f32 for now since all gpu calculations use f32
impl FPosition {
    pub fn new(x: f32, y: f32, z: f32) -> FPosition {
        FPosition {
            x,
            y,
            z,
        }
    }
}