
use crate::{
    block::*, 
    block_type::*, 
    file_system::*, 
    types::*, 
    chunk::create_chunks::generate_chunk,
    chunk::chunk_gpu_functions::check_for_touching_air,
};

use wgpu::{Device, Queue, ShaderModule};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

impl super::Chunk {
    // if this chunk has beenc created before then i create a Chunk obj, and fill it from wherever
    pub fn load_chunk(
        &mut self,
        file_system: Arc<Mutex<FileSystem>>,
        device: Arc<Mutex<Device>>,
        queue: Arc<Mutex<Queue>>,
        check_air_compute_shader_code: Arc<Mutex<ShaderModule>>,
        chunk_sizes: (usize, usize, usize),
        created_chunks: Arc<Mutex<HashSet<(i32, i32)>>>,
    ) -> (Vec<Vec<Vec<Block>>>, Arc<Mutex<i32>>, wgpu::Buffer) {
        // create the temp chunk Vector, which creates all blocks
        let mut temp_chunk_vec: Vec<Vec<Vec<Block>>> = create_temp_chunk_vector((self.chunk_id_x, self.chunk_id_z), chunk_sizes);

        // fill the temp vector with data
        // first check if the chunk has been created before if so load it
        let mut created_chunks_locked = created_chunks.lock().unwrap();
        let chunk_created: bool = created_chunks_locked.contains(&(self.chunk_id_x, self.chunk_id_z));
        drop(created_chunks_locked); // so i dont hold the lock for it and main can use it

        if chunk_created {
            // now check if it is loaded, if it is then i can just ignore it, if it isnt loaded then i need to read it from a file
            // it will already be loaded if i try to load a chunk already in the game
            // has been created before so load from file
            let mut file_system_locked = file_system.lock().unwrap();
            file_system_locked.read_chunks_from_file(
                &mut temp_chunk_vec,
                self.chunk_id_x,
                self.chunk_id_z,
                chunk_sizes,
            );
            drop(file_system_locked);
        } else {
            // else create a new one
            generate_chunk(&mut temp_chunk_vec, chunk_sizes);

            // add this chunk to created chunks
            let mut created_chunks_locked = created_chunks.lock().unwrap();
            created_chunks_locked.insert((self.chunk_id_x, self.chunk_id_z));
            drop(created_chunks_locked); // so i dont hold the lock for it and main can use it
        }


        // check each block if it is touching air, this just starts the calculation but doesnt hold until its finished
        let (compute_shader_fence, read_buffer) = check_for_touching_air(
            &mut temp_chunk_vec, 
            device.clone(), 
            queue.clone(), 
            check_air_compute_shader_code.clone(), 
            chunk_sizes
        );

        // return back from here until the check air compute shader is finished
        return (temp_chunk_vec, compute_shader_fence, read_buffer);
    }

    

    // this is called on each chunk per frame so i can do updates if needed
    pub fn update(&mut self, device: Arc<Mutex<Device>>, queue: Arc<Mutex<Queue>>) {
        // first check if there is a new instance buffer to be updated
        if self.creating_new_instance_buffers {
            // check if the new instance buffer is finished being written to
            // this is needed because locking the mutex returns a immutable reference of self
            // and is still alive when i want to pass a mutable reference to the overwrite function
            // so i need to set it to another variable and drop the mutex before i can use self again

            let new_instance_buffers_writing = self.new_instance_buffers_writing.lock().unwrap();
            if !*new_instance_buffers_writing {
                // need to drop the mutex since it is an immutable borrow. and this function need a mut borrow
                drop(new_instance_buffers_writing);
                // its finished writing so i can overwrite the old one now
                self.overwrite_old_instance_buffers();
            }

            // set creating new instance buffers to false
        } else {
            // if i am not currently creating new instance buffers
            self.update_instance_buffer(device.clone(), queue.clone());
        }
    }
}

/*
create a temporary 3d vector which will hold all of the blocks including air
this will create all the blocks give them their position and will calculate their model matrix
this will be returned and this is what the generate chunk will work on to calculate and modify the chunk as it is generated
im using this temp vector so i can easily change the values of the blocks and also change blocks from air to block and vice versa
then later ill convert from the vec to a hashmap which will only store blocks that arnt air to save space.
*/
pub fn create_temp_chunk_vector(chunk_ids: (i32, i32), chunk_sizes: (usize, usize, usize)) -> Vec<Vec<Vec<Block>>> {
    // initial xz values are defined by the chunks id,
    let mut temp_chunk_vec: Vec<Vec<Vec<Block>>> = Vec::with_capacity(chunk_sizes.0);

    for x in 0..chunk_sizes.0 as i32 {
        let mut temp2d: Vec<Vec<Block>> = Vec::with_capacity(chunk_sizes.1);

        for y in 0..chunk_sizes.1 as i16 {
            let mut temp1d: Vec<Block> = Vec::with_capacity(chunk_sizes.2);

            for z in 0..chunk_sizes.2 as i32 {
                let block_pos: (i32, i16, i32) = get_world_block_pos(
                    chunk_ids.0,
                    chunk_ids.1,
                    x,
                    y - (chunk_sizes.1 as i16 / 2),
                    z,
                    chunk_sizes,
                );

                temp1d.push(Block::new(
                    BlockType::Air,
                    block_pos.0,
                    block_pos.1,
                    block_pos.2,
                ));
            }

            temp2d.push(temp1d);
        }

        temp_chunk_vec.push(temp2d);
    }

    return temp_chunk_vec;
}


