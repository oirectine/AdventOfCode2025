use std::{collections::HashSet, fs};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
 struct Point {
     x: i64,
     y: i64,
     z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self {x,y,z}
    }
    fn distance_to (&self, other: &Point) -> f64 {
        ((self.x as f64 - other.x as f64).powi(2) + (self.y as f64 - other.y as f64).powi(2) + (self.z as f64 - other.z as f64).powi(2)).sqrt()
    }
}

//#[derive(Debug)]
struct Distance<'a> {
    distance: f64,
    points: (&'a Point, &'a Point),
}

fn main() {
    let input_string = match fs::read_to_string("input.txt") {
        Ok(x) => x,
        Err(_) => String::new(),
    };
    let mut points: Vec<Point> = Vec::new();
    for line in input_string.lines() {
        let num = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        points.push(Point::new(num[0],num[1],num[2]));
    }
    //Calculate all distances, and place them into a Vector of distances
    let mut all_distances: Vec<Distance> = Vec::new();
    for (index, point) in points.iter().enumerate() {
        for other_point in &points[index + 1..points.len()] {
            all_distances.push(Distance {
                distance: point.distance_to(other_point),
                points: (&point, &other_point),
            });
        }
    }
    all_distances.sort_by(|a, b|a.distance.total_cmp(&b.distance));

    let part_1 = count_networks(&points, &all_distances, 1000,3).0;
    let part_2 = calculate_final_network(&points, &all_distances);
    println!("After 1000 connections (Part 1): {part_1}\n Last two connections (Part 2): {part_2}");
}

fn count_networks(points: &Vec<Point>, distances: &Vec<Distance>, connections: usize, mul_largest: usize) -> (usize, usize, i64) {
    let mut point_set: HashSet<Point> = points.iter().cloned().collect();
    let mut networks: Vec<HashSet<&Point>> = Vec::new();
    let mut new_network: HashSet<&Point> = HashSet::new();
    let mut total: usize = 1;
    let mut last_two: (i64, i64) = (0,0);
    for distance in distances[0..connections].iter() {
        let mut first = None;
        let mut second = None;
  
        for (index, network) in networks.iter().enumerate() {
            if network.contains(distance.points.0) {
                first = Some(index);
            }
            if network.contains(distance.points.1) {
                second = Some(index);
            }
        }

        match (first, second) {
            (Some(m), Some(n)) => {
                if m != n { //Merge networks
                    let temp_network = networks[n].clone();
                    networks[m].extend(temp_network.iter());
                    networks.remove(n);
                }
            },
            (Some(n), None) => {
                networks[n].insert(distance.points.1);
            },
            (None, Some(n)) => {
                networks[n].insert(distance.points.0);
            },
            (None, None) =>  { //Create new network
                new_network.insert(distance.points.0);
                new_network.insert(distance.points.1);
                networks.push(new_network.clone());
                new_network.clear()
            },
        }
        point_set.remove(distance.points.0);
        point_set.remove(distance.points.1);
        last_two = (distance.points.0.x, distance.points.1.x);
    }
    networks.sort_by(|a, b|b.len().cmp(&(a.len())));
    for set in &networks[0..mul_largest] {
        total *= set.len();
    };
    if networks.len() + point_set.len() == 1 {
        println!("{:?} {:?}", last_two.0, last_two.1);
    }
    (total, networks.len() + point_set.len(), last_two.0 * last_two.1)
}

fn calculate_final_network (points: &Vec<Point>, distances: &Vec<Distance>) -> i64 {
    for i in 5000.. { //Increasing this number makes it faster. Lazy, I know.
        let result = count_networks(points, distances, i, 0);
        if result.1 == 1 {
            return result.2;
        }
    };
    0
}