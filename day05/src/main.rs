use std::fs;

fn main() {
    let input_string = match fs::read_to_string("input.txt") {
        Ok(x) => x,
        Err(_) => String::new(),
    };
    let collection: Vec<&str> = input_string.split("\n\n").collect();
    let mut count: u64 = 0;
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    // Part 1
    for num in collection[1].lines() {
        let number: u64 = num.parse().unwrap();
        let mut found = false;
        for range in collection[0].lines() {
            let start: u64 = range.split('-').collect::<Vec<&str>>()[0].parse().unwrap();
            let end: u64 = range.split('-').collect::<Vec<&str>>()[1].parse().unwrap();
            if (start..=end).contains(&number) && !found {
                count += 1;
                found = true;
            }
        }
    }
    for range in collection[0].lines() {
        let start: u64 = range.split('-').collect::<Vec<&str>>()[0].parse().unwrap();
        let end: u64 = range.split('-').collect::<Vec<&str>>()[1].parse().unwrap();
        ranges.push((start, end));
    }
    ranges.sort_by(|a, b| a.0.cmp(&b.0)); //Sort ranges
    let count_ranges = count_all_ranges(&ranges);
    println!("Fresh items (Part 1): {count}\nTotal fresh items (Part 2): {count_ranges}");
}

fn count_all_ranges(ranges: &Vec<(u64, u64)>) -> u64 {

    let mut count = 0;
    let mut previous= 0;

    //Trim range, and find result
    for range in ranges.iter() {
        if range.1 <= previous { 
            count += 0;
        }
        else if range.0 <= previous {
            count += range.1 - previous;
        }
        else {
            count += range.1 - range.0 + 1;
        }
        previous = range.1.max(previous);
    }
    count
}