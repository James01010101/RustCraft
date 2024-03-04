
// this file is for the chunk generation thread to run
use std::{
    collections::{VecDeque, HashMap, HashSet},
    sync::{Arc, Mutex},
    //thread,
    //time::Duration,
};

use crate::{
    block::*,
    chunk::{chunk_functions::*, Chunk},
    file_system::*,
    world::*,
};


use wgpu::BufferAsyncError;


// this will store all info about a chunk being generates. like how far along it is in being processed
pub struct ChunkGenerationData {

    // the actual chunk itself
    pub chunk: Chunk,

    // the chunk id ill unload later
    pub unload_chunk_ids: Option<(i32, i32)>,

    pub temp_chunk_vector: Vec<Vec<Vec<Block>>>,

    // this fence will tell me once the shader has finished running
    pub compute_shader_fence: Arc<Mutex<i32>>,
    pub read_buffer: wgpu::Buffer, // this is where the result of the compute shader will be stored

    pub buffer_slice: Option<wgpu::BufferSlice<'static>>,
    // so i know once ive started to i dont start it twice
    pub start_buffer_mapping: bool,

    // option of none until it is finished then either ok or an error
    pub mapping_state: Arc<Mutex<Option<Result<(), BufferAsyncError>>>>,

    // is the vec finished and can be inserted into the hashmap and used in game
    pub temp_chunk_vector_finished: bool,
}


pub fn run_chunk_generation_thread(
    loading_chunks_queue: Arc<Mutex<VecDeque<((i32, i32), Option<(i32, i32)>)>>>,
    chunk_sizes: (usize, usize, usize),
    created_chunks: Arc<Mutex<HashSet<(i32, i32)>>>,
    device: Arc<Mutex<wgpu::Device>>,
    queue: Arc<Mutex<wgpu::Queue>>,
    check_air_compute_shader_code: Arc<Mutex<wgpu::ShaderModule>>,
    filesystem: Arc<Mutex<FileSystem>>,
    chunks: Arc<Mutex<HashMap<(i32, i32), Chunk>>>,
    continue_running: Arc<Mutex<bool>>,
) {


    // have some struct which holds the currently creating chunks
    // since there is stages where the chunks is doing gpu stuff i wont hold ill keep going and check back with it later
    let mut currently_creating_chunks: HashMap<(i32, i32), ChunkGenerationData> = HashMap::new();

    // this will store the chunks index here that are finished and ready to be inserted into the hashmap
    // i cant insert them into the hashmap while im iterating over it so i will store them here and insert them after the iteration
    let mut finished_chunks: Vec<(i32, i32)> = Vec::new();

    // keep checking for new chunks to load and generate
    loop {  

        // check continue running
        { // mutex lock scope
            if !*continue_running.lock().unwrap() { break; }
        }

        // check if there is something in the queue to do
        check_loading_chunks_queue(
            loading_chunks_queue.clone(),
            filesystem.clone(),
            device.clone(),
            queue.clone(),
            check_air_compute_shader_code.clone(),
            created_chunks.clone(),
            &mut currently_creating_chunks,
            chunk_sizes,
        );

        // poll the gpu to update anything i need
        { // mutex lock scope
            device.lock().unwrap().poll(wgpu::Maintain::Poll);
        }

        // keep going through the chunks that are being created and work on them until they are finished
        continue_generating_chunks(
            &mut currently_creating_chunks,
            device.clone(),
            queue.clone(),
            &mut finished_chunks,
            chunk_sizes,
        );
        

        // check through the finished chunks are finish them off
        // need to clone because im not getting references im getting the values so i cant do this multiple times for each loop
        finish_up_chunks(
            &mut finished_chunks,
            chunks.clone(),
            &mut currently_creating_chunks,
            filesystem.clone(),
            created_chunks.clone(),
            chunk_sizes,
        );
    } // end loop


    // wrap up the generation thread

    // unload and save all chunks to file
    { // mutex lock scope
        unload_all_chunks(
            &mut chunks.lock().unwrap(), 
            &mut filesystem.lock().unwrap(), 
            &mut created_chunks.lock().unwrap(), 
            chunk_sizes
        );
    }

}

pub fn unload_all_chunks(
    chunks: &mut HashMap<(i32, i32), Chunk>, 
    file_system: &mut FileSystem, 
    created_chunks: &mut HashSet<(i32, i32)>,
    chunk_sizes: (usize, usize, usize)
) {
    let hashmap_chunk_keys: Vec<(i32, i32)> = chunks.keys().cloned().collect();
    

    // go through each chunk and call unload on it
        for key in hashmap_chunk_keys {
        // remove the chunk from the hashmap and return it
        remove_chunk(
            chunks, 
            key, 
            file_system,
            chunk_sizes
        );
    }
    
    file_system.save_created_chunks_file(chunk_sizes, created_chunks);
}


