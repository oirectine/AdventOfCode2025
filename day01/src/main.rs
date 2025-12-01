use std::fs;

fn main() {
    let input_string = match fs::read_to_string("input.txt") {
        Ok(x) => x,
        Err(_) => String::from(""),
    };
    let mut points_to: i16 = 50; //What the dial is pointing to
    let mut number_of_zeroes = 0;
    let lines = input_string.lines(); //Instruction lines
    for line in lines {
        // Parse each line
        let move_spaces: i16 = line[1..].trim().parse().unwrap(); //Shouldn't panic, right?
        if line.starts_with("L") {
            points_to = (points_to - move_spaces).rem_euclid(100);
        } else {
            points_to = (points_to + move_spaces).rem_euclid(100);
        }
        if points_to == 0 {
            number_of_zeroes += 1;
        }
    }
    println!("{number_of_zeroes}");
}
