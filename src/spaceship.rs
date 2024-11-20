use nalgebra_glm::Vec3;

pub struct Spaceship {
    pub position: Vec3,
    pub rotation: Vec3,
}

impl Spaceship {
    pub fn new() -> Self {
        Spaceship {
            position: Vec3::new(0.0, 0.0, 10.0), // A 10 metros frente a la cámara
            rotation: Vec3::new(0.0, 0.0, 0.0), // Sin rotación inicial
        }
    }

    pub fn update_position(&mut self, camera_position: &Vec3) {
        // Mantener la nave a 10 metros frente a la cámara
        let direction = (camera_position - self.position).normalize();
        self.position = camera_position + direction * 10.0;
    }
}
