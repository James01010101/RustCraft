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


@compute
@workgroup_size(1) // each workgroup sent from the gpu is one thread, so each thread is a different index
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    let id = global_id.x;

    // get its xyz coords from its id

    // touching air count variable (increases for every block touching air, not necessary but saves the bool if i can just keep adding and if > 1 true)

    // check the type of this block, if it is air ignore it and return

    // go through each of the 6 sides of the block if it is touching air increment its value by one

    // finally save the variable to the results buffer

    return;
}
