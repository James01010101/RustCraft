
use crate::FileSystem::FileSystem;
use crate::Renderer::*;
use crate::Settings::{screenFOV, screenHeight, screenWidth};
use crate::WindowWrapper::*;
use crate::World::*;
use crate::GPUData::GPUData;
use crate::Block::*;
use crate::Chunk::*;
use crate::Camera::*;

use std::time::Instant;
use std::mem;
use async_std::task;

use wgpu::util::DeviceExt;

use winit::event::{Event, WindowEvent};



pub fn RunMainGameLoop() {

    let dontStartScreen: bool = false;


    println!("Size of Block: {} bytes", mem::size_of::<Block>());
    println!("Size of Chunk: {} bytes\n", mem::size_of::<Chunk>());

    // Create the window wrapper
    let mut windowWrapper: WindowWrapper = WindowWrapper::new("RustCraft", screenWidth as u32, screenHeight as u32);

    let mut camera: Camera = Camera::new(
        screenFOV,
        screenWidth as u32,
        screenHeight as u32
    );

    // create Renderer and window
    let mut renderer: Renderer = task::block_on(Renderer::new(&windowWrapper, &camera));


    // create MY file system struct
    let mut fileSystem: FileSystem = FileSystem::new();
    fileSystem.CheckFileSystem();

    // create my world
    let mut world: World = World::new();
    // temp, add some blocks for testing
    world.LoadCreatedChunksFile(&mut fileSystem);
    world.AddTestBlocks();
    world.AddTestChunks(&mut fileSystem);


    // create the gpudata buffers
    let mut gpuData: GPUData = GPUData::new(&renderer.device);
    gpuData.UpdateCubeInstances(&mut world, &renderer.queue);
    



    
    
    
    let mut angle: f32 = 0.0; // Current angle of rotation
    let rotation_speed: f32 = 0.008; // Speed of rotation
    let radius: f32 = 3.0; // Distance from the center


 
    // stats before starting
    let mut frameNumber: u64 = 0;
    let windowStartTime: Instant = Instant::now();

    // event loop
    windowWrapper.eventLoop
        .run(move |event, target| {


            // check if the event is a window event, if it use get the event from inside the window event
            if let Event::WindowEvent {
                window_id: _, // ignore this variable
                event,
            } = event
            {
                match event {
                    WindowEvent::Resized(new_size) => {

                        println!("Surface resize {new_size:?}");

                        renderer.surfaceConfig.width = new_size.width.max(1);
                        renderer.surfaceConfig.height = new_size.height.max(1);
                        renderer.surface.configure(&renderer.device, &renderer.surfaceConfig);
                        
                        // updates the porjection matrix, this doesnt exist yet
                        /*
                        let mx_total = renderer.generate_matrix(renderer.surfaceConfig.width as f32 / renderer.surfaceConfig.height as f32);
                        let mx_ref: &[f32; 16] = mx_total.as_ref();
                        renderer.queue.write_buffer(&renderer.uniform_buf, 0, bytemuck::cast_slice(mx_ref));
                        */

                        // so it always generates a new frame
                        windowWrapper.window.request_redraw();


                    }
                    WindowEvent::RedrawRequested => {

                        // move the camera
                        // rotate the camera for testing
                        angle += rotation_speed;
                        camera.position.x = radius * angle.cos();
                        camera.position.z = radius * angle.sin();

                        // Calculate the new view and projection matrices
                        let vertUniforms: VertexUniforms = VertexUniforms {
                            view: camera.calculate_view_matrix().into(),
                            projection: camera.calculate_projection_matrix().into(),
                        };


                        // Create a temporary buffer with the new data for the uniform buffer
                        let staging_buffer = renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Staging uniform Buffer"),
                            contents: bytemuck::bytes_of(&vertUniforms),
                            usage: wgpu::BufferUsages::COPY_SRC,
                        });




                        // calculate the frams
                        let frame = renderer.surface
                            .get_current_texture()
                            .expect("Failed to acquire next swap chain texture");
                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());
                        let mut encoder =
                            renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });
                        
                        // update the uniform buffer with the new camera position matricies
                        encoder.copy_buffer_to_buffer(&staging_buffer, 0, &renderer.uniform_buffer, 0, staging_buffer.size());
                        

                        // if my instances have changed then i update the instance buffer with its staging buffer
                        if gpuData.instances_modified {
                            encoder.copy_buffer_to_buffer(&gpuData.instance_staging_buf, 0, &gpuData.instance_buf, 0, gpuData.instance_staging_buf.size());
                            encoder.copy_buffer_to_buffer(&gpuData.colour_staging_buf, 0, &gpuData.colour_buf, 0, gpuData.colour_staging_buf.size());
                            gpuData.instances_modified = false;
                        }


                        let depth_texture_view = &renderer.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

                        // set all of the commands i will use in the render pass
                        {
                            let mut rpass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: None,
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: &view,
                                        resolve_target: None,
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],


                                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                                        view: depth_texture_view,
                                        depth_ops: Some(wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(1.0),
                                            store: wgpu::StoreOp::Store,
                                        }),
                                        stencil_ops: None,
                                    }),

                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });

                            // Set the vertex and index buffers here
                            rpass.set_vertex_buffer(0, gpuData.vertex_buf.slice(..));
                            rpass.set_index_buffer(gpuData.index_buf.slice(..), wgpu::IndexFormat::Uint16);
                            rpass.set_bind_group(0, &renderer.bind_group, &[]);
                            rpass.set_vertex_buffer(1, gpuData.instance_buf.slice(..));
                            rpass.set_vertex_buffer(2, gpuData.colour_buf.slice(..));

                            rpass.set_pipeline(&renderer.render_pipeline);
                            rpass.draw_indexed(0..36, 0, 0..5);
                        } // the render pass must go out of scope before submit and present are called
                        // it finalises the render pass when it goes out of scope so it can be submitted to the gpu

                        renderer.queue.submit(Some(encoder.finish()));
                        frame.present();

                        // so it always generates a new frame
                        windowWrapper.window.request_redraw();
                    }
                    WindowEvent::CloseRequested => target.exit(),
                    _ => {}
                };
            }
        })
        .unwrap();

    CleanUp(&mut world, &mut fileSystem);


    let totalWindowDuration_ms = windowStartTime.elapsed().as_millis();
    let AvgFPS: f32 = frameNumber as f32 / (totalWindowDuration_ms as f32 / 1000.0);
    println!("\nTotal Window Time (ms): {:?}", totalWindowDuration_ms);
    println!("Total Frames Rendered: {}", frameNumber);
    println!("Average Frame Rate: {}", AvgFPS);
}



// this will clean up all data before the program ends
pub fn CleanUp(world: &mut World, fileSystem: &mut FileSystem) {

    let hashmapChunkKeys: Vec<(i32, i32)> = world.chunks.keys().cloned().collect();

    // go through each chunk and call unload on it
    //let mut chunk: &Chunk;

    for key in  hashmapChunkKeys {
        // remove the chunk from the hashmap and return it
        if let Some(mut chunk) = world.chunks.remove(&key) {
            fileSystem.SaveChunkToFile(chunk);
        } else {
            // if the key doesnt match a value ill print this but not panic so i can save the rest
            eprintln!("Failed CleanUp: cannot to find value with key {:?}", key);
        }
    }

    fileSystem.SaveCreatedChunksFile(world);

}