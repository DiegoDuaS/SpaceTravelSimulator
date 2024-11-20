# SpaceTravelSimulator
![image](https://github.com/user-attachments/assets/0394e284-be49-4fd1-84dd-100c71459a3a)

Proyecto Final: Gráficas por Computadora. Este proyecto simula un sistema solar con un sol y varios planetas alineados en un plano llamado plano eclíptico, el cual se origina debido a la conservación del momento angular durante la formación del sistema. Los planetas se trasladan en órbitas circulares y rotan sobre sus propios ejes. Además, se ha implementado una cámara interactiva con una nave que permite explorar los planetas desde diferentes perspectivas.

## Controles de la Nave y la Cámara

| Tecla       | Acción                                 |
|-------------|----------------------------------------|
| `W`         | Mover la cámara hacia arriba en el plano |
| `S`         | Mover la cámara hacia abajo en el plano |
| `A`         | Mover la cámara a la izquierda        |
| `D`         | Mover la cámara a la derecha          |
| `Q`         | Mover la cámara hacia adelante (acercar) |
| `E`         | Mover la cámara hacia atrás (alejar)  |
| `Izquierda` | Orbitar la cámara hacia la izquierda  |
| `Derecha`   | Orbitar la cámara hacia la derecha    |
| `Arriba`    | Acercar la cámara                     |
| `Abajo`     | Alejar la cámara                      |
| `B`         | Cambiar a vista superior (bird's-eye) |
| `V`         | Restablecer la cámara a su posición original |

## Ejectar el Proyecto 

1. Clona el repositorio en tu máquina
```bash
git clone https://github.com/DiegoDuaS/SpaceTravelSimulator.git
cd CelestialBodies_Shaders
```

2. Compila el código
```bash
cargo build --release
```

3. Corre la versión optimizada
```bash
 ./target/release/SpaceTravelSimulator
```

## Preview del Proyecto

Puedes ver un video del proyecto [aquí](https://youtu.be/aXQaL_PY0oM).
