use noise::{NoiseFn, Perlin};
use serde::Serialize;
use serde_json::json;
use wasm_bindgen::prelude::*;

/// Point of interest generated alongside the heightmap.
#[derive(Serialize)]
pub struct InterestPoint {
    pub x: u32,
    pub z: u32,
    pub kind: String,
}

#[wasm_bindgen]
pub struct WorldResult {
    #[wasm_bindgen(skip)]
    pub glb: Vec<u8>,
    #[wasm_bindgen(skip)]
    pub points: Vec<InterestPoint>,
}


/// Generate a simple heightmap and return a GLB binary containing a single mesh.
///
/// `size` controls the number of vertices per side of the square grid.
/// `scale` controls the noise frequency.
/// `seed` allows generating different maps for each call.
#[wasm_bindgen]
impl WorldResult {
    /// Access the generated GLB binary.
    #[wasm_bindgen(getter)]
    pub fn glb(&self) -> Vec<u8> {
        self.glb.clone()
    }

    /// Interest points as a JSON string.
    #[wasm_bindgen(getter)]
    pub fn points(&self) -> String {
        serde_json::to_string(&self.points).unwrap()
    }
}

/// Generate a simple heightmap and return a GLB binary containing a single mesh.
///
/// `size` controls the number of vertices per side of the square grid.
/// `scale` controls the noise frequency.
/// `seed` allows generating different maps for each call.
#[wasm_bindgen]
pub fn generate_heightmap_glb(size: u32, scale: f32, seed: u32) -> Vec<u8> {
    let (positions, indices, _) = generate_terrain(size, scale, seed);
    build_glb(&positions, &indices)
}

/// Generate a heightmap with interest points such as a mountain peak, a river
/// flowing to the sea and a forest along the river.
#[wasm_bindgen]
pub fn generate_world(size: u32, scale: f32, seed: u32) -> WorldResult {
    let (positions, indices, points) = generate_terrain(size, scale, seed);
    let glb = build_glb(&positions, &indices);
    WorldResult { glb, points }
}

fn generate_terrain(size: u32, scale: f32, seed: u32) -> (Vec<f32>, Vec<u32>, Vec<InterestPoint>) {
    let perlin = Perlin::new(seed);
    let mut heights = vec![0.0f32; (size * size) as usize];
    let center = size as f32 / 2.0;
    for z in 0..size {
        for x in 0..size {
            let mut h = 0.0;
            let mut f = 1.0f64;
            let mut a = 1.0;
            for _ in 0..4 {
                let nx = x as f64 * scale as f64 * f;
                let nz = z as f64 * scale as f64 * f;
                h += perlin.get([nx, nz]) as f32 * a as f32;
                f *= 0.5;
                a *= 0.5;
            }
            let dx = x as f32 - center;
            let dz = z as f32 - center;
            let dist = (dx * dx + dz * dz).sqrt() / center;
            let mask = (1.0 - dist).max(0.0);
            let height = h * mask;
            heights[(z * size + x) as usize] = height;
        }
    }

    let mut peak_idx = 0usize;
    for (i, &h) in heights.iter().enumerate() {
        if h > heights[peak_idx] {
            peak_idx = i;
        }
    }
    let peak_x = (peak_idx as u32) % size;
    let peak_z = (peak_idx as u32) / size;

    let mut river_path = Vec::new();
    let mut x = peak_x as i32;
    let mut z = peak_z as i32;
    let sea_level = 0.0f32;
    while x > 1 && z > 1 && (x as u32) < size - 1 && (z as u32) < size - 1 {
        let idx = (z as u32 * size + x as u32) as usize;
        river_path.push((x as u32, z as u32));
        heights[idx] = sea_level - 0.02;
        if heights[idx] <= sea_level {
            break;
        }
        let mut next = (x, z);
        let mut next_h = heights[idx];
        for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x + dx;
            let nz = z + dz;
            if nx <= 0 || nz <= 0 || (nx as u32) >= size || (nz as u32) >= size {
                continue;
            }
            let ni = (nz as u32 * size + nx as u32) as usize;
            if heights[ni] < next_h {
                next = (nx, nz);
                next_h = heights[ni];
            }
        }
        if next == (x, z) {
            if x < center as i32 {
                x -= 1
            } else {
                x += 1
            };
            if z < center as i32 {
                z -= 1
            } else {
                z += 1
            };
        } else {
            x = next.0;
            z = next.1;
        }
    }
    let river_mouth = (x as u32, z as u32);

    let mut points = Vec::new();
    points.push(InterestPoint {
        x: peak_x,
        z: peak_z,
        kind: "mountain".into(),
    });
    points.push(InterestPoint {
        x: river_mouth.0,
        z: river_mouth.1,
        kind: "river_mouth".into(),
    });
    if let Some(&(fx, fz)) = river_path.get(river_path.len() / 2) {
        points.push(InterestPoint {
            x: fx,
            z: fz,
            kind: "forest".into(),
        });
    }

    let mut positions = Vec::with_capacity((size * size * 3) as usize);
    for z in 0..size {
        for x in 0..size {
            let h = heights[(z * size + x) as usize];
            positions.extend_from_slice(&[x as f32, h, z as f32]);
        }
    }

    let mut indices = Vec::with_capacity(((size - 1) * (size - 1) * 6) as usize);

    for z in 0..(size - 1) {
        for x in 0..(size - 1) {
            let i = z * size + x;
            indices.extend_from_slice(&[i, i + 1, i + size, i + 1, i + size + 1, i + size]);
        }
    }

    (positions, indices, points)
}

fn build_glb(positions: &[f32], indices: &[u32]) -> Vec<u8> {

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

    glb.extend_from_slice(&0x46546C67u32.to_le_bytes());
    glb.extend_from_slice(&2u32.to_le_bytes());
    glb.extend_from_slice(&(total_length as u32).to_le_bytes());

    glb.extend_from_slice(&((json_str.len() + json_pad) as u32).to_le_bytes());
    glb.extend_from_slice(&0x4E4F534Au32.to_le_bytes());
    glb.extend_from_slice(json_str.as_bytes());
    glb.extend(std::iter::repeat(b' ').take(json_pad));

    glb.extend_from_slice(&((buffer_length + bin_pad) as u32).to_le_bytes());
    glb.extend_from_slice(&0x004E4942u32.to_le_bytes());
    for f in positions {
        glb.extend_from_slice(&f.to_le_bytes());
    }
    for i in indices {

        glb.extend_from_slice(&i.to_le_bytes());
    }
    glb.extend(std::iter::repeat(0).take(bin_pad));

    glb
}
