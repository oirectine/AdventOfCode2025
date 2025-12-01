use std::fs;

fn main() {
    let input_string = match fs::read_to_string("input.txt") {
        Ok(x) => x,
        Err(_) => String::from(""),
    };
    let mut points_to: i16 = 50; //What the dial is pointing to.
    let mut number_of_zeroes = 0;
    let mut number_of_clicks = 0;
    let lines = input_string.lines(); //Instruction lines.
    for line in lines {
        // Parse each line
        let move_spaces: i16 = line[1..].parse().unwrap(); //Shouldn't panic, right?
        if line.starts_with("L") {
            if points_to == 0 {points_to = 100}; //Need to reset to 100, otherwise it'll click if it starts on zero.
            number_of_clicks += (100 - points_to + move_spaces) / 100;
            points_to = (points_to - move_spaces).rem_euclid(100);
        } else {
            number_of_clicks += (points_to + move_spaces) / 100;
            points_to = (points_to + move_spaces).rem_euclid(100);
        }
        if points_to == 0 {
            number_of_zeroes += 1;
        }
    }
    println!("Turns ending on zero: {number_of_zeroes}\nNumber of clicks: {number_of_clicks}");
}

