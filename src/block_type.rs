// what type of block is it
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockType {
    // special blocks
    Air,     // no block
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
    pub fn is_dynamic(&self) -> bool {
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

    pub fn block_colour(&self) -> [f32; 4] {
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

            BlockType::Sand => false,

            BlockType::Stone => false,
            BlockType::Cobblestone => false,
        }
    }

    pub fn to_int(&self) -> u16 {
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

    pub fn from_int(id: u16) -> Self {
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
