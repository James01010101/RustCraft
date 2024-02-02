use crate::Settings::*;
use crate::Renderer::*;
use crate::World::*;

use nalgebra::{Point3, Vector3};
use glfw::Context;


pub struct GPUData {
    pub cubeVao: u32,
    pub cubeVbo: u32,
    pub cubeEbo: u32,
    pub cubeInstanceVbo: u32,
    pub cubeColoursVbo: u32,

    pub instancesUsed: u32, // how many of the instances am i actually using this frame

    pub cubeVertices: Vec<i32>,
    pub cubeTrisIndices: Vec<u16>, 
    pub cubeInstanceModelMatricies: [[[f32; 4]; 4]; maxBlocksRendered],

    pub cubeColours: [[f32; 4]; maxBlocksRendered], // temporary for now until i use textures


}


impl GPUData {
    pub fn new () -> GPUData {
        // cube vertices (assume starts at (0,0,0))
        let cubeVertices: Vec<i32> = vec![
            0, 0, 0, // Bottom Front Left
            1, 0, 0, // Bottom Front Right
            1, 0, 1, // Bottom Back Right
            0, 0, 1, // Bottom Back Left

            0, 1, 0, // Top Front Left
            1, 1, 0, // Top Front Right
            1, 1, 1, // Top Back Right
            0, 1, 1, // Top Back Left
        ];

        // this is the indexes into the cubeVertices array, so it knows what vertices to use for what triangles
        let cubeTrisIndices: Vec<u16> = vec![
            // Front face
            0, 1, 5, 0, 5, 4,
            // Back face
            3, 2, 6, 3, 6, 7,
            // Bottom face
            0, 1, 2, 0, 2, 3,
            // Top face
            4, 5, 6, 4, 6, 7,
            // Left face
            0, 3, 7, 0, 7, 4,
            // Right face
            1, 2, 6, 1, 6, 5
        ];

        // instance array
        let cubeInstanceModelMatricies: [[[f32; 4]; 4]; maxBlocksRendered] = [[[0.0; 4]; 4]; maxBlocksRendered];

        let cubeColours: [[f32; 4]; maxBlocksRendered] = [[0.0; 4]; maxBlocksRendered];


        // Create a VAO (basically how the memory is layed out)
        let mut cubeVao: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut cubeVao);
            gl::BindVertexArray(cubeVao); 
        }

        // Create a VBO (Stores the verticies data)
        let mut cubeVbo: u32 = 0;
        let vboBufferSize: usize = cubeVertices.len() * std::mem::size_of::<i32>();
        unsafe {
            gl::GenBuffers(1, &mut cubeVbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, cubeVbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                vboBufferSize as gl::types::GLsizeiptr,
                cubeVertices.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW
            );

            // Set the vertex attributes pointers (this is for the vao but do this after vbo is bound, because it connects to the vbo)
            let stride = 3 * std::mem::size_of::<f32>() as gl::types::GLsizei;
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);
        }

        // create an Element Buffer Object (to store the indexes array which points to each vertex in the verticies array)
        let mut cubeEbo: u32 = 0;
        let eboBufferSize: usize = cubeTrisIndices.len() * std::mem::size_of::<u16>();
        unsafe {
            gl::GenBuffers(1, &mut cubeEbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, cubeEbo);
            
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, 
                eboBufferSize as gl::types::GLsizeiptr, 
                cubeTrisIndices.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW
            );
        }

        // VBO for instance model matrices
        let mut cubeInstanceVbo: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut cubeInstanceVbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, cubeInstanceVbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (maxBlocksRendered * std::mem::size_of::<[[f32; 4]; 4]>()) as isize,
                cubeInstanceModelMatricies.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );

            // Set attribute pointers for instance model matrices
            let stride: i32 = std::mem::size_of::<[[f32; 4]; 4]>() as i32;
            let pointerSize: u32 = std::mem::size_of::<[f32; 4]>() as u32;
            for i in 0..4 as u32 {
                gl::EnableVertexAttribArray(1 + i);
                gl::VertexAttribPointer(
                    1 + i,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    (i * pointerSize) as *const gl::types::GLvoid,
                );
                gl::VertexAttribDivisor(1 + i, 1); // Tell OpenGL this is instanced data
            }
        }

        let mut cubeColoursVbo: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut cubeColoursVbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, cubeColoursVbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (maxBlocksRendered * std::mem::size_of::<[f32; 4]>()) as isize,
                cubeColours.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );


            gl::EnableVertexAttribArray(5);
            gl::VertexAttribPointer(
                5,
                4,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null(),
            );
            gl::VertexAttribDivisor(5, 1); // Tell OpenGL this is instanced data
        }


        GPUData {
            cubeVao,
            cubeVbo,
            cubeEbo,
            cubeInstanceVbo,
            cubeColoursVbo,

            instancesUsed: 0,

            cubeVertices,
            cubeTrisIndices, 
            cubeInstanceModelMatricies,

            cubeColours,

        }

    }


    pub fn RenderFrame(&self, renderer: &mut Renderer) {

        // clean screen
        unsafe {
            gl::UseProgram(renderer.openGLProgram);
            
            gl::ClearColor(0.0, 0.0, 0.0, 1.0); // Set clear color (black in this case)
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // Clear the screen

            // bind the specific vao for this object
            gl::BindVertexArray(self.cubeVao);
        }

        // Create a perspective projection matrix
        let projectionMatrix = nalgebra::Perspective3::new(
            renderer.camera.aspectRatio, 
            renderer.camera.fov, 
            renderer.camera.nearPlane, 
            renderer.camera.farPlane
        ).to_homogeneous();

        // Create a view matrix
        let eye = Point3::new(renderer.camera.position.x, renderer.camera.position.y, renderer.camera.position.z);
        let target = Point3::new(renderer.camera.target.x, renderer.camera.target.y, renderer.camera.target.z);
        let viewMatrix = nalgebra::Isometry3::look_at_rh(&eye, &target, &Vector3::y()).to_homogeneous();
        
       
        unsafe {
            // upload these to the gpu
            gl::UniformMatrix4fv(renderer.viewMatrixLocation, 1, gl::FALSE, viewMatrix.as_ptr());
            gl::UniformMatrix4fv(renderer.projectionMatrixLocation, 1, gl::FALSE, projectionMatrix.as_ptr());
            

            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                self.cubeTrisIndices.len() as i32, // Assuming cube_indices is defined
                gl::UNSIGNED_SHORT,
                std::ptr::null(),
                self.instancesUsed as i32,
            );
        }
            

        renderer.window.swap_buffers();
    }


    // TODO: #57 correctly load the chunks of blocks onto the gpu
    pub fn UpdateCubeInstances(&mut self, world: &mut World) {

        self.instancesUsed = world.testBlocks.len() as u32;

        // Instance model matricies, each element is a model matrix of a block
        for i in 0..self.instancesUsed {
            let i: usize = i as usize;

            self.cubeInstanceModelMatricies[i] = world.testBlocks[i].modelMatrix;
            self.cubeColours[i] = world.testBlocks[i].blockType.BlockColour();
        }

        // update the data on the gpu
        unsafe {
            // model matrix
            gl::BindBuffer(gl::ARRAY_BUFFER, self.cubeInstanceVbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (self.instancesUsed as usize * std::mem::size_of::<[[f32; 4]; 4]>()) as isize,
                self.cubeInstanceModelMatricies.as_ptr() as *const gl::types::GLvoid,
            );

            // colour
            gl::BindBuffer(gl::ARRAY_BUFFER, self.cubeColoursVbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (self.instancesUsed as usize * std::mem::size_of::<[f32; 4]>()) as isize,
                self.cubeColours.as_ptr() as *const gl::types::GLvoid,
            );
        }

    }


}