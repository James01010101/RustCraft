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

    pub fn BlockColour(&self) -> [f32; 4] {
        // go through it and divide by 255 and return
        match self {
            BlockType::Air => [175.0, 250.0, 250.0, 50.0].map(|x: f32| x / 255.0),

            BlockType::Grass => [75.0, 150.0, 50.0, 255.0].map(|x: f32| x / 255.0),
            BlockType::Dirt => [75.0, 50.0, 0.0, 255.0].map(|x: f32| x / 255.0),

            BlockType::Sand => [200.0, 200.0, 50.0, 255.0].map(|x: f32| x / 255.0),

            BlockType::Stone => [100.0, 100.0, 100.0, 255.0].map(|x: f32| x / 255.0),
            BlockType::Cobblestone => [150.0, 150.0, 150.0, 255.0].map(|x: f32| x / 255.0),
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
}


impl Block {
    pub fn new(blockType: BlockType, posX: i32, posY: i16, posZ: i32) -> Block {
        

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
        }
    }
}

