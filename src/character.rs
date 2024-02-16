
use crate::{
    types::FPosition,
    settings::*,
    camera::*,
};

pub struct Character {

    position: FPosition,
    chunk_position: (i32, i32),
}


impl Character {
    pub fn new() -> Character {
        Character {
            position: FPosition { x: 0.0, y: 0.0, z: 0.0 },
            chunk_position: (0, 0),
        }
    }

    pub fn update_chunk_position(&mut self) {
        self.chunk_position = (self.position.x as i32 / CHUNK_SIZE_X as i32, self.position.z as i32 / CHUNK_SIZE_Z as i32);
    }

    pub fn get_current_chunk(&self) -> (i32, i32) {
        self.chunk_position
    }
    

    //TODO: #110 save character position on cleanup and load it back in on load
}