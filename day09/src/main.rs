use std::fs;
struct Box {
    point_1: (i64, i64),
    point_2: (i64, i64)
}

impl Box {
    fn new(point_1: (i64, i64), point_2: (i64, i64)) -> Self {
        Self {point_1, point_2}
    }
    fn area(&self) -> i64 {
        ((self.point_1.0 - self.point_2.0).abs() + 1) * ((self.point_1.1 - self.point_2.1).abs() + 1)
    }
}

fn main() {
    let input_string = match fs::read_to_string("input.txt") {
        Ok(x) => x,
        Err(_) => String::new(),
    };
    let tiles: Vec<(i64, i64)> = input_string
        .lines()
        .map(|l| {
            let mut nums = l.split(',')
            .map(|x| x.parse().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();
    
    // Create collection of all boxes
    let mut boxes: Vec<Box> = Vec::new();
    for (index, tile) in tiles.iter().enumerate() {
        for other_tile in &tiles[index + 1..] {
            boxes.push(Box::new(*(tile),*(other_tile)));
        }
    }
    
    let mut highest_total = 0;
    for b in &boxes {
        if b.area() > highest_total {
            highest_total = b.area();
        }
    }

    println!("Area of box with highest area (Part 1): {highest_total}");
}
