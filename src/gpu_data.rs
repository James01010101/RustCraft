
use crate::renderer::*;

use wgpu::{
    BufferUsages,
    Buffer,
    util::{BufferInitDescriptor, DeviceExt},
};

use bytemuck;

pub struct GPUData {
    pub cube_vertices: Vec<f32>,
    pub cube_indices: Vec<u16>, 

    pub vertex_buf: Buffer,
    pub index_buf: Buffer,

    pub vertex_uniform_staging_buf: Buffer,

    pub instances_modified: bool,
}


impl GPUData {
    pub fn new (renderer: &Renderer) -> GPUData {
        // cube vertices (assume starts at (0,0,0))
        let cube_vertices: Vec<f32> = vec![
            0.0, 0.0, 0.0, // Bottom Front Left
            1.0, 0.0, 0.0, // Bottom Front Right
            1.0, 0.0, 1.0, // Bottom Back Right
            0.0, 0.0, 1.0, // Bottom Back Left

            0.0, 1.0, 0.0, // Top Front Left
            1.0, 1.0, 0.0, // Top Front Right
            1.0, 1.0, 1.0, // Top Back Right
            0.0, 1.0, 1.0, // Top Back Left
        ];

        // this is the indexes into the cubeVertices array, so it knows what vertices to use for what triangles
        let cube_indices: Vec<u16> = vec![
            // Front face
            0, 1, 5, 0, 5, 4,
            // Back face
            2, 3, 7, 2, 7, 6,
            // Bottom face
            0, 3, 2, 0, 2, 1,
            // Top face
            4, 5, 6, 4, 6, 7,
            // Left face
            0, 4, 7, 0, 7, 3,
            // Right face
            1, 2, 6, 1, 6, 5
        ];

        // create the buffers for this data
        // now create the vertex buffer for the gpu
        let vertex_buf = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&cube_vertices),
            usage: BufferUsages::VERTEX,
        });

        // now make the index buffer for the gpu
        let index_buf = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&cube_indices),
            usage: BufferUsages::INDEX,
        });

        // vertex uniform staging buiffer
        let vertex_uniform_staging_buf: wgpu::Buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Uniform Staging Buffer"),
            contents: bytemuck::bytes_of(&renderer.vertex_uniforms),
            usage: BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        });


        Self {
            cube_vertices,
            cube_indices,

            vertex_buf,
            index_buf,

            vertex_uniform_staging_buf,

            instances_modified: false,
        }
    }    
}