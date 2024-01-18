
// test kernel, gradient colours
pub pixelGradientKernelName: &str = "PixelGradient";
pub pixelGradientKernel: &str = r#"
kernel void PixelGradient (global uint* pixels, uint height, uint width)
{
    const size_t id = get_global_id(0);
    const size_t idy = id / width; // pixel row
    const size_t idx = id % width; // pixel col

    uint a = 0;
    uint r = (float)idy / height * 255;
    uint g = 0;
    uint b = (1 - ((float)idy / height)) * 255;
    uint pixelColour = (a << 24) | (r << 16) | (g << 8) | b;

    pixels[id] = pixelColour;
}"#;