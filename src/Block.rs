use crate::World::Position;

// what type of block is it
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockType {
    Air, // no block
    Bedrock, // unbreakable block

    // enviroment
    Grass,
    Dirt,

    Sand,

    Stone,
    Cobblestone,


    // structure

    // food

}


impl BlockType {
    pub fn IsDynamic(&self) -> bool {
        match self {
            BlockType::Air => false,
            BlockType::Bedrock => false,

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
            BlockType::Bedrock => [50.0, 50.0, 50.0, 255.0].map(|x: f32| x / 255.0),

            BlockType::Grass => [75.0, 150.0, 50.0, 255.0].map(|x: f32| x / 255.0),
            BlockType::Dirt => [75.0, 50.0, 0.0, 255.0].map(|x: f32| x / 255.0),

            BlockType::Sand => [200.0, 200.0, 50.0, 255.0].map(|x: f32| x / 255.0),

            BlockType::Stone => [100.0, 100.0, 100.0, 255.0].map(|x: f32| x / 255.0),
            BlockType::Cobblestone => [150.0, 150.0, 150.0, 255.0].map(|x: f32| x / 255.0),
        }
    }
}





// the main strut to hold all info related to a block
#[derive(Clone, Copy)]
pub struct Block {
    // what kind of object is it
    pub blockType: BlockType,

    // its bottom left front position
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

        Block {
            blockType,
            position: Position { x: posX, y: posY, z: posZ },
            modelMatrix,
        }
    }

    //TODO: #51 recalculate model matrix if a block is dynamic and moves
}

