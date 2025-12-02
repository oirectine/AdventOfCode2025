use std::{collections::HashSet, fs};

fn main() {
    let mut invalid_total_part_1 = 0;
    let mut invalid_total_part_2 = 0;
    let input_string = match fs::read_to_string("input.txt") {
        Ok(x) => x,
        Err(_) => String::from(""),
    };
    let list_of_ranges: Vec<&str> = input_string.split(',').collect();
    for i in list_of_ranges {
        add_invalid_ids(i, &mut invalid_total_part_1, &mut invalid_total_part_2);
    }
    println!("Total of invalid IDs (Part 1): {invalid_total_part_1} (Part 2): {invalid_total_part_2}");
}

//Find invalid IDs in range string, and add them to the total
fn add_invalid_ids(range_str: &str, total: &mut i64, total_2: &mut i64 ) {
    let mut unique_values: HashSet<i64> = HashSet::new();
    let range_collection: Vec<i64> = range_str.split('-')
        .map(|i|i.parse().unwrap())
        .collect();
    for i in range_collection[0]..=range_collection[1] {
        if count_digits(i) % 2 == 0 {
            let left = i / 10i64.pow(count_digits(i)/2);
            let right = i % 10i64.pow(count_digits(i)/2);
            if left == right {
                *total += i;
            }
        }
        //Split number, then check if there's a pattern
        for num in 1..=5 {
            if count_digits(i) % num == 0 {
                let number_vector = split_number(i, num as usize);
                let  mut number_set: HashSet<i64> = HashSet::new();
                for number in &number_vector {
                    number_set.insert(*number);
                }
                if number_vector.len() > 1 && number_set.len() == 1 { //Are all the numbers the same?
                    unique_values.insert(i); //Add the number to the set
                }
            }
        }
    }
    *total_2 += unique_values.iter().sum::<i64>();
}

fn split_number(n: i64, split_into: usize) -> Vec<i64> {
    let mut rtn_vec: Vec<i64> = Vec::new();
    for i in (0..count_digits(n)).step_by(split_into) {
        rtn_vec.push(n % 10i64.pow(count_digits(n) - i) / 10i64.pow(count_digits(n) - i - split_into as u32));
    }
    rtn_vec
}

fn count_digits(n: i64) -> u32 {
    if n == 0 {return 0};
    1 + count_digits (n / 10)
}