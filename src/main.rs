use std::fs::File;
use std::io::Write;

fn main() {
    // Large terrain size with random seed for variety
    let seed: u32 = rand::random();
    let result = rust_land::generate_world(256, 0.1, seed);
    let mut file = File::create("heightmap.glb").expect("create glb");
    file.write_all(&result.glb()).expect("write glb");
    std::fs::write("points.json", result.points()).expect("write json");
    println!("Generated heightmap.glb with seed {}", seed);
}
