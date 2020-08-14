use crate::camera;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniforms {
    view_proj: cgmath::Matrix4<f32>,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

impl Uniforms {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &camera::Camera) {
        self.view_proj = camera.build_view_projection_matrix();
    }
}
