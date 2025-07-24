use std::fs::File;
use std::io::Write;

fn main() {
    // Large terrain size with random seed for variety
    let seed: u32 = rand::random();
    let bytes = rust_land::generate_heightmap_glb(256, 0.1, seed);
    let mut file = File::create("heightmap.glb").expect("create glb");
    file.write_all(&bytes).expect("write glb");
    println!("Generated heightmap.glb with seed {}", seed);
}
