
use crate::Renderer::*;
use gl::{Clear, ClearColor};
use glfw::{Key, Action, Context};
use libc::c_void;
use std::mem;

pub fn RunMainGameLoop() -> Result<()> {

    // create Renderer and window
    let mut renderer: Renderer = CreateRenderer(800, 600);
    let c: u32 = 0;

    // Loop until the user closes the window
    while !renderer.window.should_close() {
        

        // Poll for and process events
        renderer.glfwObj.poll_events();
        for (_, newEvent) in glfw::flush_messages(&renderer.events) {
            handle_window_event(&mut renderer.window, newEvent);
        }

        // do stuff with the events


        /* Old render code
        // Render here
        unsafe {
            gl::ClearColor(0.0, 1.0, 0.0, 1.0); // For example, clear the screen to green
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // Swap front and back buffers
        renderer.window.swap_buffers();
        */

        // render the image
        RenderTexture(&mut renderer);
    }

    Ok(())
}

fn RenderTexture(renderer: &mut Renderer) {
    // TODO: #18 update the texture
    // swap the buffer 1 and 2
    mem::swap(&mut renderer.pixelBuff1, &mut renderer.pixelBuff2);

    // render buffer 1
    unsafe {
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as i32, renderer.screenWidth as i32, renderer.screenHeight as i32, 0, gl::RGBA, gl::FLOAT, renderer.pixelBuff1.as_ptr() as *const c_void);
    }
    renderer.window.swap_buffers();
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}