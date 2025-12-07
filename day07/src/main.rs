use std::{fs, collections::HashMap};
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
    let mut cache: HashMap<(usize, usize), u64> = HashMap::new();
    let start_col = map[0].find('S').unwrap();
    let count = fire_beam(&map, 0, start_col, false, &mut cache);
    cache.clear();
    let quantum_result = fire_beam(&map, 0, start_col, true, &mut cache) + 1;
    fn fire_beam(map: &Vec<&str>, row: usize, col: usize, quantum: bool, cache: &mut HashMap<(usize, usize), u64>) -> u64 {
        let mut timelines: u64 = 0;
        if let Some(&value) = cache.get(&(row, col)) {
            if quantum {
                return value;
            } else {
                return 0;
            }
        }
        match map.get(row) {
            None => return 0, //End
            _ => match map.get(row).unwrap().chars().nth(col).unwrap() {
                '^' => {
                    timelines += fire_beam(map, row, col - 1, quantum, cache);
                    timelines += fire_beam(map, row, col + 1, quantum, cache);
                    return timelines + 1; //Split the timeline
                },
                _ =>  {
                    timelines += fire_beam(map, row + 1, col, quantum, cache);
                },
            }
        }
        cache.insert((row, col), timelines);
        timelines
    }
    (count, quantum_result)
}