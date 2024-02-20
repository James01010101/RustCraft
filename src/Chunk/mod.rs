pub mod chunk_functions;
pub mod chunk_gpu_function;
pub mod create_chunks;

use crate::{block::*, renderer::*, types::*};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wgpu::{BufferDescriptor, BufferUsages};

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

    // the size of the new buffers, so i can compare to the old size
    pub new_instance_capacity: u32,

    // the actual size of the instance buffer being used, (size of hashmap once updated)
    pub instance_size: u32,

    // instance buffer for the chunk
    pub instance_buffer: wgpu::Buffer,
    pub instance_staging_buffer: wgpu::Buffer,

    pub new_instance_buffer: wgpu::Buffer,
    pub new_instance_staging_buffer: wgpu::Buffer,


    // if i update the staging buffer set this true so i know to copy it to the instance buffer
    pub instances_modified: bool,

    // so i know if the staging buffer is currently being written to
    // i only update the actual buffer is staging buffer write is false and modified is true
    pub staging_buffer_writing: Arc<Mutex<bool>>,

    // if i am making new buffers of a larger size i wont push anymore commands or changes to the old buffers
    pub new_instance_buffers_writing: Arc<Mutex<bool>>,

    // used on the cpu side to overwrite the old buffers once the new ones have been update with data
    pub creating_new_instance_buffers: bool,
}

impl Chunk {
    pub fn new(idx: i32, idz: i32, num_blocks: i32, renderer: &Renderer) -> Chunk {
        // if a numBlocks was passed in ill allocate the hashmap of that size
        let chunk_blocks: HashMap<(i32, i16, i32), Block>;
        let instances_to_render: HashMap<(i32, i16, i32), InstanceData> = HashMap::new();
        let alive_blocks: u32;

        if num_blocks != -1 {
            chunk_blocks = HashMap::with_capacity(num_blocks as usize);
            alive_blocks = num_blocks as u32;
        } else {
            // if the number of blocks needed is unknown ill just let it do its thing
            chunk_blocks = HashMap::new();
            alive_blocks = 0;
        }

        // make the instance buffer for the chunk init it with size of 100
        let instance_capacity: u32 = 100;
        let instance_buffer: wgpu::Buffer = renderer.device.create_buffer(&BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (std::mem::size_of::<InstanceData>() * instance_capacity as usize)
                as wgpu::BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let instance_staging_buffer: wgpu::Buffer = renderer.device.create_buffer(&BufferDescriptor {
                label: Some("Instance Staging Buffer"),
                size: (std::mem::size_of::<InstanceData>() * instance_capacity as usize)
                    as wgpu::BufferAddress,
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            });

        let new_instance_buffer: wgpu::Buffer = renderer.device.create_buffer(&BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (std::mem::size_of::<InstanceData>() * instance_capacity as usize)
                as wgpu::BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let new_instance_staging_buffer: wgpu::Buffer = renderer.device.create_buffer(&BufferDescriptor {
                label: Some("Instance Staging Buffer"),
                size: (std::mem::size_of::<InstanceData>() * instance_capacity as usize)
                    as wgpu::BufferAddress,
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
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
            new_instance_capacity: instance_capacity,

            instance_buffer,
            instance_staging_buffer,
            new_instance_buffer,
            new_instance_staging_buffer,

            instances_modified: false,
            staging_buffer_writing: Arc::new(Mutex::new(false)),
            new_instance_buffers_writing: Arc::new(Mutex::new(false)),
            creating_new_instance_buffers: false,
        }
    }

    pub fn get_instance_capacity(&self) -> u32 {
        self.instance_capacity
    }

    pub fn get_instance_size(&self) -> u32 {
        self.instance_size
    }
}
