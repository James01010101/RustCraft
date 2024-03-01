/*
This is where any chunk functions related to gpu buffers and compute shaders will live
*/

use crate::chunk::*;

use std::mem;
use wgpu::{Device, Queue, ShaderModule};
use wgpu::util::DeviceExt;

impl Chunk {
    // each frame call this on each chunk. it will update the instance buffer if the instances have been modified
    pub fn update_instance_buffer(&mut self, device: Arc<Mutex<Device>>, queue: Arc<Mutex<Queue>>) {
        // if the instances have been modified then update the instance buffer
        if self.instances_modified {
            // and if the staging buffer is finished writing
            let staging_buffer_writing_locked = self.staging_buffer_writing.lock().unwrap();
            let writing: bool = *staging_buffer_writing_locked;
            drop(staging_buffer_writing_locked);

            if !writing {
                // copy the staging buffer to the instance buffer
                let device_locked = device.lock().unwrap();
                let mut encoder = device_locked.create_command_encoder(
                    &(wgpu::CommandEncoderDescriptor {
                        label: Some("instance buffer copy encoder"),
                    }),
                );
                drop(device_locked);

                encoder.copy_buffer_to_buffer(
                    &self.instance_staging_buffer,
                    0,
                    &self.instance_buffer,
                    0,
                    ((self.instance_size as usize) * mem::size_of::<InstanceData>())
                        as wgpu::BufferAddress,
                );

                let queue_locked = queue.lock().unwrap();
                queue_locked.submit(Some(encoder.finish()));
                drop(queue_locked);

                // set the flag to false
                self.instances_modified = false;
            }
        }
    }

    // update the instance buffer for the chunk with the instances
    pub fn update_instance_staging_buffer(&mut self, queue: Arc<Mutex<Queue>>) {
        let new_instance_buffers_writing_locked = self.new_instance_buffers_writing.lock().unwrap();
        let writing: bool = *new_instance_buffers_writing_locked;
        drop(new_instance_buffers_writing_locked);

        // only start staging buffer commands if no new instance buffers are being created and written to
        if !writing {
            // make the hashmap of instances into a sclice i can pass to a buffer
            let instances_vector = self
                .instances_to_render
                .values()
                .cloned()
                .collect::<Vec<InstanceData>>();
            let instances_slice = instances_vector.as_slice();

            // update the staging gpu buffers and set the flag that this data has changed
            let queue_locked = queue.lock().unwrap();
            queue_locked.write_buffer(
                &self.instance_staging_buffer,
                0,
                bytemuck::cast_slice(instances_slice),
            );

            // submit the write command
            queue_locked.submit(std::iter::empty());

            self.instances_modified = true;

            // get a copy of the staging_buffer_writing var so i can give it to the gpu callback
            let staging_buffer_writing_clone = self.staging_buffer_writing.clone();

            // lock the mutex so i can set it
            let mut staging_buffer_writing = self.staging_buffer_writing.lock().unwrap();
            *staging_buffer_writing = true;

            // Set up a callback to run when the instance buffer copy is finished
            queue_locked.on_submitted_work_done(move || {
                // set the staging buffer writing flag to false so i know its finished writing
                let mut staging_buffer_writing_gpu = staging_buffer_writing_clone.lock().unwrap();
                *staging_buffer_writing_gpu = false;
            });
            drop(queue_locked);
            // dont need to submit the on submitted work done function
        }
    }

