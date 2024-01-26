use crate::World::Position;

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

// all of the verticies of a square
pub struct SquareVerts {
    LBB: Position,
    LBT: Position,

    LFB: Position, 
    LFT: Position,

    RBB: Position,
    RBT: Position,

    RFB: Position,
    RFT: Position,
}


// the main strut to hold all info related to a block
// previously 448 Bytes now 136 static (not including vector faces)
pub struct Block {
    // what kind of object is it
    blocksType: BlockType,

    // will it be moving frame by frame? so i know if i have to recalc it each frame or just when it changes
    dynamic: bool,

    position: Position,

    // Should add a vector of tris here that are pre computed, so its one less thing i need to calculate
    // every time i need to check for collisions, there will be exactly 12

    verts: SquareVerts,

    faces: Vec<Tris>,
    numFaces: u8, // so the gpu can loop through all faces

}


// TODO: #32 create the tris from the blocks that are touching air
// from the position of a block create all tris
pub fn CreateTrisFromBlock(pos: &Position) {

    // now make the tris from all the positions and return it
    /* Old Code
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
    */

}

pub fn CalculateSqureVerts(pos: &Position) -> SquareVerts {
    let mut sv: SquareVerts = SquareVerts {

        // Name (xyz) or (Left/Right, Front/Back, Top/Bottom)
        LBB: Position { x: pos.x, y: pos.y, z: pos.z }, // this will be the position passed in
        LBT: Position { x: pos.x, y: pos.y, z: pos.z + 1 },
        LFB: Position { x: pos.x, y: pos.y - 1, z: pos.z },
        LFT: Position { x: pos.x, y: pos.y - 1, z: pos.z + 1 },
        RBB: Position { x: pos.x + 1, y: pos.y, z: pos.z },
        RBT: Position { x: pos.x + 1, y: pos.y, z: pos.z + 1 },
        RFB: Position { x: pos.x + 1, y: pos.y - 1, z: pos.z },
        RFT: Position { x: pos.x + 1, y: pos.y - 1, z: pos.z + 1 },

    };

    return sv;
}


pub fn CreateNewBlock(blockType: BlockType, dynamic: bool, posX: i32, posY: i16, posZ: i32) -> Block {
    
    // if the block is dynamic then i create its own Position so i can change it,
    // other wise it gets a non mutable reference to a large table where all positions created are stored

    let mut newPos: Position = Position { x: posX, y: posY, z: posZ };

    let mut tris: Vec<Tris> = Vec::new();

    let mut verts: SquareVerts = CalculateSqureVerts(&newPos);
    
    let mut newBlock: Block = Block {
        blocksType: blockType,
        dynamic: dynamic,
        position: newPos,
        faces: tris,
        numFaces: 0,
        verts: verts,
    };

        return newBlock;
}

