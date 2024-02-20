/* This will store any structs that can be used all over
like position
*/

use bytemuck::{Pod, Zeroable};

// its position is the bottom left front part of the square
#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

impl Position {
    pub fn new(x: i32, y: i16, z: i32) -> Position {
        Position { x, y, z }
    }
}

// same but a float position
#[derive(Clone, Copy, Debug)]
pub struct FPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl FPosition {
    pub fn new(x: f32, y: f32, z: f32) -> FPosition {
        FPosition { x, y, z }
    }
}

// Define your uniform data to store the view and projection matrixies
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct VertexUniforms {
    pub projection_view_matrix: [[f32; 4]; 4],
}

// this will store all data related to a instance so i can move this into the buffer
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct InstanceData {
    pub model_matrix: [[f32; 4]; 4],
    pub colour: [f32; 4],
}
