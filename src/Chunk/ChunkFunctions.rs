
use std::collections::HashSet;
use std::mem;

use crate::Block::*;
use crate::Settings::*;
use crate::FileSystem::*;
use crate::Renderer::*;

use wgpu::util::DeviceExt;

use flume;
use async_std::task;



impl super::Chunk {
    
    // if this chunk has beenc created before then i create a Chunk obj, and fill it from wherever
    pub fn LoadChunk(&mut self, filesystem: &mut FileSystem, createdChunks: &mut HashSet<(i32, i32)>, renderer: &Renderer) {

        // create the temp chunk Vector, which creates all blocks
        let mut tempChunkVec: Vec<Vec<Vec<Block>>> = self.CreateTempChunkVector();

        // fill the temp vector with data
        // first check if the chunk has been created before if so load it
        if createdChunks.contains(&(self.chunkIDx, self.chunkIDz)) {

            // now check if it is loaded, if it is then i can just ignore it, if it isnt loaded then i need to read it from a file
            // it will already be loaded if i try to load a chunk already in the game
            // has been created before so load from file
            filesystem.ReadChunkFromFile(&mut tempChunkVec, self.chunkIDx, self.chunkIDz);

        } else {
            // else create a new one
            self.GenerateChunk(&mut tempChunkVec);

            // add this chunk to created chunks
            createdChunks.insert((self.chunkIDx, self.chunkIDz));
        }

        // check each block if it is touching air (async because reading from gpu is async)
        task::block_on(self.check_for_touching_air(&mut tempChunkVec, &renderer));


        // fill the chunkBlocks hashmap from the temp vector
        self.FillChunksHashMap(tempChunkVec);

    }


    // convert the temp chunks vector into the hashmap
    pub fn FillChunksHashMap(&mut self, tempChunkVec: Vec<Vec<Vec<Block>>>) {

        // loop through the temp vector and fill the hashmap
        for x in 0..chunkSizeX {
            for y in 0..chunkSizeY {
                for z in 0..chunkSizeZ {
                    // if the block is not air then add it to the hashmap
                    if tempChunkVec[x][y][z].blockType != BlockType::Air {

                        self.chunkBlocks.insert(
                            (tempChunkVec[x][y][z].position.x, tempChunkVec[x][y][z].position.y, tempChunkVec[x][y][z].position.z), 
                            tempChunkVec[x][y][z]
                        );
                        self.aliveBlocks += 1;
                    }
                }
            }
        }
    }



