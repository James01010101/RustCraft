use crate::{block::*, block_type::*, file_system::*, renderer::*, types::*, world::*};

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
        let mut temp_chunk_vec: Vec<Vec<Vec<Block>>> = self.create_temp_chunk_vector(&world);

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
            self.generate_chunk(&mut temp_chunk_vec, &world);

            // add this chunk to created chunks
            world
                .created_chunks
                .insert((self.chunk_id_x, self.chunk_id_z));
        }

        // check each block if it is touching air (async because reading from gpu is async)
        task::block_on(self.check_for_touching_air(&mut temp_chunk_vec, &renderer, &world));

        // fill the chunkBlocks hashmap from the temp vector
        self.fill_chunk_hashmap(temp_chunk_vec, &world);

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

        self.instance_size = self.instances_to_render.len() as u32;
    }

    // this is called on each chunk per frame so i can do updates if needed
    pub fn update(&mut self, renderer: &Renderer) {
        // first check if there is a new instance buffer to be updated
        if self.creating_new_instance_buffers {
            // check if the new instance buffer is finished being written to
            // this is needed because locking the mutex returns a immutable reference of self
            // and is still alive when i want to pass a mutable reference to the overwrite function
            // so i need to set it to another variable and drop the mutex before i can use self again
            let temp_instance_writing_bool: bool;
            {
                let new_instance_buffers_writing =
                    self.new_instance_buffers_writing.lock().unwrap();
                temp_instance_writing_bool = *new_instance_buffers_writing;
            }
            if !temp_instance_writing_bool {
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
    world: &World,
) -> (i32, i16, i32) {
    let world_x: i32 = chunk_id_x * (world.chunk_size_x as i32) + relative_block_x;
    let world_y: i16 = relative_block_y;
    let world_z: i32 = chunk_id_z * (world.chunk_size_z as i32) + relative_block_z;

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
