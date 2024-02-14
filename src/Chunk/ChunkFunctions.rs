
use std::collections::HashSet;
use std::mem;
use std::sync::{Arc, Mutex};

use crate::Block::*;
use crate::Settings::*;
use crate::FileSystem::*;
use crate::Renderer::*;
use crate::Chunk::InstanceData;

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
        if createdChunks.contains(&(self.chunk_id_x, self.chunk_id_z)) {

            // now check if it is loaded, if it is then i can just ignore it, if it isnt loaded then i need to read it from a file
            // it will already be loaded if i try to load a chunk already in the game
            // has been created before so load from file
            filesystem.ReadChunkFromFile(&mut tempChunkVec, self.chunk_id_x, self.chunk_id_z);

        } else {
            // else create a new one
            self.GenerateChunk(&mut tempChunkVec);

            // add this chunk to created chunks
            createdChunks.insert((self.chunk_id_x, self.chunk_id_z));
        }

        // check each block if it is touching air (async because reading from gpu is async)
        task::block_on(self.check_for_touching_air(&mut tempChunkVec, &renderer));


        // fill the chunkBlocks hashmap from the temp vector
        self.FillChunksHashMap(tempChunkVec);

        // TODO: #99 upload the instance buffer to the gpu

    }


    // convert the temp chunks vector into the hashmap
    pub fn FillChunksHashMap(&mut self, tempChunkVec: Vec<Vec<Vec<Block>>>) {

        // loop through the temp vector and fill the hashmap
        for x in 0..chunkSizeX {
            for y in 0..chunkSizeY {
                for z in 0..chunkSizeZ {
                    // if the block is not air then add it to the hashmap
                    if tempChunkVec[x][y][z].blockType != BlockType::Air {

                        self.chunk_blocks.insert(
                            (tempChunkVec[x][y][z].position.x, tempChunkVec[x][y][z].position.y, tempChunkVec[x][y][z].position.z), 
                            tempChunkVec[x][y][z]
                        );
                        self.alive_blocks += 1;

                        // also if it is touching air then add it to the instances to render hashmap
                        if tempChunkVec[x][y][z].touchingAir {
                            self.instances_to_render.insert(
                                (tempChunkVec[x][y][z].position.x, tempChunkVec[x][y][z].position.y, tempChunkVec[x][y][z].position.z), 
                                InstanceData { 
                                    modelMatrix: tempChunkVec[x][y][z].modelMatrix.clone(),
                                    colour: tempChunkVec[x][y][z].blockType.BlockColour(),
                                }
                            );
                        }
                    }
                }
            }
        }
    }



    // iterate through the whole vector and update all blocks that are touching air using a compute shader
    // this is run once on chunk creation
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

        all buffers need to be of type u32. booleans dont work between rust and gpu
        and wgsl only supports these types: f32, i32, u32, bool

        */

        // create the block types array
        let mut chunk_block_types: Vec<u32> = Vec::with_capacity(chunkSizeX * chunkSizeY * chunkSizeZ);
        for z in 0..chunkSizeX as i32 {
            for y in 0..chunkSizeY as i16 {
                for x in 0..chunkSizeZ as i32 {
                    chunk_block_types.push(tempChunkVec[x as usize][y as usize][z as usize].blockType.ToInt() as u32);
                }
            }
        }

        // create the block transparancy array
        let mut chunk_block_transparency: Vec<u32> = Vec::with_capacity(chunkSizeX * chunkSizeY * chunkSizeZ);
        for z in 0..chunkSizeX as i32 {
            for y in 0..chunkSizeY as i16 {
                for x in 0..chunkSizeZ as i32 {
                    chunk_block_transparency.push(tempChunkVec[x as usize][y as usize][z as usize].blockType.is_transparent() as u32);
                }
            }
        }

        // create the dimentions buffer so the gpu knows the max of xyz
        let dimentions: [u32; 3] = [chunkSizeX as u32, chunkSizeY as u32, chunkSizeZ as u32];

        // now the resulting buffer (cant use bool with the gpu, since rust bools arnt guarenteed to be 1 byte)
        // so instead ill use u8 for all calculations on the gpu and then just convert it to a bool on the cpu once i recieve the results

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

        
        let result_buffer_size: wgpu::BufferAddress = ((chunkSizeX * chunkSizeY * chunkSizeZ) * mem::size_of::<u32>()) as wgpu::BufferAddress;
        let result_buffer = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Result compute buffer"),
            size: result_buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });


        // load the shader

        // create the bind group and pipeline
        // Instantiates the bind group, once again specifying the binding of buffers.

        let bind_group_layout = renderer.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                // Specify your bindings here
                // block_type_buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Storage { read_only: true }, 
                        has_dynamic_offset: false, 
                        min_binding_size: None, 
                    },
                    count: None,
                },

                // block_transparency_buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Storage { read_only: true }, 
                        has_dynamic_offset: false, 
                        min_binding_size: None, 
                    },
                    count: None,
                },

                // dimentions_buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Storage { read_only: true }, 
                        has_dynamic_offset: false, 
                        min_binding_size: None, 
                    },
                    count: None,
                },

                // result_buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Storage { read_only: false }, 
                        has_dynamic_offset: false, 
                        min_binding_size: None, 
                    },
                    count: None,
                },
            ],
        });

        let bind_group = renderer.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: block_type_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: block_transparency_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: dimentions_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: result_buffer.as_entire_binding(),
                },
            ],
        });

        let pipeline_layout = renderer.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = renderer.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("check for air Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &renderer.check_air_compute_Shader_code,
            entry_point: "main",
        });


        // run the shader
        let mut encoder = renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
             label: Some("check air compute encoder") 
            });
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("check air compute pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups((chunkSizeX * chunkSizeY * chunkSizeZ) as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }

        // submit the compute shader render pass command
        renderer.queue.submit(Some(encoder.finish()));


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
        
        // make a fence so i know once the copy buffer function has finished
        // Create a shared variable
        let fence = Arc::new(Mutex::new(0));

        // Create a clone of the Arc for the callback
        let fence_clone = Arc::clone(&fence);

        // Set up a callback to run when the submitted work is done
        renderer.queue.on_submitted_work_done( move || {
            // This code will run when all work submitted to the queue up to this point has completed
            let mut fence_guard = fence_clone.lock().unwrap();
            *fence_guard = 1;
        });

        // submit the copy buffer command and the fence submitted work done command
        renderer.queue.submit(Some(encoder.finish()));

        // loop until the fence is 1 meaning the copy buffer is done
        // normally i would have this checked once per frame not in a busy waiting loop
        loop {
            // Poll the device to process outstanding work
            renderer.device.poll(wgpu::Maintain::Poll);

            // Check if the fence has been set to 1
            let mut fence_guard = fence.lock().unwrap();
            if *fence_guard == 1 {
                break;
            }
        }
        

        // now the result buffer will have finished being copied over


        let buffer_slice = read_result_buffer.slice(..);

        // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
        let (sender, receiver) = flume::bounded(1);
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        // poll the device waiting for the may async to finish since it is the only command in the queue
        renderer.device.poll(wgpu::Maintain::Wait).panic_on_timeout();


        // Awaits until `buffer_future` can be read from once the callback is run
        let result = if let Ok(Ok(())) = receiver.recv_async().await {

            // Gets contents of buffer
            let data = buffer_slice.get_mapped_range();

            // Since contents are got in bytes, this converts these bytes back to u32
            let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();

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
        /*
        println!("testing results");

        println!("length of results: {}", result.len());
        println!("length of temp: {}", chunkSizeX * chunkSizeY * chunkSizeZ);
        // print the blocks and then next to it the touching air result
        for y in 0..chunkSizeY {
            for z in 0..chunkSizeZ {
                for x in 0..chunkSizeX {
                    print!("{} ", tempChunkVec[x][y][z].blockType.ToInt());
                }

                // for spacing
                print!("   ");

                for x in 0..chunkSizeX {
                    let index: usize = x + (y * chunkSizeX) + (z * chunkSizeX * chunkSizeY);
                    print!("{} ", result[index]);
                }

                println!();
            }
            println!();
        }
        */


        // update the blocks with the results
        let mut index: usize;
        for y in 0..chunkSizeY {
            for z in 0..chunkSizeZ {
                for x in 0..chunkSizeX {
                    index = x + (y * chunkSizeX) + (z * chunkSizeX * chunkSizeY);
                    tempChunkVec[x][y][z].touchingAir = result[index] != 0;
                }
            }
        }

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