    // iterate through the whole vector and update all blocks that are touching air using a compute shader
    pub async fn check_for_touching_air(&mut self, tempChunkVec: &mut Vec<Vec<Vec<Block>>>, renderer: &Renderer) {

        /*
        create 2 buffers
        buffer 1: hold all block types (as u16)
        buffer 2: hold a bool transperency value of that block
        
        each buffer will also have a boarder around 1 thick
        and ill put the touching layer of the next chunk there so it can calulate on that too
        if the next chunk doesnt exist then ill just put void there.

        buffer 2 is needed so i can easily check if a block is touching air, instead of trying to iterate though
        an array checking the block against all block types that are transparent

        the buffer will just hold the block type, exact position isnt necessary just relative to other blocks is,
        which is its index

        the gpus output buffer is a boolean buffer of true if touching air and false otherwise

        */

        // create the block types array
        let mut chunk_block_types: Vec<u16> = Vec::with_capacity(chunkSizeX * chunkSizeY * chunkSizeZ);
        for z in 0..chunkSizeX as i32 {
            for y in 0..chunkSizeY as i16 {
                for x in 0..chunkSizeZ as i32 {
                    chunk_block_types.push(tempChunkVec[x as usize][y as usize][z as usize].blockType.ToInt());
                }
            }
        }

        // create the block transparancy array
        let mut chunk_block_transparency: Vec<bool> = Vec::with_capacity(chunkSizeX * chunkSizeY * chunkSizeZ);
        for z in 0..chunkSizeX as i32 {
            for y in 0..chunkSizeY as i16 {
                for x in 0..chunkSizeZ as i32 {
                    chunk_block_transparency.push(tempChunkVec[x as usize][y as usize][z as usize].blockType.is_transparent());
                }
            }
        }

        // create the dimentions buffer so the gpu knows the max of xyz
        let dimentions: [u32; 3] = [chunkSizeX as u32, chunkSizeY as u32, chunkSizeZ as u32];

        // now the resulting buffer (cant use bool with the gpu, since rust bools arnt guarenteed to be 1 byte)
        // so instead ill use u8 for all calculations on the gpu and then just convert it to a bool on the cpu once i recieve the results
        let mut result: Vec<u8> = vec!(0; chunkSizeX * chunkSizeY * chunkSizeZ);


        // now create the gpu buffers for both of these
        let block_type_buffer = renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("block type compute buffer"),
                contents: bytemuck::cast_slice(&chunk_block_types),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });

        let block_transparency_buffer = renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("block transparency compute buffer"),
                contents: bytemuck::cast_slice(&chunk_block_transparency),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });
        
        let dimentions_buffer = renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("block transparency compute buffer"),
                contents: bytemuck::cast_slice(&dimentions),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });

        
        let result_buffer_size: u64 = ((chunkSizeX * chunkSizeY * chunkSizeZ) * mem::size_of::<u8>()) as u64;
        let result_buffer = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Result compute buffer"),
            size: result_buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // create the bind group


        // run the shader


        // get the results from the shader
        // make the buffer ill read from
        let read_result_buffer = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Read Buffer"),
            size: result_buffer_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        let mut encoder = renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("read buffer copy encoder"),
        });
        
        // copy the data from the gpu data to my reading buffer
        encoder.copy_buffer_to_buffer(&result_buffer, 0, &read_result_buffer, 0, result_buffer_size);
        
        // submit the command
        renderer.queue.submit(Some(encoder.finish()));


        let buffer_slice = read_result_buffer.slice(..);

        // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
        let (sender, receiver) = flume::bounded(1);
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        // Poll the device in a blocking manner so that our future resolves.
        // In an actual application, `device.poll(...)` should
        // be called in an event loop or on another thread.
        renderer.device.poll(wgpu::Maintain::wait()).panic_on_timeout();


        // Awaits until `buffer_future` can be read from once the callback is run
        let result = if let Ok(Ok(())) = receiver.recv_async().await {
            // Gets contents of buffer
            let data = buffer_slice.get_mapped_range();

            // Since contents are got in bytes, this converts these bytes back to u32
            let result: Vec<u8> = bytemuck::cast_slice(&data).to_vec();

            // With the current interface, we have to make sure all mapped views are
            // dropped before we unmap the buffer.
            drop(data);
            read_result_buffer.unmap(); // Unmaps buffer from memory
                                    // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                    //   delete myPointer;
                                    //   myPointer = NULL;
                                    // It effectively frees the memory

            // Returns data from buffer
            result
        } else {
            panic!("failed to run compute on gpu!")
        };

        // print the results to check that they are right


        // update the blocks with the results


    }
}


// takes the world position and gives you the chunk id of the chunk that position is in
pub fn GetChunkId(posX: i32, posZ: i32) -> (i32, i32) {

    let chunkX: i32 = posX / chunkSizeX as i32;
    let chunkZ: i32 = posZ / chunkSizeZ as i32;

    return (chunkX, chunkZ);
}


// given a chunk x and z id and a block position relative to the origin of the chunk return the world coordinate of the block
pub fn GetWorldBlockPos(blockIDx: i32, blockIDz: i32, relBlockX: i32, relBlockY: i16, relBlockZ: i32) -> (i32, i16, i32) {
    let worldX: i32 = (blockIDx * chunkSizeX as i32) + relBlockX;
    let worldY: i16 = relBlockY;
    let worldZ: i32 = (blockIDz * chunkSizeZ as i32) + relBlockZ;

    return (worldX, worldY, worldZ);
}


// go from blocks world position to chunk relative position
pub fn GetRelativeBlockPos(worldX: i32, worldY: i16, worldZ: i32) -> (i32, i16, i32) {

    /* this does the same thing, as below, below is just obviously more efficient
    let mut chunkRelativeX: i32 = 0;
    let mut chunkRelativeZ: i32 = 0;
    let maxX: i32 = chunkSizeX as i32;
    let maxZ: i32 = chunkSizeZ as i32;

    if worldX >= 0 {
        chunkRelativeX = worldX % chunkSizeX as i32;
    } else {
        chunkRelativeX = (maxX - (worldX % maxX).abs()) % maxX;
    }

    if worldZ >= -(chunkRelativeZ as i32) {
        chunkRelativeZ = worldZ % chunkSizeZ as i32;
    } else {
        chunkRelativeZ = (maxZ - (worldZ % maxZ).abs()) % maxZ;
    }
    */
    
    let chunkRelativeX: i32 = worldX.rem_euclid(chunkSizeX as i32);
    let chunkRelativeZ: i32 = worldZ.rem_euclid(chunkSizeZ as i32);


    return (chunkRelativeX, worldY, chunkRelativeZ);
}