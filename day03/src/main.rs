use std::fs;
fn main() {
    let input_string = match fs::read_to_string("input.txt") {
        Ok(x) => x,
        Err(_) => String::from(""),
    };

    let mut total_joltage_part_1 = 0;
    let mut total_joltage_part_2 = 0;
    for line in input_string.lines() {
        total_joltage_part_1 += find_highest_joltage(line, 2).unwrap();
        total_joltage_part_2 += find_highest_joltage(line, 12).unwrap();
    }
    println!("Highest total joltage (part 1):{total_joltage_part_1} (Part 2): {total_joltage_part_2}");
}

fn find_highest_joltage(string: &str, digits: u32) -> Option<u64> {
    if digits == 0 {return None}; //Base case
    for i in (1..=9).rev() {
        if string[0..=string.len() - digits as usize].contains(&i.to_string()) {
            return Some(10u64.pow(digits - 1) * i +
                match find_highest_joltage(&string[string.find(&i.to_string()).unwrap()+1..],digits-1) {
                    Some(n) => n,
                    None => 0,
                }
            )
        }
    }
    None
}