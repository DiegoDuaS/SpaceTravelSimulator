use fastnoise_lite::FastNoiseLite;
use nalgebra_glm::{Vec3, Vec4, Mat3, dot, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use std::f32::consts::PI;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transform position
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  // Perform perspective division
  let w = transformed.w;
  let ndc_position = Vec4::new(
    transformed.x / w,
    transformed.y / w,
    transformed.z / w,
    1.0
  );

  // apply viewport matrix
  let screen_position = uniforms.viewport_matrix * ndc_position;

  // Transform normal
  let model_mat3 = mat4_to_mat3(&uniforms.model_matrix); 
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;

  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
    transformed_normal,
  }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, index: usize) -> Color {
  match index {
      0 => sun_shader(fragment, uniforms),    
      1 => lava_planet_shader(fragment, uniforms), 
      2 => planet1_shader(fragment, uniforms),  
      3 => earth_map_shader(fragment, uniforms), 
      4 => water_shader(fragment, uniforms), 
      5 => gas_giant_shader(fragment, uniforms), 
      6 => rock_shader(fragment, uniforms),
      _ => rock_shader(fragment, uniforms), 
  }
}


pub fn earth_map_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para diferentes tonalidades de océano y tierra
  let deep_ocean_color = Color::new(0, 70, 130);     // Azul oscuro para océano profundo
  let shallow_ocean_color = Color::new(0, 105, 148); // Azul más claro para aguas poco profundas
  let beach_color = Color::new(237, 201, 175);       // Color de playa entre océano y tierra
  let lowland_color = Color::new(34, 139, 34);       // Verde oscuro para tierra baja
  let highland_color = Color::new(85, 170, 85);      // Verde claro para tierras altas
  let polar_color = Color::new(255, 255, 255);       // Blanco para zonas polares

  // Coordenadas de mapa para el fragmento
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  // Definición detallada de continentes usando ruido fractal
  let land_factor = (
      (x * 3.0).sin() * (y * 4.5).cos() * 0.7 +
      (x * 1.5 + y * 2.0).sin() * 0.3 +
      (x * 5.5).cos() * (y * 3.5).sin() * 0.2 +
      ((x * 10.0).sin() * (y * 10.0).cos()).sin() * 0.1 +
      ((x * 15.0 + y * 1.5).sin() * 0.5 + 0.5) * 0.15 +
      ((x * 20.0).sin() * (y * 18.0).cos()).cos() * 0.05
  ) * 0.5 + 0.5;

  // Selección de color para el océano y la tierra según el `land_factor`
  let base_color = if y.abs() > 0.8 {
      polar_color // Zonas polares
  } else if land_factor < 0.45 {
      // Océano profundo a aguas poco profundas
      deep_ocean_color.lerp(&shallow_ocean_color, land_factor / 0.45)
  } else if land_factor < 0.5 {
      // Playa
      shallow_ocean_color.lerp(&beach_color, (land_factor - 0.45) / 0.05)
  } else if land_factor < 0.8 {
      // Tierras bajas a altas
      beach_color.lerp(&lowland_color, (land_factor - 0.5) / 0.3)
  } else {
      // Zonas de alta altitud
      lowland_color.lerp(&highland_color, (land_factor - 0.8) / 0.2)
  };

  // Cálculo del ruido de las nubes
  let zoom = 50.0;  // Para mover nuestros valores
  let oy = 50.0;
  let t = uniforms.time as f32 * 0.5;

  let noise_value = uniforms.noise.get_noise_2d(x * zoom  + t, y * zoom + oy);

  // Umbral de nubes y colores
  let cloud_threshold = 0.5; // Ajusta este valor para cambiar la densidad de las nubes
  let cloud_color = Color::new(255, 255, 255); // Blanco para las nubes

  // Determina si el píxel es parte de una nube o del cielo
  let noise_color = if noise_value > cloud_threshold {
    cloud_color
  } else {
    base_color
  };

  // Mezcla la base del color de la tierra con el color de las nubes
  let cloud_effect = noise_color.lerp(&base_color, 0.5); // Mezcla las nubes con el color de la tierra

  // Ajusta el color final con la intensidad del fragmento
  cloud_effect * fragment.intensity.max(0.2)
}

