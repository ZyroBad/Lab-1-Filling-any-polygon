# Lab 1 — Relleno de Polígonos

Proyecto desarrollado en **Rust** con **Raylib** para dibujar y rellenar polígonos mediante el algoritmo **Scanline Fill**. Los bordes se trazan con **Bresenham** y el polígono 5 funciona como un agujero dentro del polígono 4.

## Resultado

![Resultado del programa](out.png)

## Ejecución

```bash
cargo run
```

Al ejecutar el programa se genera el archivo:

```text
out.png
```

## Estructura

```text
src/
├── main.rs
├── framebuffer.rs
├── line.rs
└── polygon.rs
```

## Algoritmos utilizados

- **Scanline Fill:** rellena polígonos convexos y cóncavos.
- **Regla par-impar:** permite crear el agujero dentro del polígono 4.
- **Bresenham:** dibuja los bordes de los polígonos.

## Requisitos

- Rust
- Cargo
- Raylib 6.0
- CMake

## Archivos de entrega

```text
Cargo.toml
Cargo.lock
src/
README.md
.gitignore
out.png
```

La carpeta `target/` no debe subirse al repositorio.

Biblioteca
/
Chebas
/
README.md
# Lab 1 — Relleno de Polígonos

Proyecto desarrollado en **Rust** con **Raylib** para dibujar y rellenar polígonos mediante el algoritmo **Scanline Fill**. Los bordes se trazan con **Bresenham** y el polígono 5 funciona como un agujero dentro del polígono 4.

## Resultado

![Resultado del programa](out.png)

## Ejecución

```bash
cargo run
```

Al ejecutar el programa se genera el archivo:

```text
out.png
```

## Estructura

```text
src/
├── main.rs
├── framebuffer.rs
├── line.rs
└── polygon.rs
```

## Algoritmos utilizados

- **Regla par-impar:** permite crear el agujero dentro del polígono 4.
- **Bresenham:** dibuja los bordes de los polígonos.

## Requisitos

- Rust
- Cargo
- Raylib 6.0
- CMake

