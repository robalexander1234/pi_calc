# picalc

A Rust learning project that computes digits of π using a spigot algorithm, maps them onto a 2D landscape, applies IIR low-pass filtering, and renders the result as an interactive 3D surface plot using Plotly.

![PI Sphere](pi_sphere.gif)

## Overview

The digits of π, when laid out on a 2D grid and treated as height values, form a landscape of essentially random noise — reflecting the conjectured normality of π. A 2D Butterworth low-pass filter is applied to smooth the surface, which is then mapped as a displacement onto a sphere, producing the planet-like visualization shown above.

## Modules

- **`config.rs`** — Grid dimensions and total digit count
- **`spigot.rs`** — Computes π digits using the Rabinowitz-Wagon spigot algorithm
- **`landscape.rs`** — Constructs the 2D grid, applies IIR filtering, and renders the 3D surface
- **`main.rs`** — Entry point

## How It Works

1. The spigot algorithm generates `NUM_ROWS × NUM_COLS` digits of π
2. Digits are arranged into a 2D grid where each value (0–9) becomes a height value
3. A 2nd-order Butterworth IIR low-pass filter is applied row-wise then column-wise
4. The filtered grid is mapped as a radial displacement onto a sphere using spherical coordinates
5. The result is rendered as an interactive 3D surface using Plotly (Jet colormap)

## Dependencies

```toml
[dependencies]
plotly = "0.12"
```

## Running

```bash
cargo run --release
```

The plot will open automatically in your default browser as a local HTML file.

## Configuration

Edit `config.rs` to change the grid size:

```rust
pub const NUM_ROWS: usize = 200;
pub const NUM_COLS: usize = 200;
```

Larger grids produce more detailed landscapes but take longer to compute.
