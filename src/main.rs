

// warnings to ignore
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

// grab any modules i need
pub mod Renderer;
pub mod MainGameLoop;

fn main() {

    MainGameLoop::RunMainGameLoop();
    
}
