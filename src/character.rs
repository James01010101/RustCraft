
use crate::types::FPosition;

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

    //TODO: #110 save character position on cleanup and load it back in on load
}