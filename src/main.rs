#![allow(redundant_semicolons)]
/**
 * Takes an JSON file input of LAT/LON pairs, and calculates the haversine distance between them
 * Also calculates the average distance of each pair
 */
use std::env;
use std::fs;
use haversine_processor::Haversine;
use cpu_timer::{profile, profile_scope};


#[profile]
fn main() {
    //(cpu_elapsed) * os_freq / os_elapsed = cpu time
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} input_file.json", args[0]);
        return;
    }
    profile_scope!("read file",
    let raw_json = fs::read_to_string(&args[1]).expect("Failed to read JSON file");
    );

    profile_scope!("parse pairs",
    let file_len = raw_json.len();
    let point_pairs = Haversine::parse(raw_json);
    );
    profile_scope!("sum pairs",
    let total_points = point_pairs.len() as f64;
    let total_distance: f64 = point_pairs.iter().map(|x| Haversine::distance(&x.point1, &x.point2)).sum();
    let avg_distance = total_distance / total_points;
    );

    println!("Input Size: {}", file_len);
    println!("Pair count: {}", total_points);
    println!("Haversine Sum: {}", avg_distance);
}
