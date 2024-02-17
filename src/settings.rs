

// the name of the current world im loading
pub const WORLD_NAME: &str = "James's World";
pub const WORLD_SEED: u64 = 1;

// if shipping is not enabled i go back 3 directories to get to the assets folder otherwise i go back 1 for shipping
pub const EXE_DIRECTORY_LEVEL: usize = if cfg!(feature = "shipping") { 1 } else { 3 };


// Chunk Size
pub const CHUNK_SIZE_X: usize = 8; //32;
pub const CHUNK_SIZE_Y: usize = 16; //256; // half under, half above, Y is up down
pub const CHUNK_SIZE_Z: usize = 8; //32; 

pub const HALF_CHUNK_Y: usize = CHUNK_SIZE_Y / 2; // this is what ill take away when indexing so that z=0 is water and under is negative

// Render Distance is the radius of the circle
pub const RENDER_DISTANCE: usize = 1;

pub const SCREEN_WIDTH: usize = 1920;
pub const SCREEN_HEIGHT: usize = 1080;
pub const SCREEN_FOV: f32 = 90.0; // degrees, it will be converted to radians for actual calculations

// mouse and keyboard settings
pub const MOUSE_SENSITIVITY_H: f32 = 0.002;
pub const MOUSE_SENSITIVITY_V: f32 = 0.003;

pub const MOVEMENT_SPEED: f32 = 0.1;