use std::{collections::{HashMap, HashSet}, fs};
fn main() {
    let input_string = match fs::read_to_string("input.txt") {
        Ok(x) => x,
        Err(_) => String::new(),
    };
    let map: Vec<&str> = input_string.lines().collect();
    let counts = count_splits(&map);
    println!("Number of splits (Part 1): {}\nNumber of total timelines (Part 2): {}", counts.0, counts.1);
}

fn count_splits (map: &Vec<&str>) -> (u64, u64) {
    let mut count: u64 = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut cache: HashMap<(usize, usize), u64> = HashMap::new();
    let start_col = map[0].find('S').unwrap();
    fire_beam(&map, 0, start_col, &mut visited, &mut count, false, &mut cache);
    let quantum_result = fire_beam(&map, 0, start_col, &mut visited, &mut count, true, &mut cache) + 1;
    fn fire_beam(map: &Vec<&str>, row: usize, col: usize, visited: &mut HashSet<(usize, usize)>, count: &mut u64, quantum: bool, cache: &mut HashMap<(usize, usize), u64>) -> u64 {
        let mut timelines: u64 = 0;
        if !quantum && visited.contains(&(row, col)) {
            return 0;
        }
        if quantum {
            if let Some(&value) = cache.get(&(row, col)) {
                return value;
            }
        }
        match map.get(row) {
            None => return 0, //End
            _ => match map.get(row).unwrap().chars().nth(col).unwrap() {
                '^' => {
                    timelines += fire_beam(map, row, col - 1, visited, count, quantum, cache);
                    visited.insert((row, col - 1));
                    timelines += fire_beam(map, row, col + 1, visited, count, quantum, cache);
                    visited.insert((row, col + 1));
                    if !quantum {
                        *count += 1;
                    }
                    return timelines + 1; //Split the timeline
                },
                _ =>  {
                    timelines += fire_beam(map, row + 1, col, visited, count, quantum, cache);
                    visited.insert((row + 1, col));
                },
            }
        }
        if quantum {
            cache.insert((row, col), timelines);
        }
        timelines
    }
    (count, quantum_result)
}