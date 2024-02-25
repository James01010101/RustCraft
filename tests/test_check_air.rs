

extern crate rust_craft;
use rust_craft::{
    block::*, 
    block_type::*,
    chunk::{chunk_functions::*, chunk_gpu_functions::*, create_chunks::*}, 
};

use async_std::task;
use wgpu::{Device, Queue, ShaderModule};



pub async fn get_renderer_variables() -> (Device, Queue, ShaderModule) {
    
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();

    let shader_code =
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("check air compute shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../src/Shaders/check_air_compute.wgsl").into(),
            ),
        });

    (device, queue, shader_code)
}

// these all assume chunk sizes of (32 x 256 x 32)
fn test_check_air(chunk_sizes: (usize, usize, usize)) {

    // create the parts of the renderer that i need
    let (device, queue, shader_code) = task::block_on(get_renderer_variables());

    // get the chunk data
    let mut temp_chunk_vector: Vec<Vec<Vec<Block>>> = create_temp_chunk_vector((0, 0), chunk_sizes);
    generate_chunk(&mut temp_chunk_vector, chunk_sizes);
    

    // run the check air function
    task::block_on(check_for_touching_air(
        &mut temp_chunk_vector, 
        &device, 
        &queue, 
        &shader_code, 
        chunk_sizes
    ));


    // tests the results
    for y in 0..chunk_sizes.1 {
        for x in 0..chunk_sizes.0 {
            for z in 0..chunk_sizes.2 {
                if temp_chunk_vector[x][y][z].block_type != BlockType::Air {
                    // check the blocks around this one that are touching it (do some bounds checking to not go out of bounds)
                    // and if any blocks are air then it should be touching air
                    // up
                    let mut touching_air_count: u32 = 0;
                    if y < chunk_sizes.1 - 1 {
                        if temp_chunk_vector[x][y + 1][z].block_type == BlockType::Air {
                            assert_eq!(temp_chunk_vector[x][y][z].is_touching_air, true);
                            touching_air_count += 1;
                        }
                    }

                    // down
                    if y > 0 {
                        if temp_chunk_vector[x][y - 1][z].block_type == BlockType::Air {
                            assert_eq!(temp_chunk_vector[x][y][z].is_touching_air, true);
                            touching_air_count += 1;
                        }
                    }

                    // left
                    if x > 0 {
                        if temp_chunk_vector[x - 1][y][z].block_type == BlockType::Air {
                            assert_eq!(temp_chunk_vector[x][y][z].is_touching_air, true);
                            touching_air_count += 1;
                        }
                    }

                    // right
                    if x < chunk_sizes.0 - 1 {
                        if temp_chunk_vector[x + 1][y][z].block_type == BlockType::Air {
                            assert_eq!(temp_chunk_vector[x][y][z].is_touching_air, true);
                            touching_air_count += 1;
                        }
                    }

                    // front
                    if z > 0 {
                        if temp_chunk_vector[x][y][z - 1].block_type == BlockType::Air {
                            assert_eq!(temp_chunk_vector[x][y][z].is_touching_air, true);
                            touching_air_count += 1;
                        }
                    }

                    // back
                    if z < chunk_sizes.2 - 1 {
                        if temp_chunk_vector[x][y][z + 1].block_type == BlockType::Air {
                            assert_eq!(temp_chunk_vector[x][y][z].is_touching_air, true);
                            touching_air_count += 1;
                        }
                    }

                    // finally check if any touched air then it should be true otherwise false
                    if touching_air_count == 0 {
                        assert_eq!(temp_chunk_vector[x][y][z].is_touching_air, false,
                        "After checking all sides this block didnt touch any air blocks but is_touching_air is true");
                    } else {
                        assert_eq!(temp_chunk_vector[x][y][z].is_touching_air, true,
                        "After checking all sides this block touched air blocks but is_touching_air is false");
                    }
                    
                } else {
                    // if it is an air block it should be not touching air
                    assert_eq!(temp_chunk_vector[x][y][z].is_touching_air, false,
                    "This block is an air block but is_touching_air, is true");
                }
            }
        }
    }
}


// testing a smaller size
#[test]
fn test_check_air_1() {
    test_check_air((8, 16, 8));
}


// testing the size i would be using for the game
#[test]
fn test_check_air_2() {
    test_check_air((32, 256, 32));
}

// testing larger sizes for fun
#[test]
fn test_check_air_3() {
    test_check_air((64, 256, 64));
}

// these work but are large and take a while to run so i will ignore them
#[test]
#[ignore]
fn test_check_air_4() {
    test_check_air((64, 512, 64));
}

#[test]
#[ignore]
fn test_check_air_5() {
    test_check_air((128, 512, 128));
}

#[test]
#[ignore]
fn test_check_air_6() {
    test_check_air((256, 512, 256));
}