pub fn water_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para simular distintas profundidades en el agua
  let deep_water_color = Color::new(0, 50, 150);    // Azul oscuro para agua profunda
  let shallow_water_color = Color::new(255, 255, 255); // Azul claro para agua superficial

  // Coordenadas del fragmento y el tiempo para animación
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let time = uniforms.time as f32 * 0.1; // Factor de tiempo para animación

  // Calcula las coordenadas rotadas en 90 grados en sentido horario
  let rotated_x = y;
  let rotated_y = -x;

  // Generación de ruido de ondas en capas para la superficie del agua
  let wave_noise = uniforms.noise.get_noise_3d(rotated_x * 5.0 * 10.0, rotated_y * 5.0 * 10.0, time) * 0.5 + 0.5;

  // Ajusta el color del agua de acuerdo al valor del ruido
  let water_color = deep_water_color.lerp(&shallow_water_color, wave_noise);

  // Crea distorsiones adicionales usando ruido para simular la dinámica del agua
  let distortion_x = uniforms.noise.get_noise_3d(rotated_x * 20.0 + time * 0.3, rotated_y * 20.0, time) * 0.05;
  let distortion_y = uniforms.noise.get_noise_3d(rotated_x * 20.0, rotated_y * 20.0 + time * 0.3, time) * 0.05;

  // Aplica la distorsión en la posición de las ondas
  let final_x = rotated_x + distortion_x;
  let final_y = rotated_y + distortion_y;
  let surface_noise = uniforms.noise.get_noise_3d(final_x * 10.0, final_y * 10.0, time) * 0.5 + 0.5;

  // Ajusta el color final mezclando el color base y la distorsión de la superficie
  let final_color = water_color.lerp(&deep_water_color, surface_noise * 0.5);

  // Ajusta el color final con la intensidad del fragmento para dar brillo o sombra
  final_color * fragment.intensity.max(0.2)
}

pub fn rock_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para simular la textura rocosa del asteroide
  let dark_rock_color = Color::new(15, 10, 5);    // Marrón muy oscuro para áreas sombreadas
  let light_rock_color = Color::new(200, 150, 100); // Marrón claro más brillante para áreas iluminadas
  let black_color = Color::new(0, 0, 0);          // Negro para mayor profundidad y contraste

  // Coordenadas del fragmento
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  // Generación de ruido para simular la superficie rocosa
  // Utilizamos diferentes escalas para amplificar la variabilidad del ruido
  let base_noise = uniforms.noise.get_noise_3d(x * 5.0, y * 5.0, 0.0) * 0.5 + 0.5; // Ruido base
  let noise_variation = uniforms.noise.get_noise_3d(x * 30.0, y * 30.0, 0.0) * 0.5 + 0.5; // Ruido adicional para variabilidad

  // Ajustamos la intensidad del ruido base para crear un rango más amplio entre oscuro y claro
  let adjusted_noise = base_noise * 0.7 + noise_variation * 0.3; // Mezcla de ruidos

  // Interpolación de colores en función del valor del ruido
  // Ajusta el color de la superficie con tonos de roca utilizando el ruido generado
  let base_rock_color = dark_rock_color.lerp(&light_rock_color, adjusted_noise);

  // Usamos el ruido para darle más profundidad, haciendo las áreas más oscuras más pronunciadas
  let final_color = base_rock_color.lerp(&black_color, adjusted_noise * 0.5);

  // Ajusta el color final con la intensidad del fragmento para simular sombras
  final_color * fragment.intensity.max(0.2)
}

pub fn sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para simular el sol con tonos de naranja, amarillo y rojo
  let core_color = Color::new(255, 100, 50);    // Naranja brillante para el núcleo del sol
  let mid_color = Color::new(255, 165, 50);     // Amarillo anaranjado para la parte media
  let outer_glow_color = Color::new(255, 220, 100); // Amarillo pálido para el resplandor exterior
  let red_glow_color = Color::new(255, 50, 0);  // Rojo para acentuar el resplandor caliente en los bordes

  // Coordenadas del fragmento y el tiempo
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let time = uniforms.time as f32 * 0.1; // Tiempo para animar las manchas solares

  // Generación de ruido para simular las variaciones en la superficie del sol
  let surface_noise = uniforms.noise.get_noise_3d(x * 50.0 , y * 50.0, time) * 0.5 + 0.5;

  // Mezcla de colores del núcleo y el resplandor exterior usando el ruido
  let sun_surface_color = core_color.lerp(&mid_color, surface_noise); // Transición entre naranja y amarillo
  let final_color = sun_surface_color.lerp(&outer_glow_color, surface_noise * 0.5); // Mezcla con amarillo pálido

  // Añadir un toque de rojo en los bordes para resaltar el calor intenso
  let distance_from_center = (x * x + y * y).sqrt(); // Distancia al centro del sol
  let glow_intensity = 1.0 / (distance_from_center + 1.0); // Resplandor disminuye con la distancia
  let red_glow = red_glow_color * glow_intensity * 0.6; // Intensidad del resplandor rojo en los bordes

  // Mezclar el color final con el resplandor rojo
  let final_color_with_red = final_color + red_glow;

  // Ajusta el color final con la intensidad del fragmento para simular el brillo
  final_color_with_red
}

