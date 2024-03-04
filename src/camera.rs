use crate::{
    character::*, 
    gpu_data::*,
    types::VertexUniforms,
};

use nalgebra::{Matrix4, Point3, Vector3};

use wgpu::Queue;

pub struct Camera {
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,

    pub screen_width: u32,
    pub screen_height: u32,

    pub projection_matrix: Matrix4<f32>,
    pub view_matrix: Matrix4<f32>,

    // this is the view * proj matrix, so the gpu doesnt have to do it for each vertex
    pub projection_view_matrix: [[f32; 4]; 4],
}

impl Camera {
    pub fn new(fov: f32, screen_width: u32, screen_height: u32) -> Camera {
        // fov is in degrees
        let fov: f32 = fov.to_radians();
        let aspect_ratio: f32 = screen_width as f32 / screen_height as f32;
        let near_plane: f32 = 0.1;
        let far_plane: f32 = 100.0;

        // calc these as matricies so they can be multiplied together
        let projection_matrix: Matrix4<f32> =
            nalgebra::Perspective3::new(aspect_ratio, fov, near_plane, far_plane).to_homogeneous();

        let view_matrix: Matrix4<f32> =
            nalgebra::Perspective3::new(aspect_ratio, fov, near_plane, far_plane).to_homogeneous();

        // using nalgebra to multiply the view and projection matrix together
        let projection_view_matrix: [[f32; 4]; 4] = (projection_matrix * view_matrix).into();

        Camera {
            fov,
            aspect_ratio,
            near_plane,
            far_plane,

            screen_width,
            screen_height,

            projection_matrix,
            view_matrix,

            projection_view_matrix,
        }
    }

    // Calculate the view matrix
    pub fn calculate_view_matrix(&mut self, character: &Character) {
        self.view_matrix = nalgebra::Isometry3::look_at_rh(
            &Point3::new(
                character.position.x,
                character.position.y,
                character.position.z,
            ),
            &Point3::new(character.target.x, character.target.y, character.target.z),
            &Vector3::y(),
        )
        .to_homogeneous()
        .into();

        // update the proj view matrix
        self.projection_view_matrix = (self.projection_matrix * self.view_matrix).into();
    }

    // calculate the projection matrix, this shouldnt change unless fov changes, or aspect ratio changes
    pub fn calculate_projection_matrix(&mut self) {
        self.projection_matrix = nalgebra::Perspective3::new(
            self.aspect_ratio,
            self.fov,
            self.near_plane,
            self.far_plane,
        )
        .to_homogeneous()
        .into();

        // dont need to update the proj view matrix, since it will be updated on the next view update
    }

    // this is called once per frame and will update the cameras projection and view matricies and send them to the staging buffer
    pub fn update(&mut self, queue: &Queue, vertex_uniforms: &mut VertexUniforms, gpu_data: &GPUData, character: &Character) {
        // update the view matrix and the combined
        self.calculate_view_matrix(&character);

        // update the uniform buffer with the new camera position matricies
        vertex_uniforms.projection_view_matrix = self.projection_view_matrix;

        // update the gpus staging buffer
        // update the staging gpu buffers and set the flag that this data has changed
        queue.write_buffer(
            &gpu_data.vertex_uniform_staging_buf,
            0,
            bytemuck::bytes_of(vertex_uniforms),
        );

        // submit those write buffers so they are run
        queue.submit(std::iter::empty());
    }
}
