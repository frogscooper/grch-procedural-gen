# grch-procedural-gen

A minimal-dependency 3D BSP procedural generation system written in Rust, featuring deterministic generation, orthogonal corridor routing, and OBJ export.

<p align="center">
  <img src="assets/grch_obj_angle_1.png" width="85%">
</p>

## Features

- 3D BSP partitioning
- Deterministic seeded generation
- Orthogonal corridor routing
- Constructive algorithm that guarantees no collisions
- OBJ export
- Regression and integration tests
- The only dependencies are the standard library and rand

## Visualization

<p align="center">
  <img src="assets/grch_obj_top_angle.png" width="48%">
  <img src="assets/grch_obj_cutaway.png" width="48%">
</p>

## How it works

BSP partitioning -> room placement -> room connection -> orthogonal routing -> corridor geometry -> OBJ export
