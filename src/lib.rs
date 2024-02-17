// warnings to ignore
/*
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// might use these later
#![allow(unused_imports)]

#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

#![allow(temporary_cstring_as_ptr)] // when i do .as_ptr() as a param to a func it will dealloc after
*/

// so i can set things to 0 even if i set them straight after
#![allow(unused_assignments)]



// create any modules i need
pub mod main_game_loop; // where i create the window and renderer and the main loop
pub mod renderer;
pub mod camera; // anything to do with camera
pub mod block; // where i create my basic objects like spheres and squares
pub mod world; // this is where all of the objects in the world are stored
pub mod settings; // this is a const settings file for things like chunk size, ray bounces ect.\
pub mod chunk; // where the blocks and chunks are stored
pub mod gpu_data; // where the vbo vao ebo, and vertex and index buffers are as well as textures
pub mod file_system; // where anything to read and write to the file system is stored
pub mod window_wrapper; // where i store the window and event loop
pub mod character; // where i store everything to do with the character
pub mod types; // where any small types live, like position, instances
pub mod block_type; // seperating the block type from the block struct
pub mod my_keyboard; // stores all key presses
pub mod calculate_frame; // where i do all the calculations for the frame