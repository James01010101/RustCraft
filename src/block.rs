use crate::{block_type::BlockType, types::Position};

// the main strut to hold all info related to a block
#[derive(Clone, Copy)]
pub struct Block {
    // what kind of object is it
    pub block_type: BlockType,

    // its bottom left front position
    pub position: Position,

    // this stores the transform to the camera for this block from world space to camera
    pub model_matrix: [[f32; 4]; 4],

    // so i know to send it to the gpu or not (later only send faces touching air)
    pub touching_air: bool,
}

impl Block {
    pub fn new(block_type: BlockType, pos_x: i32, pos_y: i16, pos_z: i32) -> Block {
        let model_matrix: [[f32; 4]; 4] =
            nalgebra::Translation3::new(pos_x as f32, pos_y as f32, pos_z as f32)
                .to_homogeneous()
                .into(); // into a float 4x4 array

        Block {
            block_type,
            position: Position {
                x: pos_x,
                y: pos_y,
                z: pos_z,
            },
            model_matrix,
            touching_air: false,
        }
    }
}
