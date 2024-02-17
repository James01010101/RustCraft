
use crate::{
    types::FPosition,
    settings::*,
};

pub struct Character {

    // current position im standing at (where my head is)
    pub position: FPosition,

    // the position im looking at (for camera rendering stuff)
    pub target: FPosition,

    // chunk im standing in
    pub chunk_position: (i32, i32),
}


impl Character {
    pub fn new() -> Character {
        Character {
            position: FPosition { x: 0.0, y: 2.0, z: -5.0 },
            target: FPosition { x: 0.0, y: 0.0, z: 0.0 },
            chunk_position: (0, 0),
        }
    }

    pub fn update_chunk_position(&mut self) {
        self.chunk_position = (self.position.x as i32 / CHUNK_SIZE_X as i32, self.position.z as i32 / CHUNK_SIZE_Z as i32);
    }

    pub fn get_current_chunk(&self) -> (i32, i32) {
        self.chunk_position
    }

    pub fn move_forward(&mut self, amount: f32) {
        self.position.z += amount;
        self.target.z += amount;
    }

    pub fn move_sideways(&mut self, amount: f32) {
        self.position.x += amount;
        self.target.x += amount;
    }
    

    //TODO: #110 save character position on cleanup and load it back in on load
}