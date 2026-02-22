# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Rules

- Never add dependencies without asking first.
- Never abbreviate variable names.

## Build & Run

```bash
cargo build        # build
cargo run          # run
cargo check        # type-check only (faster)
```

## Architecture

Rust 2024 edition workspace with two crates:

- **`engine/`** — reusable engine lib (winit 0.30 + wgpu 28). Handles windowing, GPU, input, and the run loop. Uses winit's `ApplicationHandler` trait pattern.
- **`game/`** — game binary that depends on `engine`. Contains game logic, player, tilemap, etc.

Keep this split: engine knows nothing about the game; game-specific code stays in `game/`.