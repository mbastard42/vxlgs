# VXLGS

A voxel-based **Rust** game engine experimenting with **realistic physics** and **ray-marching rendering** to explore modern GPU techniques and **modular** engine design.

## Goals

- Realistic voxel **physics simulation**  
- **Ray-marching** based renderer
- **Procedural animation** & world generation  
- Modular **Rust workspace** for clean architecture and testing  

## Demo

Comming soon

## Installation

```bash
git clone https://github.com/username/vxlgs.git
cd vxlgs
cargo run
```

## Dependencies

- [log](https://crates.io/crates/log) + [env_logger](https://crates.io/crates/env_logger) → logging
- [wgpu](https://crates.io/crates/wgpu) → modern GPU abstraction
- [winit](https://crates.io/crates/winit) → windowing & input handling

<details>

```bash
vxlgs % cargo tree --depth 1 --prune renderer --prune voxel --prune shaders --prune engine --prune math 
app v0.1.0 (vxlgs/crates/app)
├── anyhow v1.0.99
├── env_logger v0.10.2
├── log v0.4.27
├── pollster v0.4.0
└── winit v0.30.12
    [build-dependencies]

engine v0.1.0 (vxlgs/crates/engine)
├── anyhow v1.0.99
└── log v0.4.27

math v0.1.0 (vxlgs/crates/math)
├── anyhow v1.0.99
└── log v0.4.27

renderer v0.1.0 (vxlgs/crates/renderer)
├── anyhow v1.0.99
├── log v0.4.27
├── pollster v0.4.0
└── wgpu v26.0.1
    [build-dependencies]

shaders v0.1.0 (vxlgs/crates/shaders)
├── anyhow v1.0.99
└── log v0.4.27

voxel v0.1.0 (vxlgs/crates/voxel)
├── anyhow v1.0.99
└── log v0.4.27
```
</details>

## Project architecture and documentation

```bash
vxlgs % tree -d -L 2
.
├── assets
│   ├── images
│   └── textures
└── crates
    ├── app
    ├── engine
    ├── math
    ├── renderer
    ├── shaders
    └── voxel
```

Each crate is self-contained and documented.
```bash
cargo doc --open
```