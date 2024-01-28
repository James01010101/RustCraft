use crate::World::Position;

// what type of block is it
pub enum BlockType {
    Air, // no block

    Grass,
    Dirt,

    Sand,

    Stone,
    Cobblestone,

}


impl BlockType {
    pub fn IsDynamic(&self) -> bool {
        match self {
            BlockType::Air => false,

            BlockType::Grass => false,
            BlockType::Dirt => false,

            BlockType::Sand => true,

            BlockType::Stone => false,
            BlockType::Cobblestone => false,
        }
    }
}





// the main strut to hold all info related to a block
// previously 448 Bytes now 136 static (not including vector faces)
pub struct Block {
    // what kind of object is it
    pub blockType: BlockType,

    // its bottom left back position
    pub position: Position,

    // this stores the transform to the camera for this block from world space to camera
    pub modelMatrix: [[f32; 4]; 4],

    pub colour: [f32; 4],
}


impl Block {
    pub fn new(blockType: BlockType, posX: i32, posY: i16, posZ: i32, R: f32, G: f32, B: f32) -> Block {
        

        let modelMatrix: [[f32; 4]; 4] = nalgebra::Translation3::new(
            posX as f32,
            posY as f32,
            posZ as f32
        ).to_homogeneous().into(); // into a float 4x4 array

        //println!("Model Matrix: {:?}", modelMatrix);

        Block {
            blockType,
            position: Position { x: posX, y: posY, z: posZ },
            modelMatrix,
            colour: [R, G, B, 1.0],
        }
    }
}

