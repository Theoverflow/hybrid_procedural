# hybrid_procedural

This crate exposes a small function for generating a heightmap mesh and
returning the result as a GLB binary.  It targets WebAssembly so it can be used
from a TypeScript project. The exported function accepts a random seed so each
heightmap can be unique.

## Building

Install `wasm-pack` and build the package:

```bash
wasm-pack build --release --target web
```

The generated package in `pkg/` can be imported from TypeScript.

To produce a large heightmap locally for inspection, run the binary:

```bash
cargo run --release
```

## Usage from TypeScript

```ts
import init, { generate_heightmap_glb } from './pkg/rust_land.js';

async function load() {
  await init();
  const seed = Math.floor(Math.random() * 0xffffffff);
  const bytes = generate_heightmap_glb(64, 0.1, seed);
  // `bytes` is an Uint8Array containing the GLB file
}
```
