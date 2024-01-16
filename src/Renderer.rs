// This file will be for all rendering to windows


use glfw::{Action, Context, Key, Glfw, InitError, WindowEvent, PWindow, GlfwReceiver};
use gl::{load_with};

pub struct Renderer {
    pub glfwObj: Glfw,

    pub window: PWindow,
    pub windowSizeX: u32,
    pub windowSizeY: u32,

    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

// this is where i write the functions for the Renderer Struct
pub fn CreateRenderer(width: u32, height: u32) -> Renderer {

    // Initialize GLFW
    let mut glfwObj = glfw::init_no_callbacks().unwrap();

    // Set up window hints here (like version, profile, etc.)
    glfwObj.window_hint(glfw::WindowHint::ContextVersion(1, 0));
    glfwObj.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfwObj.create_window(width, height, "RustCraft", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();

    // Load all OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // now create the Renderer struct to return
    let r: Renderer = Renderer {
        glfwObj: glfwObj,
        window: window,
        windowSizeX: width,
        windowSizeY: height,
        events: events
    };
    return r;
}



