use crate::Settings::*;
use crate::World::*;
use crate::Renderer::*;

use wgpu::{
    Queue,
    BufferUsages,
    Buffer,
    util::{BufferInitDescriptor, DeviceExt},
    
};

use bytemuck;

pub struct GPUData {
    pub instancesUsed: u32, // how many of the instances am i actually using this frame

    pub cubeVertices: Vec<f32>,
    pub cubeIndices: Vec<u16>, 

    pub cubeInstanceModelMatricies: [[[f32; 4]; 4]; maxBlocksRendered],
    pub cubeColours: [[f32; 4]; maxBlocksRendered], // temporary for now until i use textures

    pub vertex_buf: Buffer,
    pub index_buf: Buffer,
    pub instance_buf: Buffer,
    pub colour_buf: Buffer,

    pub instance_staging_buf: Buffer,
    pub colour_staging_buf: Buffer,
    pub vertex_uniform_staging_buf: Buffer,

    pub instances_modified: bool,
    

}


impl GPUData {
    pub fn new (renderer: &Renderer) -> GPUData {
        // cube vertices (assume starts at (0,0,0))
        let cubeVertices: Vec<f32> = vec![
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
        let cubeIndices: Vec<u16> = vec![
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

        // instance array
        
        let mut cubeInstanceModelMatricies: [[[f32; 4]; 4]; maxBlocksRendered] = [[[0.0; 4]; 4]; maxBlocksRendered];

        let mut cubeColours: [[f32; 4]; maxBlocksRendered] = [[0.0; 4]; maxBlocksRendered];
           

        // create the buffers for this data
        // now create the vertex buffer for the gpu
        let vertex_buf = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&cubeVertices),
            usage: BufferUsages::VERTEX,
        });

        // now make the index buffer for the gpu
        let index_buf = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&cubeIndices),
            usage: BufferUsages::INDEX,
        });


        let instance_buf: wgpu::Buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&cubeInstanceModelMatricies),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        let colour_buf: wgpu::Buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Colour Buffer"),
            contents: bytemuck::cast_slice(&cubeColours),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });


        // use these staging buffers so that i can copy them to the gpu from the cpu, which takes along time, async
        // then once the buffers are ready i copy them to the actual buffers on the gpu to be used
        let instance_staging_buf: wgpu::Buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Staging Buffer"),
            contents: bytemuck::cast_slice(&cubeInstanceModelMatricies),
            usage: BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        });

        let colour_staging_buf: wgpu::Buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Colour Staging Buffer"),
            contents: bytemuck::cast_slice(&cubeColours),
            usage: BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        });

        // vertex uniform staging buiffer
        let vertex_uniform_staging_buf: wgpu::Buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Uniform Staging Buffer"),
            contents: bytemuck::bytes_of(&renderer.vertUniforms),
            usage: BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        });


        Self {
            instancesUsed: 0,
            cubeVertices,
            cubeIndices,

            cubeInstanceModelMatricies,
            cubeColours,

            vertex_buf,
            index_buf,
            instance_buf,
            colour_buf,

            instance_staging_buf,
            colour_staging_buf,
            vertex_uniform_staging_buf,

            instances_modified: false,

            
        }
    }

    
    // TODO: #57 correctly load the chunks of blocks onto the gpu
    pub fn UpdateCubeInstances(&mut self, world: &mut World, queue: &Queue) {

        self.instancesUsed = world.testBlocks.len() as u32;

        // Instance model matricies, each element is a model matrix of a block
        for i in 0..self.instancesUsed {
            let i: usize = i as usize;

            self.cubeInstanceModelMatricies[i] = world.testBlocks[i].modelMatrix;
            self.cubeColours[i] = world.testBlocks[i].blockType.BlockColour();
        }

        // update the staging gpu buffers and set the flag that this data has changed
        queue.write_buffer(&self.instance_staging_buf, 0, bytemuck::cast_slice(&self.cubeInstanceModelMatricies));
        queue.write_buffer(&self.colour_staging_buf, 0, bytemuck::cast_slice(&self.cubeColours));
        self.instances_modified = true;

        // submit those write buffers so they are run
        queue.submit(std::iter::empty());

    }
}