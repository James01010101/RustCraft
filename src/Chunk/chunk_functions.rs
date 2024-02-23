use crate::{block::*, block_type::*, file_system::*, renderer::*, types::*, world::*, chunk::create_chunks::generate_chunk};

use async_std::task;

impl super::Chunk {
    // if this chunk has beenc created before then i create a Chunk obj, and fill it from wherever
    pub fn load_chunk(
        &mut self,
        file_system: &mut FileSystem,
        world: &mut World,
        renderer: &Renderer,
    ) {
        // create the temp chunk Vector, which creates all blocks
        let mut temp_chunk_vec: Vec<Vec<Vec<Block>>> = create_temp_chunk_vector((self.chunk_id_x, self.chunk_id_z), (world.chunk_size_x, world.chunk_size_y, world.chunk_size_z));

        // fill the temp vector with data
        // first check if the chunk has been created before if so load it
        if world
            .created_chunks
            .contains(&(self.chunk_id_x, self.chunk_id_z))
        {
            // now check if it is loaded, if it is then i can just ignore it, if it isnt loaded then i need to read it from a file
            // it will already be loaded if i try to load a chunk already in the game
            // has been created before so load from file
            file_system.read_chunks_from_file(
                &mut temp_chunk_vec,
                self.chunk_id_x,
                self.chunk_id_z,
                &world,
            );
        } else {
            // else create a new one
            generate_chunk(&mut temp_chunk_vec, (world.chunk_size_x, world.chunk_size_y, world.chunk_size_z), world.chunk_size_y / 2);

            // add this chunk to created chunks
            world
                .created_chunks
                .insert((self.chunk_id_x, self.chunk_id_z));
        }

        // check each block if it is touching air (async because reading from gpu is async)
        task::block_on(self.check_for_touching_air(&mut temp_chunk_vec, &renderer, &world));

        // fill the chunkBlocks hashmap from the temp vector
        self.fill_chunk_hashmap(temp_chunk_vec, &world);

        // update the number of alive blocks
        self.alive_blocks = self.chunk_blocks.len() as u32;

        // update instance size
        self.instance_size = self.instances_to_render.len() as u32;
        if self.instance_size > self.instance_capacity {
            self.update_instance_buffers_capacity(renderer);
        }

        self.update_instance_staging_buffer(renderer);
    }

    // convert the temp chunks vector into the hashmap
    pub fn fill_chunk_hashmap(&mut self, temp_chunk_vec: Vec<Vec<Vec<Block>>>, world: &World) {
        // loop through the temp vector and fill the hashmap
        for x in 0..world.chunk_size_x {
            for y in 0..world.chunk_size_y {
                for z in 0..world.chunk_size_z {
                    // if the block is not air then add it to the hashmap
                    if temp_chunk_vec[x][y][z].block_type != BlockType::Air {
                        self.chunk_blocks.insert(
                            (
                                temp_chunk_vec[x][y][z].position.x,
                                temp_chunk_vec[x][y][z].position.y,
                                temp_chunk_vec[x][y][z].position.z,
                            ),
                            temp_chunk_vec[x][y][z],
                        );
                        self.alive_blocks += 1;

                        // also if it is touching air then add it to the instances to render hashmap
                        if temp_chunk_vec[x][y][z].touching_air {
                            self.instances_to_render.insert(
                                (
                                    temp_chunk_vec[x][y][z].position.x,
                                    temp_chunk_vec[x][y][z].position.y,
                                    temp_chunk_vec[x][y][z].position.z,
                                ),
                                InstanceData {
                                    model_matrix: temp_chunk_vec[x][y][z].model_matrix.clone(),
                                    colour: temp_chunk_vec[x][y][z].block_type.block_colour(),
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    // this is called on each chunk per frame so i can do updates if needed
    pub fn update(&mut self, renderer: &Renderer) {
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
            self.update_instance_buffer(renderer);
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

// takes the world position and gives you the chunk id of the chunk that position is in
pub fn get_chunk_id(pos_x: i32, pos_z: i32, world: &World) -> (i32, i32) {
    let chunk_x: i32 = pos_x / (world.chunk_size_x as i32);
    let chunk_z: i32 = pos_z / (world.chunk_size_z as i32);

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
    let world_x: i32 = chunk_id_x * (chunk_sizes.0 as i32) + relative_block_x;
    let world_y: i16 = relative_block_y;
    let world_z: i32 = chunk_id_z * (chunk_sizes.2 as i32) + relative_block_z;

    return (world_x, world_y, world_z);
}

// go from blocks world position to chunk relative position
pub fn get_relative_block_pos(
    world_x: i32,
    world_y: i16,
    world_z: i32,
    world: &World,
) -> (i32, i16, i32) {
    let chunk_relative_x: i32 = world_x.rem_euclid(world.chunk_size_x as i32);
    let chunk_relative_z: i32 = world_z.rem_euclid(world.chunk_size_z as i32);

    return (chunk_relative_x, world_y, chunk_relative_z);
}
