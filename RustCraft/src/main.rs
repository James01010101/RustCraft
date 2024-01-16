

// warnings to ignore
#![allow(non_snake_case)]

// grab any modules i need
use GameEngine::TestFuncs::{Add};

fn main() {

    let x: u32 = 10;
    let y: u32 = 9;
    let z: u32 = Add(x, y);
    println!("{} + {} = {}", x, y, z);
}
