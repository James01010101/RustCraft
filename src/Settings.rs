

// the name of the current world im loading
pub const worldName: &str = "James's World";

// Chunk Size
pub const chunkSizeX: usize = 32;
pub const chunkSizeY: usize = 32;
pub const chunkSizeZ: usize = 256; // half under, half above

// TODO: #24 work out how to offert indexess so z=0 is water level and negative is under
pub const chunkOffestZ: usize = chunkSizeZ / 2; // this is what ill take away when indexing so that z=0 is water and under is negative

// Render Distance is x and y directions, so 5 would be 25 chunks loaded
pub const renderDistance: usize = 5;

// the size of the instance buffer to render all of the blocks, might be different for each type???
pub const maxBlocksRendered: usize = 10;

pub const screenWidth: usize = 1920;
pub const screenHeight: usize = 1080;
pub const screenFOV: f32 = 90.0; // degrees, it will be converted to radians for actual calculations