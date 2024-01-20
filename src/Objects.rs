


// a basic 3 int position position struct to store the xyz position of a block
// its position is the bottom left back part of the square
pub struct Position {
    // i16 max/min = 32,767
    // i32 max/min = 2,147,483,647

    x: i32,
    y: i32,
    z: i16,
}   

// what type of block is it
pub enum BlockType {
    Air, // no block

    Grass,
    Dirt,

    Stone,
    Cobblestone,

}


// this will store a single face with 3 verts
pub struct Tris {
    v1: Position,
    v2: Position,
    v3: Position,
}


// the main strut to hold all info related to a block
pub struct Block {
    // what kind of object is it
    blocksType: BlockType,

    // will it be moving frame by frame? so i know if i have to recalc it each frame or just when it changes
    dynamic: bool,

    position: Position,

    // Should add a vector of tris here that are pre computed, so its one less thing i need to calculate
    // every time i need to check for collisions
    // TODO: #27 add vector of tris as faces
    faces: [Tris, 12],

}


// from the position of a block create all tris
pub fn CreateTrisFromBlock(pos: &Position) -> Vec<Tris> {

}


pub fn CreateNewBlock(blockType: BlockType, dynamic: bool, posX: i32, posY: i32, posZ: i16) -> Block {
    
    let newPos: Position = Position { x: posX, y: posY, z: posZ };
    
    let newBlock: Block = Block {
        blocksType: blockType,
        dynamic: dynamic,
        position: newPos,
    };

    return newBlock;
}