use crate::Settings::*;

use std::mem;

// a basic 3 int position position struct to store the xyz position of a block
// its position is the bottom left back part of the square
#[derive(Clone)]
pub struct Position {
    // i16 max/min = 32,767
    // i32 max/min = 2,147,483,647

    pub x: i32,
    pub y: i32,
    pub z: i16,
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
    // every time i need to check for collisions, there will be exactly 12
    faces: [Tris; 12],
}


// from the position of a block create all tris
pub fn CreateTrisFromBlock(pos: &Position) -> [Tris; 12] {
    // create all verts (share them between edges to save space)
    // Name (xyz) or (Left/Right, Front/Back, Top/Bottom)
    let LBB: Position = Position { x: pos.x, y: pos.y, z: pos.z }; // this will be the position passed in
    let LBT: Position = Position { x: pos.x, y: pos.y, z: pos.z + 1 };

    let LFB: Position = Position { x: pos.x, y: pos.y - 1, z: pos.z }; 
    let LFT: Position = Position { x: pos.x, y: pos.y - 1, z: pos.z + 1 };

    let RBB: Position = Position { x: pos.x + 1, y: pos.y, z: pos.z };
    let RBT: Position = Position { x: pos.x + 1, y: pos.y, z: pos.z + 1 };

    let RFB: Position = Position { x: pos.x + 1, y: pos.y - 1, z: pos.z };
    let RFT: Position = Position { x: pos.x + 1, y: pos.y - 1, z: pos.z + 1 };

    // now make the tris from all the positions and return it
    let top1Tri: Tris = Tris { v1: LFT.clone(), v2: LBT.clone(), v3: RBT.clone() };
    let top2Tri: Tris = Tris { v1: LFT.clone(), v2: RFT.clone(), v3: RBT.clone() };

    let bottom1Tri: Tris = Tris { v1: LBB.clone(), v2: LFB.clone(), v3: RFB.clone() };
    let bottom2Tri: Tris = Tris { v1: LBB.clone(), v2: RBB.clone(), v3: RFB.clone() };

    let left1Tri: Tris = Tris { v1: LBB.clone(), v2: LBT.clone(), v3: LFT.clone() };
    let left2Tri: Tris = Tris { v1: LBB.clone(), v2: LFB.clone(), v3: LFT.clone() };

    let right1Tri: Tris = Tris { v1: RFB.clone(), v2: RFT.clone(), v3: RBT.clone() };
    let right2Tri: Tris = Tris { v1: RFB.clone(), v2: RBB.clone(), v3: RBT.clone() };

    let front1Tri: Tris = Tris { v1: LFB.clone(), v2: LFT.clone(), v3: RFT.clone() };
    let front2Tri: Tris = Tris { v1: LFB.clone(), v2: RFB.clone(), v3: RFT.clone() };

    let back1Tri: Tris = Tris { v1: RBB.clone(), v2: RBT.clone(), v3: LBT.clone() };
    let back2Tri: Tris = Tris { v1: RBB.clone(), v2: LBB.clone(), v3: LBT.clone() };


    let mut trisArr: [Tris; 12] =  [top1Tri, top2Tri, 
                                    bottom1Tri, bottom2Tri, 
                                    left1Tri, left2Tri,
                                    right1Tri, right2Tri,
                                    front1Tri, front2Tri,
                                    back1Tri, back2Tri];

    return trisArr;

}


pub fn CreateNewBlock(blockType: BlockType, dynamic: bool, posX: i32, posY: i32, posZ: i16) -> Block {
    
    // if the block is dynamic then i create its own Position so i can change it,
    // other wise it gets a non mutable reference to a large table where all positions created are stored

    let mut newPos: Position = Position { x: posX, y: posY, z: posZ };

    let mut tris: [Tris; 12] = CreateTrisFromBlock(&newPos);
    
    let mut newBlock: Block = Block {
        blocksType: blockType,
        dynamic: dynamic,
        position: newPos,
        faces: tris,
    };

        return newBlock;
}

