# hybrid_procedural

This crate exposes a small function for generating a heightmap mesh and
returning the result as a GLB binary.  It targets WebAssembly so it can be used
from a TypeScript project.

## Building

Install `wasm-pack` and build the package:

```bash
wasm-pack build --release --target web
```

The generated package in `pkg/` can be imported from TypeScript.

## Usage from TypeScript

```ts
import init, { generate_heightmap_glb } from './pkg/rust_land.js';

async function load() {
  await init();
  const bytes = generate_heightmap_glb(64, 0.1);
  // `bytes` is an Uint8Array containing the GLB file
}
```
