use nalgebra_glm::{look_at, perspective, Mat4, Vec3, Vec4};
use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;
use rand::Rng;

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;
mod cuerpo;
mod spaceship;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use triangle::triangle;
use shaders::{vertex_shader, fragment_shader};
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};
use cuerpo::Cuerpo;
use spaceship::Spaceship;



pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite
}

fn create_noise(_index: usize) -> FastNoiseLite {
    match _index {
        0 => create_sun_noise(),    
        1 => create_plant3_noise(), 
        2 => create_plant1_noise(),
        3 => create_cloud_noise(), 
        4 => create_water_noise(), 
        5 => create_plant2_noise(), 
        6 => create_rock_noise(),
        _ => create_rock_noise(), 
    }
}

fn create_cloud_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(12345);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_gain(Some(0.5));  
    noise.set_frequency(Some(0.005));
    noise
}

fn create_water_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(34526);
    noise.set_noise_type(Some(NoiseType::Cellular)); // Tipo de ruido: Celular
    noise.set_fractal_type(Some(FractalType::FBm));  // Tipo de fractal, en este caso FBm
    noise.set_frequency(Some(0.1));   
    noise.set_fractal_octaves(Some(2));              
    noise
}

fn create_rock_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(12345);
    noise.set_noise_type(Some(NoiseType::Cellular));
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_frequency(Some(0.1));                
    noise
}

fn create_sun_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(12345);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));  // Perlin noise for smooth, natural texture
    noise.set_fractal_type(Some(FractalType::Ridged)); // FBm for layered detail
    noise.set_fractal_octaves(Some(1));             // High octaves for rich detail
    noise.set_fractal_lacunarity(Some(2.49));        // Higher lacunarity = more contrast between layers
    noise.set_fractal_gain(Some(0.8));              // Higher gain = more influence of smaller details
    noise.set_frequency(Some(0.064));                // Low frequency = large features
    noise
}

fn create_plant1_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(121);
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_fractal_type(Some(FractalType::PingPong));
    noise.set_fractal_octaves(Some(2));    
    noise.set_frequency(Some(0.015));     
    noise.set_fractal_lacunarity(Some(2.49));    
    noise.set_fractal_gain(Some(0.8));         
    noise
}

fn create_plant2_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(111);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_fractal_type(Some(FractalType::PingPong));
    noise.set_fractal_octaves(Some(2));    
    noise.set_frequency(Some(0.040));     
    noise.set_fractal_lacunarity(Some(6.27));    
    noise.set_fractal_gain(Some(0.8));         
    noise
}

fn create_plant3_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(92);
    noise.set_noise_type(Some(NoiseType::Cellular));
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise.set_fractal_octaves(Some(1));    
    noise.set_frequency(Some(0.013));     
    noise.set_fractal_lacunarity(Some(6.27));    
    noise.set_fractal_gain(Some(0.8));         
    noise
}

