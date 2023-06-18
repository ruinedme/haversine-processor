/**
 * Takes an JSON file input of LAT/LON pairs, and calculates the haversine distance between them
 * Also calculates the average distance of each pair
 */
use std::env;
use std::fs;
use std::time;

fn main() {
    let start = time::SystemTime::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} input_file.json", args[0]);
        return;
    }
    let loaded_file_time = time::SystemTime::now();
    let raw_json = fs::read_to_string(&args[1]).expect("Failed to read JSON file");
    let loaded_file_time = loaded_file_time.elapsed().unwrap();

    let parsed_pairs_time = time::SystemTime::now();
    let point_pairs = Haversine::parse(raw_json);
    let parsed_pairs_time = parsed_pairs_time.elapsed().unwrap();

    let mut total_distance = 0.0f64;
    let total_points = point_pairs.len() as f64;
    let calc_avg_dist_time = time::SystemTime::now();
    for pair in point_pairs {
        total_distance += Haversine::distance(&pair.point1, &pair.point2);
    }
    let avg_distance = total_distance / total_points;
    let calc_avg_dist_time = calc_avg_dist_time.elapsed().unwrap();

    println!("average distance of all points: {}", avg_distance);
    let end_time = start.elapsed().unwrap();
    println!("times:");
    println!(
        "load file: {} \u{00B5}s -- {} s",
        loaded_file_time.as_micros(),
        loaded_file_time.as_secs()
    );
    println!(
        "parsed pairs: {} \u{00B5}s -- {} s",
        parsed_pairs_time.as_micros(),
        parsed_pairs_time.as_secs()
    );
    println!(
        "calculate avg distance: {} \u{00B5}s -- {} s",
        calc_avg_dist_time.as_micros(),
        calc_avg_dist_time.as_secs()
    );
    println!(
        "total time: {} \u{00B5}s -- {} s",
        end_time.as_micros(),
        end_time.as_secs()
    );
}

#[derive(Debug)]
struct Haversine;

#[derive(Debug)]
struct Point {
    lat: f64,
    lon: f64,
}

struct PointPair {
    point1: Point,
    point2: Point,
}

//Simplistic parser that takes advantage of the input file format for quick parsing
impl Haversine {
    fn parse(input: String) -> Vec<PointPair> {
        let mut points: Vec<PointPair> = Vec::new();
        //split string by beginning of of each object
        let s_split: Vec<&str> = input.split("{\n").collect();
        for s in s_split {
            //split by each item seapartor
            let pair: Vec<&str> = s.trim().split(",\n").collect();
            if pair.len() < 4 {
                continue;
            }
            //split each of the 4 values by a KV separator
            let p1: Vec<&str> = pair[0].split(": ").collect();
            let p2: Vec<&str> = pair[1].split(": ").collect();
            let p3: Vec<&str> = pair[2].split(": ").collect();
            let p4: Vec<&str> = pair[3].split(": ").collect();
            //have to do some extra processing on the last point as it will contain the matching }
            let p4: Vec<&str> = p4[1].split("\n").collect();
            let point_pair = PointPair {
                point1: Point {
                    lat: p1[1].parse().unwrap(),
                    lon: p2[1].parse().unwrap(),
                },
                point2: Point {
                    lat: p3[1].parse().unwrap(),
                    lon: p4[0].parse().unwrap(),
                },
            };
            points.push(point_pair);
        }
        return points;
    }

    /// Calculates the haversine distance between 2 points on a sphere the approx size of earth in km
    fn distance(point1: &Point, point2: &Point) -> f64 {
        // latitude is Y
        // longitude is X
        let earth_radius = 6372.8; // in km
        let lon1 = point1.lon.to_radians();
        let lon2 = point2.lon.to_radians();
        let lat1 = point1.lat.to_radians();
        let lat2 = point2.lat.to_radians();
        let d_lon = lon2 - lon1;
        let d_lat = lat2 - lat1;

        let a =
            (d_lat / 2.0).sin().powf(2.0) + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powf(2.0);
        let c = 2.0 * a.sqrt().asin();

        earth_radius * c
    }
}
