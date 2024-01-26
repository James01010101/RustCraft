use crate::Objects::Position;



pub struct Camera {
    fov: f32,
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,

    cameraPosition: Position,
    cameraTarget: Position,

}


impl Camera {
    pub fn new(fov: f32, screenWidth: u32, screenHeight: u32) {
        let aspect_ratio: f32 = screenWidth as f32 / screenHeight as f32;
        let near_plane: f32 = 0.1;
        let far_plane: f32 = 100.0;

        // initial position and target
        let cameraPosition: Position = Position::new(0.0, 0.0, -5.0);
        let cameraTarget: Position = Position::new(0.0, 0.0, 0.0);

        Camera {
            fov: fov,
            aspect_ratio,
            near_plane,
            far_plane,

            cameraPosition,
            cameraTarget,
        }
    }
}