fn create_solar_system() -> Vec<Cuerpo> {
    let mut rng = rand::thread_rng();
    vec![
        Cuerpo {
            name: "Sol".to_string(),
            translation: Vec3::new(2.0, 0.0, -5.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: 1.0f32,
            vertex_array: Obj::load("assets/models/cuerpo2.obj")
                .expect("Failed to load sol model")
                .get_vertex_array(),
            orbit_radius: 0.0,
            phase_offset: rng.gen_range(0.0..std::f32::consts::TAU),
            rotation_speed: 0.0
        },
        Cuerpo {
            name: "Volcanis".to_string(),
            translation: Vec3::new(4.0, 0.0, -8.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: 0.3f32,
            vertex_array: Obj::load("assets/models/cuerpo2.obj")
                .expect("Failed to load Volcanis model")
                .get_vertex_array(),
            orbit_radius: 2.0,
            phase_offset: rng.gen_range(0.0..std::f32::consts::TAU),
            rotation_speed: rng.gen_range(0.01..0.05)
        },
        Cuerpo{
            name: "Morveth".to_string(),
            translation: Vec3::new(4.0, 0.0, -8.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: 0.4f32,
            vertex_array: Obj::load("assets/models/cuerpo2.obj")
                .expect("Failed to load Morveth model")
                .get_vertex_array(),
            orbit_radius: 3.0,
            phase_offset: rng.gen_range(0.0..std::f32::consts::TAU),
            rotation_speed: rng.gen_range(0.01..0.05)
        },
        Cuerpo{
            name: "GaiaNova".to_string(),
            translation: Vec3::new(4.0, 0.0, -8.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale:  0.4f32,
            vertex_array: Obj::load("assets/models/cuerpo2.obj")
                .expect("Failed to load GaiaNova model")
                .get_vertex_array(),
            orbit_radius: 4.0,
            phase_offset: rng.gen_range(0.0..std::f32::consts::TAU),
            rotation_speed: rng.gen_range(0.01..0.05)
        },
        Cuerpo{
            name: "Aquarion".to_string(),
            translation: Vec3::new(4.0, 0.0, -8.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale:  0.3f32,
            vertex_array: Obj::load("assets/models/cuerpo2.obj")
                .expect("Failed to load Aquarion model")
                .get_vertex_array(),
            orbit_radius: 5.0,
            phase_offset: rng.gen_range(0.0..std::f32::consts::TAU),
            rotation_speed: rng.gen_range(0.01..0.05)
        },
        Cuerpo{
            name: "Stratos".to_string(),
            translation: Vec3::new(4.0, 0.0, -8.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: 0.8f32,
            vertex_array: Obj::load("assets/models/saturno2.obj")
                .expect("Failed to load Startos model")
                .get_vertex_array(),
            orbit_radius: 7.0,
            phase_offset: rng.gen_range(0.0..std::f32::consts::TAU),
            rotation_speed: rng.gen_range(0.01..0.05)
        },
        Cuerpo{
            name: "KratonV".to_string(),
            translation: Vec3::new(4.0, 0.0, -8.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: 0.09f32,
            vertex_array: Obj::load("assets/models/asteroide.obj")
                .expect("Failed to load KratonV model")
                .get_vertex_array(),
            orbit_radius: 8.0,
            phase_offset: rng.gen_range(0.0..std::f32::consts::TAU),
            rotation_speed: rng.gen_range(0.01..0.05)
        },
    ]
}


fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}


fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], index: usize) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            // Apply fragment shader
            let shaded_color = fragment_shader(&fragment, &uniforms, index);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let disappearance_buffer = 5.0;  
    let stars = generate_stars(500, framebuffer_width, framebuffer_height);
    
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Proyecto-SistemaSolar",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x000000); // Fondo negro

    // Camera parameters
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 20.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    let mut translation_nave = Vec3::new(1.5, 1.5, 19.0);
    let mut rotation_nave = Vec3::new(0.0, 1.0, 0.0);
    let scale = 0.03f32;

    let solar_system = create_solar_system();
    let mut time = 0;
    let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    
    // Parámetro para la velocidad de rotación
    let orbital_speed = 0.01;  // Controla la velocidad de rotación de los planetas
    
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        time += 1;

        handle_input(&window, &mut camera, &mut translation_nave, &mut rotation_nave);

        framebuffer.clear();
        framebuffer.draw_stars(&stars); 

        for (index, body) in solar_system.iter().enumerate() {
            let noise = create_noise(index);
            
            let mut uniforms = Uniforms { 
                model_matrix: Mat4::identity(), 
                view_matrix: Mat4::identity(), 
                projection_matrix, 
                viewport_matrix, 
                time, 
                noise 
            };
        
            // Posición orbital
            let radius = body.orbit_radius;
            let angle = time as f32 * orbital_speed + body.phase_offset;
            let x_position = radius * angle.cos();
            let z_position = radius * angle.sin();
            let translation = Vec3::new(x_position, 0.0, z_position);
        
            // Rotación sobre su propio eje
            let rotation_angle = time as f32 * body.rotation_speed; // Calcula el ángulo de rotación
            let rotation_matrix = Mat4::from_axis_angle(&Vec3::y_axis(), rotation_angle); // Rotación en el eje Y
        
            // Crear la model matrix combinando traslación, rotación y escala
            let translation_matrix = Mat4::new_translation(&translation);
            let scale_matrix = Mat4::new_scaling(body.scale);
        
            uniforms.model_matrix = translation_matrix * rotation_matrix * scale_matrix;
            uniforms.view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
            uniforms.time = time;
        
            let camera_to_planet_distance = (camera.eye - &translation).magnitude();
            if camera_to_planet_distance <= body.scale + disappearance_buffer {
                continue; 
            }
        
            if is_in_camera(&translation, &uniforms.view_matrix, &uniforms.projection_matrix) {
                render(&mut framebuffer, &uniforms, &body.vertex_array, index);
            }
        }

        let noise = create_noise(6);

        let mut uniforms = Uniforms { 
            model_matrix: Mat4::identity(), 
            view_matrix: Mat4::identity(), 
            projection_matrix, 
            viewport_matrix, 
            time, 
            noise 
        };

        uniforms.model_matrix = create_model_matrix(translation_nave, scale, rotation_nave);
        uniforms.view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);

        let obj = Obj::load("assets/models/nave.obj").expect("Failed to load obj");
        let vertex_arrays = obj.get_vertex_array(); 

        render(&mut framebuffer, &uniforms,&vertex_arrays, 7);
        
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
    }
}


fn handle_input(window: &Window, camera: &mut Camera, translation_nave: &mut Vec3, rotation_nave: &mut Vec3) {
    let movement_speed = 1.0;
    let rotation_speed = PI / 50.0;
    let zoom_speed = 0.1;
    
    // Inicializamos original_camera con los valores predeterminados
    let mut original_camera: Option<(Vec3, Vec3, Vec3)> = Some((
        Vec3::new(0.0, 0.0, 20.0), // Posición original de la cámara
        Vec3::new(0.0, 0.0, 0.0),  // Centro de la cámara
        Vec3::new(0.0, 1.0, 0.0),  // Dirección "arriba" de la cámara
    ));

    // Cámara: controles de órbita
    if window.is_key_down(Key::Left) {
        camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {
        camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {
        camera.orbit(0.0, rotation_speed);
    }

    // Movimiento de la cámara
    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
        movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
        movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
        movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
        movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
        camera.move_center(movement);
    }

    // Zoom de la cámara
    if window.is_key_down(Key::Up) {
        camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down) {
        camera.zoom(-zoom_speed);
    }

    // Guardar la posición original de la cámara cuando se presiona 'B'
    if window.is_key_down(Key::B) {
        if original_camera.is_none() {
            // Guardamos la cámara solo si no ha sido guardada antes
            original_camera = Some((
                camera.eye.clone(), 
                camera.center.clone(), 
                camera.up.clone()
            ));
        }
        let solar_system_center = Vec3::new(0.0, 0.0, 0.0); // Asume que el centro del sistema solar es (0, 0, 0)
        let bird_eye_height = 10.0; // Altura de la vista superior
        camera.eye = Vec3::new(solar_system_center.x, bird_eye_height, solar_system_center.z);
        camera.center = solar_system_center;
        camera.up = Vec3::new(0.0, 0.0, -1.0); // Eje "arriba" apunta hacia el eje Z negativo
    }

    // Volver a la posición original (key 'V')
    if window.is_key_down(Key::V) {
        if let Some((original_eye, original_center, original_up)) = original_camera {
            camera.eye = original_eye;
            camera.center = original_center;
            camera.up = original_up;
            original_camera = None; // Restablecer la cámara original después de restaurarla
        }
    }

    // Actualizar la posición de la nave basada en la posición de la cámara
    let camera_direction = (camera.center - camera.eye).normalize();
    *translation_nave = camera.eye + camera_direction * 1.0; // Mantener la nave a 2 unidades frente a la cámara
    rotation_nave.y = camera_direction.y.atan2(camera_direction.x);  // La nave rota según el ángulo de la cámara
}



fn is_in_camera(position: &Vec3, view_matrix: &Mat4, projection_matrix: &Mat4) -> bool {
    // Convertir la posición a Vec4 (con w = 1.0 para objetos estáticos)
    let position_4d = Vec4::new(position.x, position.y, position.z, 1.0);

    // Aplicar las transformaciones (vista + proyección)
    let clip_space_pos = projection_matrix * view_matrix * position_4d;

    // Normalizar a NDC
    let x_ndc = clip_space_pos.x / clip_space_pos.w;
    let y_ndc = clip_space_pos.y / clip_space_pos.w;
    let z_ndc = clip_space_pos.z / clip_space_pos.w;

    // Comprobar si la posición está dentro del frustum
    // x, y dentro de [-1, 1] y z dentro de [0, 1]
    x_ndc >= -1.0 && x_ndc <= 1.0 &&
    y_ndc >= -1.0 && y_ndc <= 1.0 &&
    z_ndc >= 0.0 && z_ndc <= 1.0
}

fn generate_stars(num_stars: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut rng = rand::thread_rng();
    (0..num_stars)
        .map(|_| {
            (
                rng.gen_range(0..width),  // Posición X de la estrella
                rng.gen_range(0..height) // Posición Y de la estrella
            )
        })
        .collect()
}





