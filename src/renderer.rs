// This file will be for all rendering to windows

use crate::{
    camera::*, 
    gpu_data::*, 
    types::*, 
    window_wrapper::*, 
    chunk::*,
};

use wgpu::{
    util::DeviceExt, Adapter, Device, Instance, PipelineLayout, Queue, RenderPipeline,
    ShaderModule, Surface, SurfaceConfiguration,
};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Renderer {
    pub instance: Instance,
    pub surface: Surface<'static>,
    pub adapter: Adapter,
    pub device: Arc<Mutex<Device>>,
    pub queue: Arc<Mutex<Queue>>,

    // shaders
    pub vertex_shader_code: ShaderModule,
    pub fragment_shader_code: ShaderModule,
    pub check_air_compute_shader_code: Arc<Mutex<ShaderModule>>,

    // pipeline
    pub pipeline_layout: PipelineLayout,
    pub render_pipeline: RenderPipeline,
    pub surface_config: SurfaceConfiguration,
    pub bind_group: wgpu::BindGroup,
    pub vertex_uniforms: VertexUniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub depth_texture: wgpu::Texture,
}

// this is where i write the functions for the Renderer Struct
impl Renderer {
    pub async fn new(window_wrapper: &WindowWrapper, camera: &Camera) -> Renderer {
        // set the window height and width variables
        let window_height = window_wrapper.window.inner_size().height;
        let window_width = window_wrapper.window.inner_size().width;

        // create instance and surface
        let instance = wgpu::Instance::default();
        let surface = instance
            .create_surface(window_wrapper.window.clone())
            .unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        // compile my shaders
        let vertex_shader_code = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("Shaders/vertex.wgsl").into()),
        });

        let fragment_shader_code = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("Shaders/fragment.wgsl").into()),
        });

        let check_air_compute_shader_code =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("check air compute shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("Shaders/check_air_compute.wgsl").into(),
                ),
            });

        let surface_config = surface
            .get_default_config(&adapter, window_width, window_height)
            .unwrap();
        surface.configure(&device, &surface_config);

        //let swapchain_capabilities = surface.get_capabilities(&adapter);
        //let swapchain_format = swapchain_capabilities.formats[0];

        // describe the layout of the vertex buffer in memory, 3 floats of pos
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: 3 * std::mem::size_of::<f32>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3],
        };

        // describe the layout of the instance buffer in memory, 4x4 matrix which is actually 4x vec4
        let instance_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![
                2 => Float32x4, 3 => Float32x4, 4 => Float32x4, 5 => Float32x4, // matrix
                6 => Float32x3, // color
            ],
        };

        // this holsd the uniform data for the vertex shader, the view and projection matrixies combined
        let vertex_uniforms: VertexUniforms = VertexUniforms {
            projection_view_matrix: camera.projection_view_matrix,
        };

        // Create a buffer on the GPU and copy your uniform data into it
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(&vertex_uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create a bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("uniform bind group layout"),
        });

        // Create a bind group that binds your uniform buffer to binding point 0
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform bind group"),
        });

        // used for depth testing so objects in front are drawn over objects behind
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: window_width,
                height: window_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[wgpu::TextureFormat::Depth32Float],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader_code,
                entry_point: "main", // the entry point for the vertex shader
                buffers: &[vertex_buffer_layout, instance_buffer_layout], // Add the vertex_buffer_layout here
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader_code,
                entry_point: "main", // the entry point for the fragment shader
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
                unclipped_depth: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        let device = Arc::new(Mutex::new(device));
        let queue = Arc::new(Mutex::new(queue));
        let check_air_compute_shader_code = Arc::new(Mutex::new(check_air_compute_shader_code));

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,

            // shaders
            vertex_shader_code,
            fragment_shader_code,
            check_air_compute_shader_code,

            // pipeline
            pipeline_layout,
            render_pipeline,
            surface_config,
            bind_group,
            vertex_uniforms,
            uniform_buffer,
            depth_texture,
        }
    }

    pub fn render_frame(&self, gpu_data: &GPUData, chunks: &HashMap<(i32, i32), Chunk>) {
        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // create a command encoder for the render pass
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // Update buffers
        // update the uniform buffer with the new camera position matricies ( since it is a queue this must complete before rendering)
        encoder.copy_buffer_to_buffer(
            &gpu_data.vertex_uniform_staging_buf,
            0,
            &self.uniform_buffer,
            0,
            gpu_data.vertex_uniform_staging_buf.size(),
        );

        let depth_texture_view = &self
            .depth_texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // set all of the commands i will use in the render pass
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
            rpass.set_vertex_buffer(0, gpu_data.vertex_buf.slice(..));
            rpass.set_index_buffer(gpu_data.index_buf.slice(..), wgpu::IndexFormat::Uint16);
            rpass.set_bind_group(0, &self.bind_group, &[]);

            rpass.set_pipeline(&self.render_pipeline);

            // draw all full cubes (indicies 0.36 is all the indicies for a cube
            // then draw the number of instances specified
            //rpass.draw_indexed(0..36, 0, 0..gpuData.instancesUsed as u32);

            // Iterate over each chunk
            for chunk in chunks.values() {
                // Set the instance buffer for this chunk
                rpass.set_vertex_buffer(1, chunk.instance_buffer.slice(..));

                // Draw the instances for this chunk
                rpass.draw_indexed(0..36, 0, 0..chunk.instance_size);
            }
        } // the render pass must go out of scope before submit and present are called
          // it finalises the render pass when it goes out of scope so it can be submitted to the gpu

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
