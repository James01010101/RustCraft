
use crate::Renderer::*;
use gl::{Clear, ClearColor};
use glfw::{Key, Action, Context};

pub fn RunMainGameLoop() {

    // create Renderer and window
    let mut renderer: Renderer = CreateRenderer(800, 600);

    // Loop until the user closes the window
    while !renderer.window.should_close() {
        // Render here
        unsafe {
            gl::ClearColor(0.0, 1.0, 0.0, 1.0); // For example, clear the screen to green
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Swap front and back buffers
        renderer.window.swap_buffers();

        // Poll for and process events
        renderer.glfwObj.poll_events();
        for (_, newEvent) in glfw::flush_messages(&renderer.events) {
            handle_window_event(&mut renderer.window, newEvent);
        }
    }

    
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}