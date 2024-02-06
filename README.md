# RustCraft
Minecraft from scratch using Rust

// with window stuff
testing shaders
https://github.com/austinEng/webgpu-samples/blob/main/src/shaders/basic.vert.wgsl


Important Notes

**Coordinate System** 
X: left and right. Left is negative, going right is positive
Y, up and down, up is positive, down is negative
Z: forward and back, forward is positive, backwards is negative

the point of a block is the front bottom left vertex
same for the start of a chunk

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


**File Structure** 
Chunks Files:
each file will have name x_z.txt
and element line will be a block type enum as an int.
starting at the bottom corner (0, 0, 0) will be the first element read
that line will be all the increasing x value,
then next line i increase z by one and then all x again
then once thats done ill leave a line gap and then increase y by 1 and keep going