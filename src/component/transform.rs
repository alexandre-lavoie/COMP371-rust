use crate::component::Component;

#[derive(Clone, Debug)]
pub struct Transform {
    position: [f32; 3],
    rotation: [f32; 3],
    scale: [f32; 3],
    matrix: [f32; 16],
    matrix_update: bool,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: [0f32; 3],
            rotation: [0f32; 3],
            scale: [1f32; 3],
            matrix: [0f32; 16],
            matrix_update: true
        }
    }
}

impl Transform {
    pub fn get_matrix(&self) -> [f32; 16] {
        self.matrix
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        self.matrix_update = true;
        self.position = position;
    }

    pub fn delta_position(&mut self, delta: [f32; 3], dt: f32) {
        let mut position = self.get_position().clone();

        position[0] += delta[0] / dt;
        position[1] += delta[1] / dt;
        position[2] += delta[2] / dt;

        self.set_position(position);
    }

    pub fn get_position(&self) -> &[f32; 3] {
        &self.position
    }

    pub fn set_rotation(&mut self, rotation: [f32; 3]) {
        self.matrix_update = true;
        self.rotation = normalize_rotation(&rotation);
    }

    pub fn delta_rotation(&mut self, delta: [f32; 3], dt: f32) {
        let mut rotation = self.get_rotation().clone();

        rotation[0] += delta[0] * dt / 30.;
        rotation[1] += delta[1] * dt / 30.;
        rotation[2] += delta[2] * dt / 30.;

        self.set_rotation(rotation);
    }

    pub fn get_rotation(&self) -> &[f32; 3] {
        &self.rotation
    }

    pub fn set_scale(&mut self, scale: [f32; 3]) {
        self.matrix_update = true;
        self.scale = scale;
    }

    pub fn get_scale(&self) -> &[f32; 3] {
        &self.scale
    }

    pub fn get_forward(&self) -> [f32; 3] {
        let mut matrix = mat4::new_identity::<f32>();

        let matrix_clone = matrix.clone();

        let rotation = self.get_rotation();

        mat4::rotate(
            &mut matrix,
            &matrix_clone,
            &rotation[0].to_radians(),
            &rotation[1].to_radians(),
            &rotation[2].to_radians(),
        );

        return [-matrix[1], -matrix[5], -matrix[9]];
    }

    pub fn get_left(&self) -> [f32; 3] {
        let mut matrix = mat4::new_identity::<f32>();

        let matrix_clone = matrix.clone();

        let rotation = self.get_rotation();

        mat4::rotate(
            &mut matrix,
            &matrix_clone,
            &rotation[0].to_radians(),
            &rotation[1].to_radians(),
            &rotation[2].to_radians(),
        );

        return [matrix[0], matrix[4], matrix[8]];
    }

    fn calculate_matrix(&self) -> [f32; 16] {
        let mut matrix = mat4::new_identity::<f32>();

        let mut matrix_clone = matrix.clone();

        mat4::translate(&mut matrix, &matrix_clone, &self.get_position());

        matrix_clone = matrix.clone();

        let rotation = self.get_rotation();

        mat4::rotate(
            &mut matrix,
            &matrix_clone,
            &rotation[0].to_radians(),
            &rotation[1].to_radians(),
            &rotation[2].to_radians(),
        );

        matrix_clone = matrix.clone();
        mat4::scale(&mut matrix, &matrix_clone, &self.get_scale());

        matrix
    }
}

impl Component for Transform {
    fn update(&mut self, dt: f32) {
        if self.matrix_update {
            self.matrix_update = false;

            self.matrix = self.calculate_matrix();
        }
    }
}

pub fn normalize_rotation(rotation: &[f32; 3]) -> [f32; 3] {
    let mut r = [0f32; 3];

    for (i, v) in rotation.iter().enumerate() {
        r[i] = if v < &0. {
            (360. - v.abs()) % 360.
        } else {
            v % 360.
        };
    }

    r
}
