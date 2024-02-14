
// holds the window and event loop 
// this needs to be created before the renderer because the window needs to outlast it (lifetime stuff)


use std::sync::Arc;
use winit::{
    dpi::{LogicalPosition, LogicalSize}, event_loop::EventLoop, window::{Window, WindowBuilder}
};
// will hold the event loop and window
pub struct WindowWrapper {
    pub eventLoop: EventLoop<()>,
    pub window: Arc<Window>,
}


impl WindowWrapper {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let eventLoop = EventLoop::new().unwrap();
        let mut builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width, height))
            .with_position(LogicalPosition{ x: 0, y: 0 }); // spawn the window on the top left out of the way
        
        let window: Arc<Window> = Arc::new(builder.build(&eventLoop).unwrap());

        Self { 
            eventLoop, 
            window,
        }
    }
}