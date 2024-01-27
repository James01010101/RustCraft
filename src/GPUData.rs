

pub struct GPUData {
    pub cubeVao: u32,
    pub cubeVbo: u32,
    pub cubeEbo: u32,
    pub cubeVerticies: Vec<i32>,
    pub cubeTrisIndices: Vec<u16>, 
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

        println!("VBO Buffer Size: {} bytes \nEBO Buffer Size: {} bytes",vboBufferSize, eboBufferSize);

        GPUData {
            cubeVao,
            cubeVbo,
            cubeEbo,
            cubeVerticies,
            cubeTrisIndices, 
        }

    }
}