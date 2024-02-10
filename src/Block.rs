use crate::World::Position;

// what type of block is it
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockType {
    // special blocks
    Air, // no block
    Bedrock, // unbreakable block
    Void, // no block (shouldnt really be used, but when doing calculations and checking boarders of chunks when they dont exists yet)

    // food

    // building

    // enviroment
    Grass,
    Dirt,

    Sand,

    Stone,
    Cobblestone,


    

    

}


impl BlockType {
    pub fn IsDynamic(&self) -> bool {
        match self {
            // special blocks
            BlockType::Air => false,
            BlockType::Bedrock => false,
            BlockType::Void => false,

            // enviroment
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
            BlockType::Void => [0.0, 0.0, 0.0, 0.0].map(|x: f32| x / 255.0),

            BlockType::Grass => [75.0, 150.0, 50.0, 255.0].map(|x: f32| x / 255.0),
            BlockType::Dirt => [75.0, 50.0, 0.0, 255.0].map(|x: f32| x / 255.0),

            BlockType::Sand => [200.0, 200.0, 50.0, 255.0].map(|x: f32| x / 255.0),

            BlockType::Stone => [100.0, 100.0, 100.0, 255.0].map(|x: f32| x / 255.0),
            BlockType::Cobblestone => [150.0, 150.0, 150.0, 255.0].map(|x: f32| x / 255.0),
        }
    }

    pub fn is_transparent(&self) -> bool {
        // label which blocks are transparent or not (air, water, glass, etc.)
        match self {
            // special blocks
            BlockType::Air => true,
            BlockType::Bedrock => false,
            BlockType::Void => false, // dont render a wall of a chunk touching void

            // enviroment
            BlockType::Grass => false,
            BlockType::Dirt => false,

            BlockType::Sand => true,

            BlockType::Stone => false,
            BlockType::Cobblestone => false,

        }
    }

    pub fn ToInt(&self) -> u16 {
        match self {
            // special blocks 0-20
            BlockType::Air => 0,
            BlockType::Bedrock => 1,
            BlockType::Void => 2,

            // food 21-100

            // building 101-300

            // enviroment 301-1000
            BlockType::Grass => 301,
            BlockType::Dirt => 302,

            BlockType::Sand => 320,

            BlockType::Stone => 400,
            BlockType::Cobblestone => 401,

            
        }
    }


    pub fn FromInt(id: u16) -> Self {
        match id {
            // special blocks 0-20
            0 => BlockType::Air,
            1 => BlockType::Bedrock,
            2 => BlockType::Void,

            // food 21-100

            // building 101-300

            // enviroment 301-1000
            301 => BlockType::Grass,
            302 => BlockType::Dirt,

            320 => BlockType::Sand,

            400 => BlockType::Stone,
            401 => BlockType::Cobblestone,

            // if not found panic
            _ => panic!("BlockType not found for id: {}", id),
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
    
    // so i know to send it to the gpu or not (later only send faces touching air)
    pub touchingAir: bool,
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
            touchingAir: false,
        }
    }

    //TODO: #51 recalculate model matrix if a block is dynamic and moves
}

