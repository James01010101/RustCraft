

use crate::World::FPosition;
use crate::Renderer::*;
use crate::GPUData::*;

use nalgebra::{Vector3, Point3, Matrix4};

pub struct Camera {
    pub fov: f32,
    pub aspectRatio: f32,
    pub nearPlane: f32,
    pub farPlane: f32,

    pub position: FPosition,
    pub target: FPosition,

    pub projection_matrix: Matrix4<f32>,
    pub view_matrix: Matrix4<f32>,

    // this is the view * proj matrix, so the gpu doesnt have to do it for each vertex
    pub projection_view_matrix: [[f32; 4]; 4],

}


impl Camera {
    pub fn new(fov: f32, screenWidth: u32, screenHeight: u32) -> Camera {

        // fov is in degrees
        let fov: f32 = fov.to_radians();

        let aspectRatio: f32 = screenWidth as f32 / screenHeight as f32;
        let nearPlane: f32 = 0.1;
        let farPlane: f32 = 100.0;

        // initial position and target
        let position: FPosition = FPosition::new(0.0, 2.0, -5.0);
        let target: FPosition = FPosition::new(0.0, 0.0, 0.0);

        // calc these as matricies so they can be multiplied together
        let projection_matrix: Matrix4<f32> = nalgebra::Perspective3::new(
            aspectRatio, 
            fov, 
            nearPlane, 
            farPlane).to_homogeneous();

        let view_matrix: Matrix4<f32> = nalgebra::Perspective3::new(
            aspectRatio, 
            fov, 
            nearPlane, 
            farPlane
        ).to_homogeneous();

        
        // using nalgebra to multiply the view and projection matrix together
        let projection_view_matrix: [[f32; 4]; 4] = (projection_matrix * view_matrix).into();


    
        Camera {
            fov,
            aspectRatio,
            nearPlane,
            farPlane,

            position,
            target,

            projection_matrix,
            view_matrix,

            projection_view_matrix,
        }
    }

    
    // Calculate the view matrix
    pub fn calculate_view_matrix(&mut self) {
        self.view_matrix = nalgebra::Isometry3::look_at_rh(
            &Point3::new(self.position.x, self.position.y, self.position.z), 
            &Point3::new(self.target.x, self.target.y, self.target.z), 
            &Vector3::y()
        ).to_homogeneous().into();

        // update the proj view matrix 
        self.projection_view_matrix = (self.projection_matrix * self.view_matrix).into();
    }

    //TODO: #76 update the projection matrix only if the fov changes or the aspexct ratio changes
    // calculate the projection matrix, this shouldnt change unless fov changes, or aspect ratio changes
    pub fn calculate_projection_matrix(&mut self) {
        self.projection_matrix = nalgebra::Perspective3::new(
            self.aspectRatio, 
            self.fov, 
            self.nearPlane, 
            self.farPlane
        ).to_homogeneous().into();

        // update the proj view matrix
        self.projection_view_matrix = (self.projection_matrix * self.view_matrix).into();
    }


    // this is called once per frame and will update the cameras projection and view matricies and send them to the staging buffer
    pub fn update(&mut self, renderer: &mut Renderer, gpuData: &GPUData) {

        // update the view matrix and the combined
        self.calculate_view_matrix();

        // update the uniform buffer with the new camera position matricies
        renderer.vertUniforms.projection_view_matrix = self.projection_view_matrix;

        // update the gpus staging buffer
        // update the staging gpu buffers and set the flag that this data has changed
        renderer.queue.write_buffer(&gpuData.vertex_uniform_staging_buf, 0, bytemuck::bytes_of(&renderer.vertUniforms));

        // submit those write buffers so they are run
        renderer.queue.submit(std::iter::empty());

    }

}