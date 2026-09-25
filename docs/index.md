# SpaceCorps Documentation Portal

> Authoritative engineering manuals, CLI tool specifications, and simulation physics documentation.

## Architecture & Ecosystem Overview

SpaceCorps builds native Rust 2024 systems engineered for autonomous AI agents, scientific molecular simulation, and cross-platform 3D graphics.

### 1. Autonomous Agent CLI Tools (23 Tools)

- Sub-3ms process startup latency with zero VM/interpreter overhead.
- Dual output modes: human YAML by default, strict JSON with `--json`.
- Complete catalog: [SpaceCorps Tools Directory](https://spacecorps.github.io/tools/)
- Machine-readable instructions: [llms.txt](https://spacecorps.github.io/llms.txt)

### 2. Space3d Cross-Platform Game & Simulation Engine

- Pure-Rust 3D engine built on `wgpu 30` with GPU-driven indirect drawing (multi-draw indirect).
- Renders 1,000,000+ instances in under 5 draw calls with GPU frustum culling.
- Deterministic 60Hz ECS simulation decoupled from render frame rate.
- Cross-platform target support: Desktop (macOS, Linux, Windows), WebAssembly (WebGL2/WebGPU), mobile (iOS, Android).

### 3. Space3d-Molecular Scientific Simulation

- **spacemd Engine:** Atomistic simulation of crystals (FCC, BCC), liquids, and ionic solids. Potentials: Lennard-Jones, Morse, Buckingham, Ewald Coulomb, and EAM. Throughput: 74M–108M atom-steps/s on Apple Silicon.
- **spaceemit Engine:** Thermal-field electron emission physics solved in 14 µs per point.
- **OpenAPI 3.1 Spec:** Available at [https://spacecorps.github.io/openapi.json](https://spacecorps.github.io/openapi.json).
- **Cargo Sparse Registry:** `sparse+https://spacecorps-registry.sliplane.app/index/`
