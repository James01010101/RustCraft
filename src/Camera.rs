use crate::Objects::Position;



pub struct Camera {
    pub fov: f32,
    pub aspectRatio: f32,
    pub nearPlane: f32,
    pub farPlane: f32,

    pub cameraPosition: Position,
    pub cameraTarget: Position,

}


impl Camera {
    pub fn new(fov: f32, screenWidth: u32, screenHeight: u32) -> Camera {
        let aspectRatio: f32 = screenWidth as f32 / screenHeight as f32;
        let nearPlane: f32 = 0.1;
        let farPlane: f32 = 100.0;

        // initial position and target
        let cameraPosition: Position = Position::new(0, 0, 0);
        let cameraTarget: Position = Position::new(0, 0, 0);

        Camera {
            fov: fov,
            aspectRatio,
            nearPlane,
            farPlane,

            cameraPosition,
            cameraTarget,
        }
    }
}