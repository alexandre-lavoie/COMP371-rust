use crate::component::{HasComponents, HasComponent, Transform};
use crate::controller::HasControllers;
use crate::render::CameraRenderer;
use mat4;

pub trait CameraModel: HasComponents + HasControllers + HasComponent<Transform> {
    fn get_renderer(&self) -> &CameraRenderer;

    fn get_renderer_mut(&mut self) -> &mut CameraRenderer;

    fn calculate_camera_matrix(&self) -> [f32; 16] {
        let mut matrix = mat4::new_identity::<f32>();

        let mut matrix_clone = matrix.clone();

        let transform: &Transform = self.get_component().unwrap();

        let position = transform.get_position();

        let rotation = transform.get_rotation();

        mat4::rotate(
            &mut matrix,
            &matrix_clone,
            &(180. - rotation[0]).to_radians(),
            &rotation[1].to_radians(),
            &rotation[2].to_radians(),
        );

        matrix_clone = matrix.clone();

        mat4::translate(
            &mut matrix,
            &matrix_clone,
            &[-position[0], -position[1], -position[2]],
        );

        matrix
    }

    fn update_matrix(&mut self) {
        let matrix = self.calculate_camera_matrix().clone();

        let renderer = self.get_renderer_mut();

        renderer.set_camera_matrix(matrix);

        renderer.update_projection_matrix();
    }
}