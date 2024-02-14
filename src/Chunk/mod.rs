

pub mod ChunkFunctions;
pub mod CreateChunks;

use crate::Block::*;
use crate::Renderer::*;


use std::collections::HashMap;
use wgpu::{BufferUsages, BufferDescriptor};
use bytemuck::{Pod, Zeroable};



// This will store all of the blocks and objects within a chunk
pub struct Chunk {
    // Blocks, These are all of the mostly static blocks that will be in the world
    // using a tuple as key so i can access it without needing to create a position struct just to get it
    pub chunk_blocks: HashMap<(i32, i16, i32), Block>,

    // just the blocks that are touching air that will be sent to the gpu
    pub instances_to_render: HashMap<(i32, i16, i32), InstanceData>,

    // other objects like mobs that dont fit into a single block
    //pub chunkObjects: Vec

    // Chunk ID eg. floor(posx / chuckSizex) is the chunk
    pub chunk_id_x: i32,
    pub chunk_id_z: i32,

    // total number of non air blocks in the chunk
    pub alive_blocks: u32,

    // allocated capacsty of the instance buffer
    pub instance_capacity: u32,

    // the actual size of the instance buffer being used, (size of hashmap once updated)
    pub instance_size: u32,

    // instance buffer for the chunk
    pub instance_buf: wgpu::Buffer,
    pub instance_staging_buf: wgpu::Buffer,

    // if i update the staging buffer set this true so i know to copy it to the instance buffer
    pub instances_modified: bool,
}


impl Chunk {
    pub fn new(idx: i32, idz: i32, num_blocks: i32, renderer: &Renderer) -> Chunk {

        // if a numBlocks was passed in ill allocate the hashmap of that size
        let chunk_blocks: HashMap<(i32, i16, i32), Block>;
        let instances_to_render: HashMap<(i32, i16, i32), InstanceData>;
        let alive_blocks: u32;

        if num_blocks != -1 {
            chunk_blocks = HashMap::with_capacity(num_blocks as usize);
            alive_blocks = num_blocks as u32;

        } else { // if the number of blocks needed is unknown ill just let it do its thing
            chunk_blocks = HashMap::new();
            alive_blocks = 0;
        }


        // make the instance buffer for the chunk init it with size of 100
        let instance_capacity: u32 = 100;
        let instance_buf: wgpu::Buffer = renderer.device.create_buffer(&BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (std::mem::size_of::<InstanceData>() * instance_capacity as usize) as wgpu::BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let instance_staging_buf: wgpu::Buffer = renderer.device.create_buffer(&BufferDescriptor {
            label: Some("Instance Staging Buffer"),
            size: (std::mem::size_of::<InstanceData>() * instance_capacity as usize) as wgpu::BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            chunk_blocks,
            instances_to_render,

            chunk_id_x: idx,
            chunk_id_z: idz,
            alive_blocks,

            instance_size: 0,
            instance_capacity,
            instance_buf,
            instance_staging_buf,

            instances_modified: false,
        }
    }


    pub fn get_instance_capacity(&self) -> u32 {
        self.instance_capacity
    }

    pub fn get_instance_size(&self) -> u32 {
        self.instance_size
    }
}




// this will store all data related to a instance so i can move this into the buffer
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct InstanceData {
    pub modelMatrix: [[f32; 4]; 4],
    pub colour: [f32; 4],
}