pub fn planet1_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para simular un planeta rocoso extraterrestre
  let deep_purple_color = Color::new(110, 60, 160);    // Morado oscuro suave para áreas rocosas
  let light_purple_color = Color::new(130, 90, 190);    // Púrpura suave, más cercano al morado oscuro
  let lavender_color = Color::new(150, 110, 210);       // Lavanda, ligeramente más claro pero manteniendo la saturación
  let violet_color = Color::new(140, 80, 200);      // Violeta para áreas de vegetación alienígena
  let very_dark_purple_color = Color::new(30, 10, 40);

  // Coordenadas del fragmento (sin animación de tiempo)
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  // Generación de ruido para simular la textura de la superficie rocoso
  // El ruido ya no depende del tiempo, asegurando que la textura sea estática
  let noise_value = uniforms.noise.get_noise_3d(x * 60.0, y * 60.0, 0.0) * 0.5 + 0.5;

  // Mapeo de valores de ruido a diferentes colores de superficie para un planeta rocoso
  let planet_color = if noise_value < 0.2 {
      very_dark_purple_color
  } else if noise_value < 0.3 {
      // Áreas rocosas o desérticas con morado oscuro
      deep_purple_color
  } else if noise_value < 0.5 {
      // Áreas con terreno más suave con púrpura claro
      light_purple_color
  } else if noise_value < 0.7 {
      // Áreas con vegetación alienígena de color lavanda
      lavender_color
  } else {
      // Áreas con vegetación alienígena densa o características únicas, como un violeta intenso
      violet_color
  };

  // Ajusta el color final con la intensidad del fragmento para simular iluminación o sombra
  planet_color * fragment.intensity.max(0.2)
}

pub fn gas_giant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para simular un planeta gaseoso
  let cloud_base_color = Color::new(240, 240, 240);  // Blanco/Gris suave para las nubes del gas
  let gas_color_1 = Color::new(200, 150, 50);        // Amarillo cálido para gas
  let gas_color_2 = Color::new(50, 100, 255);        // Azul suave para gas frío
  let gas_color_3 = Color::new(120, 90, 200);        // Morado tenue para gas frío

  // Coordenadas del fragmento y el tiempo para animación
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let time = uniforms.time as f32 * 0.1; // Tiempo para animar el gas

  // Generación de ruido para simular la dinámica del gas (nubes moviéndose)
  let surface_noise = uniforms.noise.get_noise_3d(x * 15.0, y * 15.0, time) * 0.5 + 0.5;
  
  // El color base del planeta gaseoso se ajusta según el ruido para simular nubes
  let cloud_color = cloud_base_color.lerp(&gas_color_1, surface_noise);

  // Agregar un toque de gas de diferentes colores (más detalles)
  let gas_noise_1 = uniforms.noise.get_noise_3d(x * 20.0, y * 20.0, time * 0.5) * 0.5 + 0.5;
  let gas_noise_2 = uniforms.noise.get_noise_3d(x * 10.0, y * 10.0, time * 0.3) * 0.5 + 0.5;
  
  // Mezclar los colores con un gradiente sutil usando ruido
  let gas_color_1 = gas_color_1.lerp(&gas_color_2, gas_noise_1);
  let gas_color_2 = gas_color_2.lerp(&gas_color_3, gas_noise_2);
  
  // Ajustar el color final con un mix de los diferentes gases
  let final_gas_color = cloud_color.lerp(&gas_color_1, gas_noise_1);
  let final_color = final_gas_color.lerp(&gas_color_2, gas_noise_2);

  final_color * fragment.intensity.max(0.2)
}

pub fn lava_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para simular un planeta rocoso y lava
  let rock_color = Color::new(100, 60, 30);    // Café para las rocas
  let lava_color = Color::new(255, 69, 0);     // Naranja brillante para lava
  let cooled_lava_color = Color::new(160, 82, 45); // Naranja más oscuro para lava enfriada
  let hot_lava_color = Color::new(255, 0, 0);  // Rojo brillante para lava caliente

  // Coordenadas del fragmento y el tiempo
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let time = uniforms.time as f32 * 0.05; // Tiempo para animar la lava (más lento)

  // Generación de ruido para simular la variación de la superficie
  let surface_noise = uniforms.noise.get_noise_3d(x * 150.0, y * 150.0, time) * 0.5 + 0.5;

  // Generar una variación de lava (el ruido se mueve para simular la lava)
  let lava_noise = uniforms.noise.get_noise_3d(x * 100.0, y * 100.0, time * 0.2) * 0.5 + 0.5;

  // El color base del planeta rocoso (rocas) se ajusta según el ruido
  // La interpolación entre el café y la lava se ajusta para hacer la lava más visible
  let rock_surface = rock_color.lerp(&lava_color, lava_noise * 0.4);  // Aumentar la influencia de la lava

  // El color de la lava en la superficie dependerá de la intensidad de la "lava"
  // Mezclamos el rojo brillante con el naranja dependiendo de la intensidad de la lava
  let lava_surface = lava_color.lerp(&hot_lava_color, lava_noise); // Lava más caliente será roja

  // El color de la lava más fría será un naranja más oscuro
  let cooled_lava_surface = cooled_lava_color.lerp(&lava_surface, surface_noise);

  // Mezcla el color de la roca con el de la lava (áreas de lava activa y enfriada)
  let final_surface_color = rock_surface.lerp(&cooled_lava_surface, surface_noise * 0.5); // Aumentar la mezcla de lava

  // Ajusta el color final con la intensidad del fragmento
  let final_color = final_surface_color * fragment.intensity.max(0.2);

  final_color
}












