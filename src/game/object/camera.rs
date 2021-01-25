use crate::*;
use crate::game::*;

#[derive (Default, Clone, Debug)]
pub struct Camera {
    transform: Transform,
    renderer: CameraRenderer,
    fps_controller: FPSController,
    input: Input
}

impl CameraModel for Camera {
    fn get_renderer(&self) -> &CameraRenderer {
        &self.renderer
    } 

    fn get_renderer_mut(&mut self) -> &mut CameraRenderer {
        &mut self.renderer
    } 
}

impl HasComponents for Camera {
    fn update_components(&mut self, dt: f32) {
        self.transform.update(dt);
    }
}

impl HasControllers for Camera {
    fn update_controllers(&mut self, dt: f32) {
        let mut clone = self.fps_controller.clone();

        clone.update(self, dt);

        self.fps_controller = clone;
    }
}

impl HasController<Camera, FPSController> for Camera {
    fn attach_controller(&mut self, controller: FPSController) {
        self.fps_controller = controller;
    }

    fn get_controller(&self) -> &FPSController {
        &self.fps_controller
    }
}

impl HasComponent<Input> for Camera {
    fn attach_component(&mut self, controller: Input) {
        self.input = controller;
    }

    fn get_component(&self) -> Result<&Input, &'static str> {
        Ok(&self.input)
    }

    fn get_component_mut(&mut self) -> Result<&mut Input, &'static str> {
        Ok(&mut self.input)
    }
}

impl HasComponent<Transform> for Camera {
    fn get_component(&self) -> Result<&Transform, &'static str> {
        Ok(&self.transform)
    }

    fn get_component_mut(&mut self) -> Result<&mut Transform, &'static str> {
        Ok(&mut self.transform)
    }

    fn attach_component(&mut self, transform: Transform) {
        self.transform = transform;
    }
}