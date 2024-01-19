

// Chunk Size
pub const chunkSizeX: u16 = 32;
pub const chunkSizeY: u16 = 32;
pub const chunkSizeZ: u16 = 256; // half under, half above

// TODO: #24 work out how to offert indexess so z=0 is water level and negative is under
pub const chunkOffestZ: u16 = chunkSizeZ / 2; // this is what ill take away when indexing so that z=0 is water and under is negative

// Render Distance is x and y directions, so 5 would be 25 chunks loaded
pub const renderDistance: u8 = 5;