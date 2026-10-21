# vector3

**vector3** is a Rust N-body gravitational particle simulator built around a custom `Vector3` math library, a Barnes–Hut octree for approximate force calculation, and a real-time 3D renderer (`three-d`). It was designed for parallel, cache-friendly particle updates (via `rayon`) and GPU-instanced rendering of large particle counts.

## Preview

| |
|---|
| ![Preview](docs/preview/preview_1.png) |

## Key Features

* **Barnes–Hut Octree:** `Octree<T>` recursively subdivides space into 8 children, tracks mass and center-of-mass per node (`propagate`), and approximates long-range gravity via a `theta` opening-angle criterion (`calculate_acceleration`).
* **Custom Vector3 Math:** Dot/cross products, normalization, reflection, kinetic energy, and Morton-code (Z-order curve) encoding for spatial sorting, implemented through the `VectorOps`, `Reflector`, `Kinetic`, and `Distributing` traits.
* **Verlet Integration:** Particles (`Particle`) advance via position-based Verlet integration (`Moving::update`), tracking `position`/`last_position` rather than explicit velocity.
* **Collision & Gravity Resolution:** Pairwise elastic collision response and gravitational acceleration are computed per-particle as `Correction` enum variants (`Collision`, `Gravity`, `Kill`) and applied afterward, avoiding aliasing during parallel iteration.
* **Parallelism:** Particle updates, octree queries, and correction computation are parallelized across cores with `rayon`.
* **Spatial Distribution Helpers:** Initial particle placement via Fibonacci-sphere (`distribute_across_the_sphere`) or torus (`distribute_across_the_torus`) sampling.
* **Real-Time 3D Rendering:** GPU-instanced spheres rendered with `three-d`, with orbit camera controls and FPS/particle-count logging once per second.
* **Structured Logging:** `tracing` + `tracing-subscriber` spans per simulation frame, configurable via `RUST_LOG`.

## Installation

### Requirements
* [Rust](https://www.rust-lang.org/) (2024 edition, stable toolchain) and `cargo`
* A GPU/driver capable of OpenGL (used by `three-d`)

### Build and Installation

```bash
git clone https://github.com/robertmon-dev/barnes-hutt-3d.git
cd barnes-hutt-3d

cargo build --release
```

### Running

```bash
cargo run --release
```

This opens a window and runs a Barnes–Hut gravitational simulation of particles distributed across a sphere.

## Configuration

Simulation parameters are set as constants/literals rather than environment variables:

* `src/main.rs` — particle count, world half-dimension, particle radius, render scale
* `src/attractor/consts.rs` — `G_CONST` (gravitational constant), `EPSILON` (softening factor), `THETA` (Barnes–Hut opening angle), `MAX_FACTOR` (acceleration clamp), `TREE_DEPTH` (max octree depth)

Logging verbosity is controlled via the standard `RUST_LOG` environment variable (e.g. `RUST_LOG=debug cargo run`), consumed by `tracing-subscriber`'s `EnvFilter`.

## Project Structure

```text
├── src/
│   ├── aabb/              # Axis-aligned bounding box (containment, intersection)
│   ├── attractor/         # Barnes-Hut gravity + collision resolution
│   │   ├── consts.rs       # G_CONST, EPSILON, THETA, MAX_FACTOR, TREE_DEPTH
│   │   └── correction.rs   # Correction enum (Collision / Gravity / Kill)
│   ├── logger/            # tracing / tracing-subscriber setup
│   ├── octree/            # Octree<T>: subdivision, mass propagation, force approximation
│   ├── particle/          # Particle struct + Moving trait (Verlet integration)
│   ├── renderer/          # three-d window, camera, GPU-instanced rendering
│   ├── vector/            # Vector3 + VectorOps/Reflector/Kinetic/Distributing traits
│   └── main.rs            # Simulation setup and step loop
├── docs/preview/          # README preview screenshots
├── Cargo.toml
└── Cargo.lock
```

## Development and Testing

The project is a single Cargo crate (no workspace):

* **Build:** `cargo build` / `cargo build --release`
* **Run:** `cargo run --release`
* **Check / lint:** `cargo check`, `cargo clippy`
* **Format:** `cargo fmt`

## Support

The simulation emits structured logs via `tracing` (per-frame spans, FPS/particle-count reported once per second). Set `RUST_LOG=debug` or `RUST_LOG=trace` for more verbose output while debugging the attractor/octree logic.
