# RustCraft
Minecraft from scratch using Rust

// with window stuff
gl docs: https://docs.rs/gl/latest/gl/all.html

glfw docs: https://docs.rs/glfw/latest/glfw/all.html

gl Examples for using shaders:
https://github.com/brendanzab/gl-rs/blob/master/gl/examples/triangle.rs





Important Notes

**Coordinate System**
X: left and right. Left is negative, going right is positive
Y, up and down, up is positive, down is negative
Z: forward and back, forward is positive, backwards is negative

the point of a block is the back bottom left vertex

**GL Functions**
BufferData copys the data to the gpu, it allocates a new array for the data and will automatically free the unused one
give it either DYNAMIC_DRAW or STATIC_DRAW, dynamic is for objects ill change the values on constantly
static for objects that dont have values changed often, but they can still change

BufferSubData rewrites data in a buffer, doesnt realloc.


**Static and Dynamic Objects**
Static:
tris dont change so they dont need to be recalculated

Dynamic:
need to recalc tris everytime it moves