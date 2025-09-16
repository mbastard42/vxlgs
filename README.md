# VXLGS

A voxel-based **Rust** game engine experimenting with **realistic physics** and **ray-marching rendering** to explore modern GPU techniques and **modular** engine design.

## Applications

The project provides four main programs:

```bash
apps
├── client      # Game client for interacting with the voxel world
├── launcher    # Entry point for managing builds, updates, and startup
├── server      # Dedicated server handling simulation and multiplayer state
└── studio      # Editor for voxel assets, world building, and testing
```

## Libraries

These applications rely on multiple shared libraries, organized by domain:

```bash
libs
├── engine
│   ├── behaviors   # AI, entity logic, and procedural behaviors
│   ├── math        # Math primitives and utilities
│   └── physics     # Physics engine
│
├── graphics
│   ├── frontend    # Windowing, input handling, and high-level graphics API
│   ├── renderer    # Core renderer with ray-marching and GPU pipelines
│   └── shaders     # WGSL/GLSL shader modules for the renderer
│
├── infra
│   ├── core        # Core utilities, common traits, and runtime helpers
│   ├── logging     # Structured logging and diagnostics
│   ├── net         # Networking layer (client-server communication)
│   ├── persistence # Save/load systems, serialization, and data storage
│   └── protocol    # Communication protocols and message formats
│
├── prelude
│   └── src         # Commonly re-exported types and traits for ergonomic imports
│
└── voxel
    ├── growth      # Procedural systems for vegetation and voxel growth
    ├── structure   # World structures
    └── terrain     # Terrain generation and manipulation algorithms
```

## Documentation

The documentation begins [[here]](doc/0.0.index.md) and covers the architectural design, optimization processes, and development guidelines.

Although the code is self-documented, you can also generate the developer documentation locally with:

```bash
cargo doc --open --no-deps --document-private-items 
```