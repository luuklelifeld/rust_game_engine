# Rust 3D game engine / rendering library

This is a hobby project to get more experience with 3d rendering maths, and to start my Rust learning journey.

This project is very similar to my other project, [SDL2Game](https://github.com/luuklelifeld/sdl2game). This is basically a rebuilding of that project in Rust. It even uses very similar dependencies, `SDL2 -> winit`, `wgpu -> bgfx`.

## Dependencies

The main dependencies are `winit` and `wgpu`. `winit` allows window creation, management, keyboard input, maybe audio like SDL2? `wgpu` is a layer above graphics APIs that allow multiple graphics APIs to be used in a single implemention.

## Resources

I'm using [this tutorial](https://sotrh.github.io/learn-wgpu/#what-is-wgpu) to learn about specific `wgpu` APIs. It's very well written, and assumes a tiny bit of 3D rendering knowledge, which matches with my current experience.

## Rules

I have set a few rules to maximize personal growth in this project.

1. Writing code with AI is forbidden. It's too easy to skip over learning steps by relying on AI. It's fine to use AI for ideation.
2. The goal is NOT to create a fully fledged game.

## Installation

`cargo build` or `cargo run`. Rust's dependency and build system is amazing compared to C++.

## Architecture

TODO
