# RustCraft
Minecraft from scratch using Rust


opencl3 Docs: https://docs.rs/opencl3/latest/opencl3/

opencl3 examples: https://github.com/kenba/opencl3/tree/main/examples

// with window stuff
gl docs: https://docs.rs/gl/latest/gl/all.html

glfw docs: https://docs.rs/glfw/latest/glfw/all.html

gl Examples for using shaders:
https://github.com/brendanzab/gl-rs/blob/master/gl/examples/triangle.rs

currently want to try to get to fade from red into 0 at the middle
green fades from 0 at top to 255 in middle to 0 at bottom
blue is 0 at middle and fades to 255 at bottom






Important Notes

**Coordinate System**
X: left and right. Left is negative, going right is positive
Y: forward and back, forward is positive, backwards is negative
Z, up and down, up is positive, down is negative


**Static and Dynamic Objects**
Static:
tris dont change so they dont need to be recalculated

Dynamic:
need to recalc tris everytime it moves