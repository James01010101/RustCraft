

use crate::World::FPosition;
use nalgebra::{Vector3, Point3, Matrix4};

pub struct Camera {
    pub fov: f32,
    pub aspectRatio: f32,
    pub nearPlane: f32,
    pub farPlane: f32,

    pub position: FPosition,
    pub target: FPosition,

    pub viewMatrix: Matrix4<f32>,
    pub projectionMatrix: Matrix4<f32>,

}


impl Camera {
    pub fn new(fov: f32, screenWidth: u32, screenHeight: u32) -> Camera {
        let aspectRatio: f32 = screenWidth as f32 / screenHeight as f32;
        let nearPlane: f32 = 0.1;
        let farPlane: f32 = 100.0;

        // initial position and target
        let position: FPosition = FPosition::new(0.0, 2.0, -5.0);
        let target: FPosition = FPosition::new(0.0, 0.0, 0.0);

        let viewMatrix: Matrix4<f32> = nalgebra::Isometry3::look_at_rh(
            &Point3::new(position.x, position.y, position.z), 
            &Point3::new(target.x, target.y, target.z), 
            &Vector3::y()
        ).to_homogeneous();

        let projectionMatrix: Matrix4<f32> = nalgebra::Perspective3::new(
            aspectRatio, 
            fov, 
            nearPlane, 
            farPlane
        ).to_homogeneous();

        Camera {
            fov: fov.to_radians(),
            aspectRatio,
            nearPlane,
            farPlane,

            position,
            target,

            viewMatrix,
            projectionMatrix,
        }
    }

    
    // Calculate the view matrix
    pub fn calculate_view_matrix(&self) -> Matrix4<f32> {
        nalgebra::Isometry3::look_at_rh(
            &Point3::new(self.position.x, self.position.y, self.position.z), 
            &Point3::new(self.target.x, self.target.y, self.target.z), 
            &Vector3::y()
        ).to_homogeneous()
    }

    //TODO: #76 update the projection matrix only if the fov changes or the aspexct ratio changes
    // calculate the projection matrix, this shouldnt change unless fov changes, or aspect ratio changes
    pub fn calculate_projection_matrix(&self) -> Matrix4<f32> {
        nalgebra::Perspective3::new(
            self.aspectRatio, 
            self.fov, 
            self.nearPlane, 
            self.farPlane
        ).to_homogeneous()
    }

}