pub fn check_loading_chunks_queue(
    loading_chunks_queue: Arc<Mutex<VecDeque<((i32, i32), Option<(i32, i32)>)>>>,
    filesystem: Arc<Mutex<FileSystem>>,
    device: Arc<Mutex<wgpu::Device>>,
    queue: Arc<Mutex<wgpu::Queue>>,
    check_air_compute_shader_code: Arc<Mutex<wgpu::ShaderModule>>,
    created_chunks: Arc<Mutex<HashSet<(i32, i32)>>>,
    currently_creating_chunks: &mut HashMap<(i32, i32), ChunkGenerationData>,
    chunk_sizes: (usize, usize, usize),
) {
    let element = { // mutex scope
        loading_chunks_queue.lock().unwrap().pop_front()
    };
    
    match element {
        Some((load_chunk_ids, unload_chunk_ids)) => {
            // load the chunk
            let mut new_chunk: Chunk = Chunk::new(load_chunk_ids.0, load_chunk_ids.1, -1, device.clone());
            let (temp_chunk_vector, compute_shader_fence, read_buffer) = new_chunk.load_chunk(
                filesystem.clone(), 
                device.clone(), 
                queue.clone(), 
                check_air_compute_shader_code.clone(), 
                chunk_sizes, 
                created_chunks.clone()
            );

            let new_chunk_data = ChunkGenerationData {
                chunk: new_chunk,
                unload_chunk_ids,
                temp_chunk_vector,
                compute_shader_fence,
                read_buffer,
                start_buffer_mapping: false,
                mapping_state: Arc::new(Mutex::new(None)),
                buffer_slice: None,
                temp_chunk_vector_finished: false,
            };

            // add this chunks data to the hashmap so i can check it later
            currently_creating_chunks.insert(load_chunk_ids, new_chunk_data);
        }
        None => {
            // nothing in the queue so ill sleep for a bit
            //thread::sleep(Duration::from_millis(10));
        }
    } // end match element
}