// once the temp chunk vector has all the blocks in it correctly ill fill them into the hashmap to save space on non air blocks
pub fn fill_chunk_hashmap( 
    chunk_blocks: &mut HashMap<(i32, i16, i32), Block>, 
    instances_to_render: &mut HashMap<(i32, i16, i32), InstanceData>,
    temp_chunk_vec: Vec<Vec<Vec<Block>>>, 
    chunk_sizes: (usize, usize, usize)
) {

    // cache each vector so i dont have to do 3 indexes each time
    let mut cached_vec_x: &Vec<Vec<Block>>;
    let mut cached_vec_y: &Vec<Block>;
    let mut cached_block: &Block;
    let mut cached_position: &Position;
    let mut cached_key: (i32, i16, i32);

    let max_elements: usize = chunk_sizes.0 * chunk_sizes.1 * chunk_sizes.2;

    let mut blocks_to_insert: Vec<((i32, i16, i32), Block)> = Vec::with_capacity(max_elements);
    let mut instances_to_insert: Vec<((i32, i16, i32), InstanceData)> = Vec::with_capacity(max_elements);

    // loop through the temp vector and fill the hashmap
    for x in 0..chunk_sizes.0 {
        cached_vec_x = &temp_chunk_vec[x];

        for y in 0..chunk_sizes.1 {
            cached_vec_y = &cached_vec_x[y];

            for z in 0..chunk_sizes.2 {
                cached_block = &cached_vec_y[z];

                // if the block is not air then add it to the hashmap
                if cached_block.block_type != BlockType::Air {
                    cached_position = &cached_block.position;
                    cached_key = (cached_position.x, cached_position.y, cached_position.z);

                    // and if it is touching air then add it to the instances to render hashmap
                    if cached_block.is_touching_air {
                        instances_to_insert.push((cached_key,
                            InstanceData {
                                model_matrix: cached_block.model_matrix.clone(),
                                colour: cached_block.block_type.block_colour(),
                            }
                        ));
                    }

                    blocks_to_insert.push((cached_key, *cached_block));
                }
            }
        }
    }

    // now reserve exactly the amount of memory needed and fill the hashmaps
    chunk_blocks.reserve(blocks_to_insert.len());
    for i in 0..blocks_to_insert.len() {
        chunk_blocks.insert(blocks_to_insert[i].0, blocks_to_insert[i].1);
    }

    instances_to_render.reserve(instances_to_insert.len());
    for i in 0..instances_to_insert.len() {
        instances_to_render.insert(instances_to_insert[i].0, instances_to_insert[i].1);
    }
}



// takes the world position and gives you the chunk id of the chunk that position is in
pub fn get_chunk_id(pos_x: i32, pos_z: i32, chunk_sizes: (usize, usize, usize)) -> (i32, i32) {
    let chunk_x: i32 = pos_x / (chunk_sizes.0 as i32);
    let chunk_z: i32 = pos_z / (chunk_sizes.2 as i32);

    return (chunk_x, chunk_z);
}

// given a chunk x and z id and a block position relative to the origin of the chunk return the world coordinate of the block
pub fn get_world_block_pos(
    chunk_id_x: i32,
    chunk_id_z: i32,
    relative_block_x: i32,
    relative_block_y: i16,
    relative_block_z: i32,
    chunk_sizes: (usize, usize, usize),
) -> (i32, i16, i32) {
    let world_x: i32 = (chunk_id_x * chunk_sizes.0 as i32) + relative_block_x;
    let world_y: i16 = relative_block_y;
    let world_z: i32 = (chunk_id_z * chunk_sizes.2 as i32) + relative_block_z;

    return (world_x, world_y, world_z);
}

// go from blocks world position to chunk relative position
pub fn get_relative_block_pos(
    world_x: i32,
    world_y: i16,
    world_z: i32,
    chunk_sizes: (usize, usize, usize),
) -> (i32, i16, i32) {
    let chunk_relative_x: i32 = world_x.rem_euclid(chunk_sizes.0 as i32);
    let chunk_relative_z: i32 = world_z.rem_euclid(chunk_sizes.2 as i32);

    return (chunk_relative_x, world_y, chunk_relative_z);
}
