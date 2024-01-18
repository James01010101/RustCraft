
use crate::GPUKernels::{PixelGradient, PixelShift};
use crate::Renderer::{Renderer};

use opencl3::kernel::{Kernel};
use opencl3::program::{Program};

// this will store the compiled kernels
pub struct Kernels {
    pub pixelGradientKernel: Kernel,
    pub pixelShiftKernel: Kernel,
}


// this will go through and compile all kernels and save them to the Kernels Struct and return the Kernels struct
pub fn CreateKernels(renderer: &Renderer) -> Kernels {

    let pixelGradient = CompileKernel(&renderer, PixelGradient::pixelGradientKernel, PixelGradient::pixelGradientKernelName);
    let pixelShift = CompileKernel(&renderer, PixelShift::pixelShiftKernel, PixelShift::pixelShiftKernelName);
    
    let kernels = Kernels {
        pixelGradientKernel: pixelGradient,
        pixelShiftKernel: pixelShift,
    };

    return kernels;
}

// compile the kernels
pub fn CompileKernel(renderer: &Renderer, program: &str, name: &str) -> Kernel {
    println!("Compiling Kernel: {}", name);

    let builtProgram = Program::create_and_build_from_source(&renderer.context, program, "")
        .expect("Program::create_and_build_from_source failed");

    let kernel = Kernel::create(&builtProgram, name).expect("Kernel::create failed");

    return kernel;
}