pub fn continue_generating_chunks(
    currently_creating_chunks: &mut HashMap<(i32, i32), ChunkGenerationData>,
    device: Arc<Mutex<wgpu::Device>>,
    queue: Arc<Mutex<wgpu::Queue>>,
    finished_chunks: &mut Vec<(i32, i32)>,
    chunk_sizes: (usize, usize, usize),
) {
    let keys: Vec<(i32, i32)> = currently_creating_chunks.keys().cloned().collect();
    for waiting_chunk_id in keys {
        let waiting_chunk_data: &mut ChunkGenerationData = currently_creating_chunks.get_mut(&waiting_chunk_id).expect("unable to get chunk from 'currently_creating_chunks' as they key doesnt exist");
        let chunk: &mut Chunk = &mut waiting_chunk_data.chunk;

        // Check if the compute fence has been set to 1 so the shader is finished running and results copied to the correct buffer
        let fence_value: i32 = { // mutex lock scope
            *waiting_chunk_data.compute_shader_fence.lock().unwrap()
        };

        if fence_value == 1 {
            // check if the buffer hasnt started being mapped yet
            if !waiting_chunk_data.start_buffer_mapping {
                // it hasnt so i need to start mapping the buffer
                waiting_chunk_data.buffer_slice = Some(waiting_chunk_data.read_buffer.slice(..));

                // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
                let mapping_state_clone = waiting_chunk_data.mapping_state.clone();
                waiting_chunk_data.buffer_slice.expect("Buffer Slice doesn't exist when it gets mapped")
                    .map_async(wgpu::MapMode::Read, move |v| {
                        // update this with the result once finished
                        *mapping_state_clone.lock().unwrap() = Some(v);
                    }
                );

                waiting_chunk_data.start_buffer_mapping = true;


            } else { // mapping has started
                
                if !waiting_chunk_data.temp_chunk_vector_finished {

                    // check if it has finished mapping (mapping state wont be None)
                    let state = { // mutex lock scope
                        (*waiting_chunk_data.mapping_state.lock().unwrap()).clone()
                    };
                    match state {
                        Some(r) => {
                            r.unwrap(); // if it is an error it will panic

                            // otherwise i can get the data from the buffer
                            let data = waiting_chunk_data.buffer_slice.expect("Buffer Slice doesn't exist when it gets mapped range").get_mapped_range();

                            // Since contents are got in bytes, this converts these bytes back to u32
                            let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();

                            // With the current interface, we have to make sure all mapped views are
                            // dropped before we unmap the buffer.
                            drop(data);
                            waiting_chunk_data.read_buffer.unmap(); // Unmaps buffer from memory
                                                        // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                                        //   delete myPointer;
                                                        //   myPointer = NULL;
                                                        // It effectively frees the memory

                            // set the touching air values
                            let mut index: usize;
                            for y in 0..chunk_sizes.1 {
                                for z in 0..chunk_sizes.2 {
                                    for x in 0..chunk_sizes.0 {
                                        index = x + (y * chunk_sizes.0) + (z * chunk_sizes.0 * chunk_sizes.1);
                                        waiting_chunk_data.temp_chunk_vector[x][y][z].is_touching_air = result[index] != 0;
                                    }
                                }
                            }

                            waiting_chunk_data.temp_chunk_vector_finished = true;
                            
                            // put the temp chunk vector into the chunk hashmap
                            
                            fill_chunk_hashmap(
                                &mut chunk.chunk_blocks, 
                                &mut chunk.instances_to_render, 
                                &waiting_chunk_data.temp_chunk_vector, 
                                chunk_sizes
                            );
                            

                            // update the number of alive blocks
                            chunk.alive_blocks = chunk.chunk_blocks.len() as u32;

                            // update instance size
                            chunk.instance_size = chunk.instances_to_render.len() as u32;
                            if chunk.instance_size > chunk.instance_capacity {
                                chunk.update_instance_buffers_capacity(device.clone(), queue.clone());
                            } else {
                                // if i am not increasing the size of the buffer i can just update the staging buffer, 
                                // which will then eventually update the main buffer
                                chunk.update_instance_staging_buffer(queue.clone());
                            }

                        }
                        None => {
                            // it hasnt finished mapping yet so i check next time
                        }
                    }
                } else {
                    // this is the final step where i will insert the chunk into the hashmap and remove it from the currently_creating_chunks
                    // if the buffer are of size and ready to go

                    // update the instance buffer if possible so finish up the chunk, will finished resizing the buffer and update main buffer if needed
                    { // so mutex will be dropped
                        chunk.update(&device.lock().unwrap(), &queue.lock().unwrap());
                    }

                    // if no instances are modified (the buffers were of the correct size when they were created so it did the usual buffer update and the main buffer now has the correct data)
                    // and no new instance buffer are being created (if there were new instance buffer being created they have finished and main buffer has the correct data now)
                    // then the main buffer has the correct data and size so i can move to the main thread
                    if !chunk.instances_modified && !chunk.creating_new_instance_buffers {
                        // add the chunk to the finished chunks list so i can insert it into the main hashmap after the iteration
                        finished_chunks.push(waiting_chunk_id.clone());
                    }
                } // end if temp_chunk_vector_finished
            } // end if mapping has started
        } // end if fence value == 1
    } // end iteration over currently_creating_chunks
}


pub fn finish_up_chunks(
    finished_chunks: &mut Vec<(i32, i32)>,
    chunks: Arc<Mutex<HashMap<(i32, i32), Chunk>>>,
    currently_creating_chunks: &mut HashMap<(i32, i32), ChunkGenerationData>,
    filesystem: Arc<Mutex<FileSystem>>,
    created_chunks: Arc<Mutex<HashSet<(i32, i32)>>>,
    chunk_sizes: (usize, usize, usize),
) {
    for finished_chunk_id in finished_chunks.clone() {
        // remove the chunk from the creating hashmap
        let finished_chunk_data: ChunkGenerationData = currently_creating_chunks
            .remove(&finished_chunk_id)
            .expect("unable to remove chunk from 'currently_creating_chunks' as they key doesnt exist");

        // remove the old one from the actual chunks in main
        // if there is an actual chunk to unload (not always if im first loading in all chunks)
        match finished_chunk_data.unload_chunk_ids {
            Some(unload_chunk_id) => {
                remove_chunk(&mut chunks.lock().unwrap(), unload_chunk_id, &mut filesystem.lock().unwrap(), chunk_sizes);
            }
            None => {
                // if there is no chunk to unload then i dont need to do anything
            }
        }

        // add the new one to the actual chunks in main
        { // mutex lock scope
            // add this chunk to created chunks now that it is finished (needs to be done before i move value into hashmap)
            created_chunks.lock().unwrap().insert((finished_chunk_data.chunk.chunk_id_x, finished_chunk_data.chunk.chunk_id_z));

            // insert it into the hashmap last
            chunks.lock().unwrap().insert((finished_chunk_data.chunk.chunk_id_x, finished_chunk_data.chunk.chunk_id_z), finished_chunk_data.chunk);
            
        }
    }
}