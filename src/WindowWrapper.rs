
// holds the window and event loop 
// this needs to be created before the renderer because the window needs to outlast it (lifetime stuff)


use std::sync::Arc;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
    dpi::LogicalSize,
};
// will hold the event loop and window
pub struct WindowWrapper {
    eventLoop: EventLoop<()>,
    window: Arc<Window>,
}


impl WindowWrapper {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let eventLoop = EventLoop::new().unwrap();
        let mut builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width, height));
        
        let window: Arc<Window> = Arc::new(builder.build(&eventLoop).unwrap());

        Self { 
            eventLoop, 
            window,
        }
    }
}