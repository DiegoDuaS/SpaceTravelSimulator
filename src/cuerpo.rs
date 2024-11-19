use fastnoise_lite::FastNoiseLite;
use nalgebra_glm::Vec3;
use crate::vertex::Vertex;

pub struct Cuerpo {
    pub name: String,
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: f32,
    pub vertex_array: Vec<Vertex>, // Vertices del modelo 3D del planeta
    pub orbit_radius: f32,        // Radio de la órbita del planeta
    pub phase_offset: f32,
    pub rotation_speed: f32,
}