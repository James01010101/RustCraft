// This file will be for all rendering to windows

extern crate glfw;
extern crate gl;

use crate::GPUKernels::OpenGLShaders;
use crate::Camera::Camera;

use std::ffi::{CStr, CString};

use glfw::{Context, GlfwReceiver, PWindow, WindowEvent};
use glfw::ffi::glfwSwapInterval;

use gl::types::{GLchar, GLint};


pub struct Renderer {

    pub glfw: glfw::Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,

    pub totalPixels: u32,

    pub openGLProgram: u32,

    pub camera: Camera,

    // gpu memory locations
    pub viewMatrixLocation: i32,
    pub projectionMatrixLocation: i32,

}

// this is where i write the functions for the Renderer Struct
impl Renderer { 
    pub fn new(width: u32, height: u32, fieldOfView: f32) -> Renderer {

        let mut glfw: glfw::Glfw = glfw::init(glfw::fail_on_errors).unwrap();

        // create the window
        let (mut window, events) = glfw.create_window(
            width, 
            height, 
            "RustCraft", 
            glfw::WindowMode::Windowed
        )
            .expect("Failed to create GLFW window.");
        
        window.set_pos(0, 30); // spawn the window on the top left of the screen so its out of the way (+y30 so i can see the top bar)
        window.set_char_polling(true);
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.make_current();
        

        // load all gl functions
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        
        // other important gl functions
        unsafe {
            // uncap the frame rate (0: uncapped, otherwise fps is monitorRefresh / n)
            // TODO: #36 timedelta so movement isnt relative to fps
            glfwSwapInterval(0);
            
            gl::Enable(gl::DEPTH_TEST);
            
            // use blending to use alpha channel for colours
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        }


        // get the open gl version
        let glVersion = unsafe {
            let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
            data.to_string_lossy().into_owned()
        };
        println!("OpenGL version: {}", glVersion);

        // make the open gl graphics pipeline
        let openGLProgram = CreateOpenGLProgram();

        // calculate camera info
        let camera: Camera = Camera::new(fieldOfView, width, height);

        // shader variables locations
        let mut viewLocation: i32 = 0;
        let mut projectionLocation: i32 = 0;
        unsafe {
            viewLocation = gl::GetUniformLocation(openGLProgram, CString::new("view").unwrap().as_ptr());
            projectionLocation = gl::GetUniformLocation(openGLProgram, CString::new("projection").unwrap().as_ptr());
        }

        // create pixel buffers (CPU)
        let totalPixels: u32 = width * height;


        Renderer {
            glfw: glfw,
            window: window,
            events: events,

            totalPixels: totalPixels,

            openGLProgram: openGLProgram,

            camera: camera,

            viewMatrixLocation: viewLocation,
            projectionMatrixLocation: projectionLocation,
        }

    }
}


// creates and compiles the vertex and fragment shaders into a program
fn CreateOpenGLProgram() -> u32 {
    unsafe {
        // Create Vertex Shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &CString::new(OpenGLShaders::vertex_shader_source).unwrap().as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);

        // Check for vertex shader compile errors
        let mut success = gl::FALSE as GLint;
        let mut info_log: Vec<u8> = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // Subtract 1 to skip the trailing null character

        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(vertex_shader, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::COMPILATION_FAILED\n{}", std::str::from_utf8(&info_log).unwrap());
        }
    
        // Create Fragment Shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &CString::new(OpenGLShaders::fragment_shader_source).unwrap().as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_shader);

        // Check for fragment shader compile errors
        success = gl::FALSE as GLint;
        let mut info_log: Vec<u8> = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // Subtract 1 to skip the trailing null character

        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(fragment_shader, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::COMPILATION_FAILED\n{}", std::str::from_utf8(&info_log).unwrap());
        }
    
        // Create Shader Program
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        
        // Check for linking errors
        success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // Subtract 1 to skip the trailing null character

        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(shader_program, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}", std::str::from_utf8(&info_log).unwrap());
        }
    
        // Delete shaders; no longer necessary after linking
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    }


}


