use crate::World::FPosition;



pub struct Camera {
    pub fov: f32,
    pub aspectRatio: f32,
    pub nearPlane: f32,
    pub farPlane: f32,

    pub position: FPosition,
    pub target: FPosition,

}


impl Camera {
    pub fn new(fov: f32, screenWidth: u32, screenHeight: u32) -> Camera {
        let aspectRatio: f32 = screenWidth as f32 / screenHeight as f32;
        let nearPlane: f32 = 0.1;
        let farPlane: f32 = 100.0;

        // initial position and target
        let position: FPosition = FPosition::new(0.0, 2.0, -5.0);
        let target: FPosition = FPosition::new(0.0, 0.0, 0.0);

        Camera {
            fov: fov.to_radians(),
            aspectRatio,
            nearPlane,
            farPlane,

            position,
            target,
        }
    }
}