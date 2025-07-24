# hybrid_procedural

This crate exposes functions for generating a procedural terrain heightmap as a
GLB mesh.  It targets WebAssembly so it can be used from a TypeScript project.
The generator accepts a random seed so every call can produce a different map
and it also reports a few interest points such as the main mountain peak and a
river path.

## Building

Install `wasm-pack` and build the package:

```bash
wasm-pack build --release --target web
```

The generated package in `pkg/` can be imported from TypeScript. Two exported
functions are available: `generate_heightmap_glb` which just returns the GLB
mesh, and `generate_world` which additionally reports interesting locations
like the river path.

To produce a large heightmap locally for inspection, run the binary:

```bash
cargo run --release
```

## Usage from TypeScript

```ts
import init, { generate_world } from './pkg/rust_land.js';

async function load() {
  await init();
  const seed = Math.floor(Math.random() * 0xffffffff);
  const result = generate_world(64, 0.1, seed);
  const bytes = result.glb;
  const points = result.points;
  // `bytes` is a Uint8Array containing the GLB file
  // `points` is an array with interesting locations
}
```
