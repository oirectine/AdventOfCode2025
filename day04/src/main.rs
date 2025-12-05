use std::{fs, collections::HashMap};

fn main() {
    let input_string = match fs::read_to_string("input.txt") {
        Ok(x) => x,
        Err(_) => String::new(),
    };

    let mut map = HashMap::new();
    let mut boxes: Vec<(i32, i32)> = Vec::new();
    let mut count = 0;

    for (y, line) in input_string.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), char);
            if char == '@' {
                boxes.push((x as i32, y as i32));
            }
        }
    }
    for i in boxes {
        if check_directions(4, i, &map) {
            count += 1;
        }
    }
    println!("{count}");
}

fn check_directions(num: u8, coords: (i32, i32), map: &HashMap<(i32, i32), char>) -> bool {
    let mut neighbour_count = 0;
    let directions: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1)
    ];
    for (x, y) in directions.iter() {
        if map.get(&(coords.0 + *x, coords.1 + *y)).unwrap_or(&'.') == &'@' {
            neighbour_count += 1;
        }
    }

    if neighbour_count < num {
        return true;
    }
    false
}