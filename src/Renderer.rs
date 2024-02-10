// This file will be for all rendering to windows

use crate::Camera::*;
use crate::WindowWrapper::*;

use wgpu::{
    Device,
    Queue,
    Adapter,
    Instance,
    Surface,
    ShaderModule,
    PipelineLayout,
    RenderPipeline,
    SurfaceConfiguration,

    util::DeviceExt,

};

use bytemuck::{Pod, Zeroable};

pub struct Renderer {
    pub instance: Instance,
    pub surface: Surface<'static>,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub vertShaderCode: ShaderModule,
    pub fragShaderCode: ShaderModule,
    pub pipeline_layout: PipelineLayout,
    pub render_pipeline: RenderPipeline,
    pub surfaceConfig: SurfaceConfiguration,
    pub bind_group: wgpu::BindGroup,
    pub vertUniforms: VertexUniforms, 
    pub uniform_buffer: wgpu::Buffer,
    pub depth_texture: wgpu::Texture,
}

// this is where i write the functions for the Renderer Struct
impl Renderer { 
    pub async fn new(windowWrapper: &WindowWrapper, camera: &Camera) -> Renderer {
        
        // set the window height and width variables
        let windowHeight = windowWrapper.window.inner_size().height;
        let windowWidth = windowWrapper.window.inner_size().width;

        // create instance and surface
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(windowWrapper.window.clone()).unwrap();

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
        let vertShaderCode = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("Shaders/myVert.wgsl").into()),
        });

        let fragShaderCode = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("Shaders/myFrag.wgsl").into()),
        });

        let mut surfaceConfig = surface.get_default_config(
            &adapter, 
            windowWidth, 
            windowHeight)
            .unwrap();
        surface.configure(&device, &surfaceConfig);

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        
        // describe the layout of the vertex buffer in memory, 3 floats of pos
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: 3 * std::mem::size_of::<f32>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3],
        };


        // describe the layout of the instance buffer in memory, 4x4 matrix which is actually 4x vec4
        let instance_buffer_layout = wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![2 => Float32x4, 3 => Float32x4, 4 => Float32x4, 5 => Float32x4],
            };

        let colour_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                    },
                ],
            };

        
        // this holsd the uniform data for the vertex shader, the view and projection matrixies combined
        let vertUniforms: VertexUniforms = VertexUniforms {
            projection_view_matrix: camera.projection_view_matrix,
        };

        // Create a buffer on the GPU and copy your uniform data into it
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(&vertUniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create a bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("uniform bind group layout"),
        });

        // Create a bind group that binds your uniform buffer to binding point 0
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
            label: Some("uniform bind group"),
        });

        // used for depth testing so objects in front are drawn over objects behind
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: windowWidth,
                height: windowHeight,
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
                module: &vertShaderCode,
                entry_point: "main", // the entry point for the vertex shader
                buffers: &[vertex_buffer_layout, instance_buffer_layout, colour_buffer_layout], // Add the vertex_buffer_layout here
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragShaderCode,
                entry_point: "main", // the entry point for the fragment shader
                targets: &[Some(wgpu::ColorTargetState {
                    format: surfaceConfig.format,
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
        

        Self {       
            instance,
            surface,
            adapter,
            device,
            queue,
            vertShaderCode,
            fragShaderCode,
            pipeline_layout,
            render_pipeline,
            surfaceConfig,
            bind_group,
            vertUniforms,
            uniform_buffer,
            depth_texture,
        }
    }
}


// Define your uniform data to store the view and projection matrixies
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct VertexUniforms {
    pub projection_view_matrix: [[f32; 4]; 4],
}