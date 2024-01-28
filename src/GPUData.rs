use crate::Settings::*;

pub struct GPUData {
    pub cubeVao: u32,
    pub cubeVbo: u32,
    pub cubeEbo: u32,
    pub cubeInstanceVbo: u32,
    pub cubeColoursVbo: u32,

    pub instancesUsed: u32, // how many of the instances am i actually using this frame

    pub cubeVerticies: Vec<i32>,
    pub cubeTrisIndices: Vec<u16>, 
    pub cubeInstanceModelMatricies: [[[f32; 4]; 4]; maxBlocksRendered],

    pub cubeColours: [[f32; 4]; maxBlocksRendered], // temporary for now until i use textures


}


impl GPUData {
    pub fn new () -> GPUData {
        // cube verticies (assume starts at (0,0,0))
        let cubeVerticies: Vec<i32> = vec![
            0, 0, 0, // Bottom Back Left
            1, 0, 0, // Bottom Back Right
            1, 0, 1, // Bottom Front Right
            0, 0, 1, // Bottom Front Left
            0, 1, 0, // Top Back Left
            1, 1, 0, // Top Back Right
            1, 1, 1, // Top Front Right
            0, 1, 1, // Top Front Left
        ];

        // this is the indexes into the cubeVerticies array, so it knows what verticies to use for what triangles
        let cubeTrisIndices: Vec<u16> = vec![
            // Bottom face
            0, 1, 2, 0, 2, 3,
            // Top face
            4, 5, 6, 4, 6, 7,
            // Front face
            3, 2, 6, 3, 6, 7,
            // Back face
            4, 5, 1, 4, 1, 0,
            // Left face
            4, 0, 3, 4, 3, 7,
            // Right face
            1, 5, 6, 1, 6, 2
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
        let vboBufferSize: usize = cubeVerticies.len() * std::mem::size_of::<i32>();
        unsafe {
            gl::GenBuffers(1, &mut cubeVbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, cubeVbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                vboBufferSize as gl::types::GLsizeiptr,
                cubeVerticies.as_ptr() as *const gl::types::GLvoid, 
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


        println!("VBO Buffer Size: {} bytes \nEBO Buffer Size: {} bytes",vboBufferSize, eboBufferSize);

        GPUData {
            cubeVao,
            cubeVbo,
            cubeEbo,
            cubeInstanceVbo,
            cubeColoursVbo,

            instancesUsed: 0,

            cubeVerticies,
            cubeTrisIndices, 
            cubeInstanceModelMatricies,

            cubeColours,

        }

    }
}