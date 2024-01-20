
use crate::Chunk::*;
use crate::Objects::*;

use std::collections::HashMap;


// this struct will hold all of the Chunks as well as arrays of mobs
struct World {

    // TODO: #25 Use a hashmap to store currently loaded chunks
    chunks: HashMap<i32, Chunk>,

    testBlocks: Vec<Block>,
}


// TODO: #22 Make the create world function
pub fn CreateWorld() {


}