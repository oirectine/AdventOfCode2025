use std::{fs, collections::HashMap};

fn main() {
    let input_string = match fs::read_to_string("input.txt") {
        Ok(x) => x,
        Err(_) => String::new(),
    };

    let mut map = HashMap::new();
    let mut rolls: Vec<(i32, i32)> = Vec::new();
    let mut indices: Vec<i32> = Vec::new(); //Indices to sweep
    let mut sweep: Vec<(i32, i32)> = Vec::new(); //Sweep from HashMap
    let mut count;
    let mut counts: Vec<i32> = Vec::new();

    //Populate initial HashMap
    for (y, line) in input_string.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), char);
            if char == '@' {
                rolls.push((x as i32, y as i32));
            }
        }
    }

    //Check hashmap, then sweep until none can be swept
    loop {
        sweep.clear();
        indices.clear();
        count = 0;
        for (i, roll) in rolls.iter().enumerate() {
            if check_directions(4, *roll, &map) {
                count += 1;
                indices.push(i as i32);
                sweep.push(*roll);
            }
        }
        if count == 0 {
            break;
        }
        counts.push(count);
        for i in indices.iter().rev() {
            rolls.remove(*i as usize);
        }
        for i in sweep.iter() { //Sweep the map!
            map.insert(*i, '.');
        }
    }

    let sum: i32 = counts.iter().sum();
    println!("Part 1: {}, Part 2: {}", counts[0], sum);
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