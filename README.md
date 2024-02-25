# RustCraft
Minecraft from scratch using Rust and wgpu
testing shaders
https://github.com/austinEng/webgpu-samples/blob/main/src/shaders/basic.vert.wgsl


Important Notes

**Coordinate System** 
X: left and right. Left is positive, going right is negative (because this is how world space is)
Y, up and down, up is positive, down is negative
Z: forward and back, forward is positive, backwards is negative

the point of a block is the front bottom left vertex
same for the start of a chunk


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




Implement queues in some capacity this could help solve alot of problems
i could use it to raycast to check which block i am looking at
for instance add blocks from where the camera dirwctky is 
away feom the camera up to say 10 blocks away
then i just check theough the blocks and check the first one i hit that is not air

can also use queus for lighting 


**GitHub** 
Once a pull request has gone through to delete the branch locally
git branch -d "branchname" // this will delete the local version of the branch (if it has been merged)
git fetch --prune // this will delete the remote connection to this branch
