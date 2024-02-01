

// the name of the current world im loading
pub const worldName: &str = "James's World";

// Chunk Size
pub const chunkSizeX: usize = 8; //32;
pub const chunkSizeY: usize = 16; //256; // half under, half above, Y is up down
pub const chunkSizeZ: usize = 8; //32; 

// TODO: #24 work out how to offert indexess so z=0 is water level and negative is under
pub const chunkOffestZ: usize = chunkSizeZ / 2; // this is what ill take away when indexing so that z=0 is water and under is negative

// Render Distance is the radius of the circle
pub const renderDistance: usize = 5;

// the size of the instance buffer to render all of the blocks, might be different for each type???
// TODO: #40 make sure the active rendered cant go above this
pub const maxBlocksRendered: usize = 100;

pub const screenWidth: usize = 1920;
pub const screenHeight: usize = 1080;
pub const screenFOV: f32 = 90.0; // degrees, it will be converted to radians for actual calculations