use cpu_timer;
/**
 * Takes an JSON file input of LAT/LON pairs, and calculates the haversine distance between them
 * Also calculates the average distance of each pair
 */
use std::env;
use std::fs;

fn main() {
    //allow for retireval of os_freq and cpu_freq
    let timer = cpu_timer::Timer::new();

    let startup_start = timer.cpu_timer;
    //(cpu_elapsed) * os_freq / os_elapsed = cpu time
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} input_file.json", args[0]);
        return;
    }
    let startup_elapsed = cpu_timer::read_cpu_timer() - startup_start;

    let load_file_start = cpu_timer::read_cpu_timer();
    let raw_json = fs::read_to_string(&args[1]).expect("Failed to read JSON file");
    let load_file_elapsed = cpu_timer::read_cpu_timer() - load_file_start;

    let parse_start = cpu_timer::read_cpu_timer();
    let file_len = raw_json.len();
    let point_pairs = Haversine::parse(raw_json);
    let parse_elapsed = cpu_timer::read_cpu_timer() - parse_start;

    let calc_dist_start = cpu_timer::read_cpu_timer();
    let mut total_distance = 0.0f64;
    let total_points = point_pairs.len() as f64;
    for pair in point_pairs {
        total_distance += Haversine::distance(&pair.point1, &pair.point2);
    }
    let avg_distance = total_distance / total_points;
    let calc_dist_elapsed = cpu_timer::read_cpu_timer() - calc_dist_start;

    let misc_out_start = cpu_timer::read_cpu_timer();
    println!("Input Size: {}", file_len);
    println!("Pair count: {}", total_points);
    println!("Haversine Sum: {}", avg_distance);
    let misc_out_elapsed = cpu_timer::read_cpu_timer() - misc_out_start;

    let total_os_time =
        (cpu_timer::read_os_timer() - timer.os_timer) as f64 / timer.os_freq as f64 * 1000f64;
    let total_cpu_time = (cpu_timer::read_cpu_timer() - timer.cpu_timer) as f64;

    //=============== PRINT STATS ======================================================================================
    println!(
        "Total time: {:0.4}ms (CPU Freq {})",
        total_os_time, cpu_timer::cpu_freq()
    );
    println!(
        "  Startup: {} ({:0.2}%)",
        startup_elapsed,
        startup_elapsed as f64 / total_cpu_time * 100f64
    );
    println!(
        "  Read: {} ({:0.2}%)",
        load_file_elapsed,
        load_file_elapsed as f64 / total_cpu_time * 100f64
    );
    println!(
        "  Parse: {} ({:0.2}%)",
        parse_elapsed,
        parse_elapsed as f64 / total_cpu_time * 100f64
    );
    println!(
        "  Sum: {}, ({:0.2}%)",
        calc_dist_elapsed,
        calc_dist_elapsed as f64 / total_cpu_time * 100f64
    );
    println!(
        "  MiscOutput: {}, ({:0.2}%)",
        misc_out_elapsed,
        misc_out_elapsed as f64 / total_cpu_time * 100f64
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
