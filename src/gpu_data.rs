use crate::renderer::*;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages,
};

pub struct GPUData {
    pub cube_vertices: Vec<f32>,
    pub cube_indices: Vec<u16>,

    pub vertex_buf: Buffer,
    pub index_buf: Buffer,

    pub vertex_uniform_staging_buf: Buffer,
}

impl GPUData {
    pub fn new(renderer: &Renderer) -> GPUData {
        // cube vertices (assume starts at (0,0,0) which is not the bottom front right to align with the world coords)
        let cube_vertices: Vec<f32> = vec![
            0.0, 0.0, 0.0, // 0 Bottom Front Right
            1.0, 0.0, 0.0, // 1 Bottom Front Left
            1.0, 0.0, 1.0, // 2 Bottom Back Left
            0.0, 0.0, 1.0, // 3 Bottom Back Right
            0.0, 1.0, 0.0, // 4 Top Front Right
            1.0, 1.0, 0.0, // 5 Top Front Left
            1.0, 1.0, 1.0, // 6 Top Back Left
            0.0, 1.0, 1.0, // 7 Top Back Right
        ];

        // this is the indexes into the cubeVertices array, so it knows what vertices to use for what triangles
        // they are clockwise so the triangles are facing the right way
        let cube_indices: Vec<u16> = vec![
            // Front face
            0, 1, 4, 5, 4, 1, // Back face
            3, 7, 2, 6, 2, 7, // Bottom face
            3, 2, 0, 1, 0, 2, // Top face
            4, 5, 7, 6, 7, 5, // Left face
            1, 2, 5, 6, 5, 2, // Right face
            3, 0, 7, 4, 7, 0,
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
        let vertex_uniform_staging_buf: wgpu::Buffer =
            renderer.device.create_buffer_init(&BufferInitDescriptor {
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
        }
    }
}
