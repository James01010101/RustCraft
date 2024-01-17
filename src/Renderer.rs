// This file will be for all rendering to windows


use glfw::{Action, Context, Key, Glfw, InitError, WindowEvent, PWindow, GlfwReceiver};
use gl::{load_with};
use std::ptr::null;

pub struct Renderer {
    pub glfwObj: Glfw,

    pub window: PWindow,
    pub windowSizeX: u32,
    pub windowSizeY: u32,

    pub events: GlfwReceiver<(f64, WindowEvent)>,

    pub gltexture: gl::types::GLuint,
}

// this is where i write the functions for the Renderer Struct
pub fn CreateRenderer(width: u32, height: u32) -> Renderer {

    // Initialize GLFW
    let mut glfwObj = glfw::init_no_callbacks().unwrap();

    // Set up window hints here (like version, profile, etc.)
    glfwObj.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfwObj.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfwObj.create_window(width, height, "RustCraft", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();

    // Load all OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // make the gl texture
    let mut gltexture: gl::types::GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut gltexture);
        gl::BindTexture(gl::TEXTURE_2D, gltexture);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA, width, height, 0, gl::RGBA, gl::FLOAT, null());
    }

    // create my pixel buffers
    // vec is created like this vec![[num type; amount]; size]
    // so each element will be 4 f32, and there is width * height elements
    let mut pixelBuff1 = vec![[0.0f32; 4]; width * height]; // Initialize with black color
    let mut pixelBuff2 = vec![[128.0f32; 4]; width * height]; // Initialize with grey color
    let mut pixelBuff3 = vec![[255.0f32; 4]; width * height]; // Initialize with white color

    // now create the Renderer struct to return
    let r: Renderer = Renderer {
        glfwObj: glfwObj,
        window: window,
        windowSizeX: width,
        windowSizeY: height,
        events: events,
        gltexture: gltexture
    };
    return r;
}


