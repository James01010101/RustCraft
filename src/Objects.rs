

// a basic 3 int position position struct to store the xyz position of a block
pub struct Position {
    // i16 max/min = 32,767
    // i32 max/min = 2,147,483,647

    x: i32,
    y: i32,
    z: i16,
}   


pub struct Block {
    // what kind of object is it
    blocksType: BlockType,

    // will it be moving frame by frame? so i know if i have to recalc it each frame or just when it changes
    dynamic: bool,

    position: Position,

}


// what type of block is it
pub enum BlockType {
    Air, // no block

    Grass,
    Dirt,

    Stone,
    Cobblestone,

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