    // increase instance buffers capasity
    // this will be called when i try to add blocks to the instance buffer but its size exceeds its capasity
    pub fn update_instance_buffers_capacity(&mut self, device: Arc<Mutex<Device>>, queue: Arc<Mutex<Queue>>) {
        
        self.creating_new_instance_buffers = true;

        let mut capacity_increase_amount: u32 = 100;

        // check that increasing by this much will be enough
        let amount_needed: u32 = self.instance_size - self.instance_capacity;
        while amount_needed > capacity_increase_amount {
            // if 100 isnt enough then keep adding until it is over, not exact so there is still some room
            capacity_increase_amount += 100;
        }

        // i need to reallocate the capacity of the instance buffers
        self.new_instance_capacity = self.instance_capacity + capacity_increase_amount;

        let device_locked = device.lock().unwrap();

        // dont overwrite the staging yet since it might have commands being done
        self.new_instance_staging_buffer = device_locked.create_buffer(&BufferDescriptor {
            label: Some("Instance Staging Buffer"),
            size: (std::mem::size_of::<InstanceData>() * self.new_instance_capacity as usize)
                as wgpu::BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        self.new_instance_buffer = device_locked.create_buffer(&BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (std::mem::size_of::<InstanceData>() * self.new_instance_capacity as usize)
                as wgpu::BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        drop(device_locked);

        // dont need to copy the buffer just copy the instance hashmap into the buffer and overwrite the old buffer so it is freed
        let instances_vector = self
            .instances_to_render
            .values()
            .cloned()
            .collect::<Vec<InstanceData>>();
        let instances_slice = instances_vector.as_slice();

        // update the new actual gpu buffer and set the flag that this data has changed
        let queue_locked = queue.lock().unwrap();
        queue_locked.write_buffer(
            &self.new_instance_buffer,
            0,
            bytemuck::cast_slice(instances_slice),
        );
        // submit the write command
        queue_locked.submit(std::iter::empty());

        // update the fence for the new buffers so i know not to update the old ones anymore
        let new_instance_buffers_writing_clone = self.new_instance_buffers_writing.clone();
        let mut new_instance_buffers_writing = self.new_instance_buffers_writing.lock().unwrap();
        *new_instance_buffers_writing = true;

        queue_locked.on_submitted_work_done(move || {
            // set the staging buffer writing flag to false so i know its finished writing
            let mut new_instance_buffers_writing = new_instance_buffers_writing_clone.lock().unwrap();
            *new_instance_buffers_writing = false;
        });
        drop(queue_locked);
    }

    // once the new buffer has finished being written to with new data i can overwrite the old buffers
    // since no more commands will be pushed to the old buffers since i started making these new ones all old buffer commands SHOULD be finished at this point
    pub fn overwrite_old_instance_buffers(&mut self) {
        // overwrite the old buffers with the new ones
        self.instance_buffer.destroy();

        mem::swap(&mut self.instance_buffer, &mut self.new_instance_buffer);
        mem::swap(&mut self.instance_staging_buffer, &mut self.new_instance_staging_buffer);

        self.creating_new_instance_buffers = false;

        // update so that capacity always holds the exact size of the buffer, and is only updated to the new size once the new buffers are in place
        self.instance_capacity = self.new_instance_capacity;
    }
}


// compute shader to label which blocks are touching air
    // iterate through the whole vector and update all blocks that are touching air using a compute shader
    // this is run once on chunk creation
    pub fn check_for_touching_air(
        temp_chunk_vec: &mut Vec<Vec<Vec<Block>>>,
        device: Arc<Mutex<Device>>,
        queue: Arc<Mutex<Queue>>,
        shader_code: Arc<Mutex<ShaderModule>>,
        chunk_sizes: (usize, usize, usize),
    ) -> (Arc<Mutex<i32>>, wgpu::Buffer) {
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
        let mut chunk_block_types: Vec<u32> = Vec::with_capacity(chunk_sizes.0 * chunk_sizes.1 * chunk_sizes.2);
        for z in 0..chunk_sizes.2 as i32 {
            for y in 0..chunk_sizes.1 as i16 {
                for x in 0..chunk_sizes.0 as i32 {
                    chunk_block_types.push(
                        temp_chunk_vec[x as usize][y as usize][z as usize]
                            .block_type
                            .to_int() as u32,
                    );
                }
            }
        }

        // create the block transparancy array
        let mut chunk_block_transparency: Vec<u32> = Vec::with_capacity(chunk_sizes.0 * chunk_sizes.1 * chunk_sizes.2);
        for z in 0..chunk_sizes.2 as i32 {
            for y in 0..chunk_sizes.1 as i16 {
                for x in 0..chunk_sizes.0 as i32 {
                    chunk_block_transparency.push(
                        temp_chunk_vec[x as usize][y as usize][z as usize]
                            .block_type
                            .is_transparent() as u32,
                    );
                }
            }
        }

        // create the dimentions buffer so the gpu knows the max of xyz
        let dimentions: [u32; 3] = [
            chunk_sizes.0 as u32,
            chunk_sizes.1 as u32,
            chunk_sizes.2 as u32,
        ];

        // now the resulting buffer (cant use bool with the gpu, since rust bools arnt guarenteed to be 1 byte)
        // so instead ill use u8 for all calculations on the gpu and then just convert it to a bool on the cpu once i recieve the results

        // now create the gpu buffers for both of these
        let mut device_locked = device.lock().unwrap();
        let block_type_buffer = device_locked.create_buffer_init(
            &(wgpu::util::BufferInitDescriptor {
                label: Some("block type compute buffer"),
                contents: bytemuck::cast_slice(&chunk_block_types),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }),
        );

        let block_transparency_buffer = device_locked.create_buffer_init(
            &(wgpu::util::BufferInitDescriptor {
                label: Some("block transparency compute buffer"),
                contents: bytemuck::cast_slice(&chunk_block_transparency),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }),
        );

        let dimentions_buffer = device_locked.create_buffer_init(
            &(wgpu::util::BufferInitDescriptor {
                label: Some("block transparency compute buffer"),
                contents: bytemuck::cast_slice(&dimentions),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }),
        );

        let result_buffer_size: wgpu::BufferAddress = 
            (chunk_sizes.0 * chunk_sizes.1 * chunk_sizes.2 * mem::size_of::<u32>()) as wgpu::BufferAddress;
        let result_buffer = device_locked.create_buffer(
            &(wgpu::BufferDescriptor {
                label: Some("Result compute buffer"),
                size: result_buffer_size,
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            }),
        );

        // load the shader

        // create the bind group and pipeline
        // Instantiates the bind group, once again specifying the binding of buffers.

        let bind_group_layout = device_locked.create_bind_group_layout(
            &(wgpu::BindGroupLayoutDescriptor {
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
            }),
        );

        let bind_group = device_locked.create_bind_group(
            &(wgpu::BindGroupDescriptor {
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
            }),
        );

        let pipeline_layout = device_locked.create_pipeline_layout(
            &(wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            }),
        );

        let check_air_shader_locked = shader_code.lock().unwrap();
        let compute_pipeline = device_locked.create_compute_pipeline(
            &(wgpu::ComputePipelineDescriptor {
                label: Some("check for air Compute Pipeline"),
                layout: Some(&pipeline_layout),
                module: &check_air_shader_locked,
                entry_point: "main",
            }),
        );
        drop(check_air_shader_locked);

        // run the shader
        let mut encoder = device_locked.create_command_encoder(
            &(wgpu::CommandEncoderDescriptor {
                label: Some("check air compute encoder"),
            }),
        );
        {
            let mut compute_pass = encoder.begin_compute_pass(
                &(wgpu::ComputePassDescriptor {
                    label: Some("check air compute pass"),
                    timestamp_writes: None,
                }),
            );
            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(
                chunk_sizes.0 as u32,
                chunk_sizes.1 as u32,
                chunk_sizes.2 as u32,
            ); // Number of cells to run, the (x,y,z) size of item being processed
        }

        // submit the compute shader render pass command
        let queue_locked = queue.lock().unwrap();
        queue_locked.submit(Some(encoder.finish()));

        // get the results from the shader
        // make the buffer ill read from
        let read_result_buffer: wgpu::Buffer = device_locked.create_buffer(
            &(wgpu::BufferDescriptor {
                label: Some("Read Buffer"),
                size: result_buffer_size,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
        );

        let mut encoder = device_locked.create_command_encoder(
            &(wgpu::CommandEncoderDescriptor {
                label: Some("read buffer copy encoder"),
            }),
        );

        // copy the data from the gpu data to my reading buffer
        encoder.copy_buffer_to_buffer(
            &result_buffer,
            0,
            &read_result_buffer,
            0,
            result_buffer_size,
        );

        // submit the copy buffer command and the fence submitted work done command
        queue_locked.submit(Some(encoder.finish()));
        

        // make a fence so i know once the copy buffer function has finished
        // Create a shared variable
        let fence = Arc::new(Mutex::new(0));

        // Create a clone of the Arc for the callback
        let fence_clone = Arc::clone(&fence);

        // Set up a callback to run when the submitted work is done
        queue_locked.on_submitted_work_done(move || {
            // This code will run when all work submitted to the queue up to this point has completed
            let mut fence_guard = fence_clone.lock().unwrap();
            *fence_guard = 1;
        });
    
        drop(device_locked); // dont need this anymore so i wont hold it
        drop(queue_locked); // dont need this anymore so i wont hold it

        // stop here since itll be waiting for gpu stuff
        return (fence, read_result_buffer);
    }