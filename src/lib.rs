use wasm_bindgen::prelude::*;
use noise::{NoiseFn, Perlin};
use serde_json::json;

/// Generate a simple heightmap and return a GLB binary containing a single mesh.
///
/// `size` controls the number of vertices per side of the square grid.
/// `scale` controls the noise frequency.
#[wasm_bindgen]
pub fn generate_heightmap_glb(size: u32, scale: f32) -> Vec<u8> {
    // Generate vertex positions using Perlin noise
    let perlin = Perlin::new(0);
    let mut positions: Vec<f32> = Vec::with_capacity((size * size * 3) as usize);
    for z in 0..size {
        for x in 0..size {
            let xf = x as f64 * scale as f64;
            let zf = z as f64 * scale as f64;
            let y = perlin.get([xf, zf]) as f32;
            positions.push(x as f32);
            positions.push(y);
            positions.push(z as f32);
        }
    }

    // Indices for a triangle grid
    let mut indices: Vec<u32> = Vec::with_capacity(((size - 1) * (size - 1) * 6) as usize);
    for z in 0..(size - 1) {
        for x in 0..(size - 1) {
            let i = z * size + x;
            indices.extend_from_slice(&[i, i + 1, i + size, i + 1, i + size + 1, i + size]);
        }
    }

    let pos_bytes = positions.len() * std::mem::size_of::<f32>();
    let idx_bytes = indices.len() * std::mem::size_of::<u32>();
    let buffer_length = pos_bytes + idx_bytes;

    // Build glTF JSON
    let gltf = json!({
        "asset": {"version": "2.0"},
        "buffers": [{"byteLength": buffer_length}],
        "bufferViews": [
            {"buffer":0,"byteOffset":0,"byteLength":pos_bytes,"target":34962},
            {"buffer":0,"byteOffset":pos_bytes,"byteLength":idx_bytes,"target":34963}
        ],
        "accessors": [
            {"bufferView":0,"componentType":5126,"count":positions.len()/3,"type":"VEC3"},
            {"bufferView":1,"componentType":5125,"count":indices.len(),"type":"SCALAR"}
        ],
        "meshes": [{"primitives":[{"attributes":{"POSITION":0},"indices":1,"mode":4}]}],
        "nodes": [{"mesh":0}],
        "scenes": [{"nodes":[0]}],
        "scene": 0
    });

    let json_str = gltf.to_string();
    let json_pad = (4 - (json_str.len() % 4)) % 4;
    let bin_pad = (4 - (buffer_length % 4)) % 4;

    let total_length = 12 + 8 + json_str.len() + json_pad + 8 + buffer_length + bin_pad;
    let mut glb: Vec<u8> = Vec::with_capacity(total_length);

    // GLB header
    glb.extend_from_slice(&0x46546C67u32.to_le_bytes()); // magic 'glTF'
    glb.extend_from_slice(&2u32.to_le_bytes()); // version
    glb.extend_from_slice(&(total_length as u32).to_le_bytes());

    // JSON chunk
    glb.extend_from_slice(&((json_str.len() + json_pad) as u32).to_le_bytes());
    glb.extend_from_slice(&0x4E4F534Au32.to_le_bytes()); // 'JSON'
    glb.extend_from_slice(json_str.as_bytes());
    glb.extend(std::iter::repeat(b' ').take(json_pad));

    // Binary chunk
    glb.extend_from_slice(&((buffer_length + bin_pad) as u32).to_le_bytes());
    glb.extend_from_slice(&0x004E4942u32.to_le_bytes()); // 'BIN\0'
    for f in &positions {
        glb.extend_from_slice(&f.to_le_bytes());
    }
    for i in &indices {
        glb.extend_from_slice(&i.to_le_bytes());
    }
    glb.extend(std::iter::repeat(0).take(bin_pad));

    glb
}
