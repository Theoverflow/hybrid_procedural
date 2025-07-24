// Switch to gltf-rs for GLTF construction and binary export
use wasm_bindgen::prelude::*;
use noise::{Perlin, NoiseFn};
use gltf::json::{Root, buffer::{Buffer, View}, accessor::{Accessor, Type as AccType, ComponentType}, mesh::{Mesh, Primitive, Mode}, Node, Scene, validation::Checked::Valid};
// use gltf::export::binary::write;
use gltf::json::serialize::to_writer;
use std::collections::BTreeMap;

#[wasm_bindgen]
pub fn generate_landscape(size: u32, scale: f32) -> Vec<u8> {
    // 1) Generate Perlin-based heightmap
    let perlin = Perlin::new(0); // Use 0 as the seed, or choose any u32 value
    let mut positions = Vec::with_capacity((size * size * 3) as usize);
    for x in 0..size {
        for z in 0..size {
            let xf = x as f64 * scale as f64;
            let zf = z as f64 * scale as f64;
            let y = perlin.get([xf, zf]) as f32 * 10.0;
            positions.extend_from_slice(&[x as f32, y, z as f32]);
        }
    }

    // 2) Build index buffer for triangles
    let mut indices = Vec::with_capacity(((size-1)*(size-1)*6) as usize);
    for x in 0..(size-1) {
        for z in 0..(size-1) {
            let i = x * size + z;
            indices.extend_from_slice(&[
                i, i + 1, i + size,
                i + 1, i + size + 1, i + size
            ]);
        }
    }

    // 3) Pack binary data: positions (f32) then indices (u32)
    let mut buffer_data = Vec::new();
    for f in &positions { buffer_data.extend_from_slice(&f.to_le_bytes()); }
    for idx in &indices { buffer_data.extend_from_slice(&idx.to_le_bytes()); }

    // 4) GLTF JSON structures
    // Buffer container
    let buffer = Buffer {
        byte_length: buffer_data.len() as u32,
        uri: None,
        name: None,
        extensions: None,
        extras: Default::default(),
    };

    // bufferViews
    let bv_pos = View {
        buffer: 0.into(),
        byte_length: (positions.len()*4) as u32,
        byte_offset: Some(0),
        byte_stride: Some(12),
        target: Some(gltf::json::buffer::Target::ArrayBuffer),
        name: None,
        extensions: None,
        extras: Default::default(),
    };
    let bv_idx = View {
        buffer: 0.into(),
        byte_length: (indices.len()*4) as u32,
        byte_offset: Some((positions.len()*4) as u32),
        byte_stride: None,
        target: Some(gltf::json::buffer::Target::ElementArrayBuffer),
        name: None,
        extensions: None,
        extras: Default::default(),
    };

    // Accessors
    let acc_pos = Accessor {
        buffer_view: Some(0.into()),
        byte_offset: 0,
        count: (positions.len()/3) as u32,
        component_type: ComponentType::F32,
        type_: AccType::Vec3,
        normalized: false,
        min: Some(json::Value::from([0.0, 0.0, 0.0])),
        max: Some(json::Value::from([size as f32, 10.0, size as f32])),
        sparse: None,
        name: None,
        extensions: None,
        extras: Default::default(),
    };
    let acc_idx = Accessor {
        buffer_view: Some(1.into()),
        byte_offset: 0,
        count: indices.len() as u32,
        component_type: ComponentType::U32,
        type_: AccType::Scalar,
        normalized: false,
        min: None,
        max: None,
        sparse: None,
        name: None,
        extensions: None,
        extras: Default::default(),
    };

    // Mesh primitive
    let mut attrs = BTreeMap::new();
    attrs.insert("POSITION".into(), 0.into());
    let prim = Primitive {
        attributes: attrs,
        indices: Some(1.into()),
        material: None,
        mode: Mode::Triangles,
        targets: None,
        extensions: None,
        extras: Default::default(),
    };

    let mesh = Mesh {
        primitives: vec![prim],
        weights: None,
        name: Some("LandscapeMesh".into()),
        extensions: None,
        extras: Default::default(),
    };

    let node = Node {
        mesh: Some(0.into()),
        ..Default::default()
    };

    let scene = Scene {
        nodes: vec![0.into()],
        ..Default::default()
    };

    let root = Root {
        buffers: vec![buffer],
        buffer_views: vec![bv_pos, bv_idx],
        accessors: vec![acc_pos, acc_idx],
        meshes: vec![mesh],
        nodes: vec![node],
        scenes: vec![scene],
        scene: Some(0.into()),
        ..Default::default()
    };
    // 5) Serialize to GLB bytes
    to_binary(&root, &[buffer_data]).unwrap()
}