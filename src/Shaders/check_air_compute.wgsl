@group(0)
@binding(0)
var<storage, read> block_type_buffer: array<u32>;

@group(0)
@binding(1)
var<storage, read> block_transparency_buffer: array<u32>;

@group(0)
@binding(2)
var<storage, read> dimentions_buffer: array<u32>;

@group(0)
@binding(3)
var<storage, read_write> result_buffer: array<u32>;


// function to take the xyz values and return the index
fn get_index(x: u32, y: u32, z: u32) -> u32 {
    return (x + (y * dimentions_buffer[0]) + (z * dimentions_buffer[0] * dimentions_buffer[1]));
}


@compute
@workgroup_size(1) // each workgroup sent from the gpu is one thread, so each thread is a different index
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    let id: u32 = global_id.x;
    var newid: u32 = 0;

    // get its xyz coords from its id
    let z: u32 = id / (dimentions_buffer[0] * dimentions_buffer[1]);
    let y: u32 = (id % (dimentions_buffer[0] * dimentions_buffer[1])) / dimentions_buffer[0];
    let x: u32 = id % dimentions_buffer[0];

    // touching air count variable (increases for every block touching air, not necessary but saves the bool if i can just keep adding and if > 1 true)
    // will do all faces now do i can later check only which faces not that any face it touching air
    var touching_air: u32 = 0;

    // check the type of this block, if it is air ignore it and return
    if block_type_buffer[id] == 0 {
        result_buffer[id] = 0u;
        return;
    }

    // go through each of the 6 sides of the block if it is touching air increment its value by one
    // also check that the block ill check is in bounds, eg not x=-1

    // top
    if y + 1 < dimentions_buffer[1] { // check bounds
        newid = get_index(x, y+1, z);
        if block_transparency_buffer[newid] > 0 { // true (so it is transparent)
            touching_air = touching_air + 1;
        }
    }

    // bottom
    if y - 1 >= 0 { // check bounds
        newid = get_index(x, y-1, z);
        if block_transparency_buffer[newid] > 0 { // true (so it is transparent)
            touching_air = touching_air + 1;
        }
    }


    // right
    if x + 1 < dimentions_buffer[0] { // check bounds
        newid = get_index(x+1, y, z);
        if block_transparency_buffer[newid] > 0 { // true (so it is transparent)
            touching_air = touching_air + 1;
        }
    }

    // left
    if x - 1 >= 0 { // check bounds
        newid = get_index(x-1, y, z);
        if block_transparency_buffer[newid] > 0 { // true (so it is transparent)
            touching_air = touching_air + 1;
        }
    }


    // back
    if z + 1 < dimentions_buffer[2] { // check bounds
        newid = get_index(x, y, z+1);
        if block_transparency_buffer[newid] > 0 { // true (so it is transparent)
            touching_air = touching_air + 1;
        }
    }

    // front
    if z - 1 >= 0 { // check bounds
        newid = get_index(x, y, z-1);
        if block_transparency_buffer[newid] > 0 { // true (so it is transparent)
            touching_air = touching_air + 1;
        }
    }


    // finally save the variable to the results buffer
    result_buffer[id] = touching_air;

    return;
}
