#[derive(Debug)]
pub struct Haversine;

#[derive(Debug)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

pub struct PointPair {
    pub point1: Point,
    pub point2: Point,
}

//Simplistic parser that takes advantage of the input file format
//This is very slow.
impl Haversine {
    pub fn parse(input: String) -> Vec<PointPair> {
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
    pub fn distance(point1: &Point, point2: &Point) -> f64 {
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