use std::fs;

fn main() {
    let input_string = match fs::read_to_string("input.txt") {
        Ok(x) => x,
        Err(_) => String::new(),
    };

    //Populate vectors
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut ceph_numbers: Vec<Vec<i64>> = Vec::new();
    let lines: Vec<&str> = input_string.lines().collect();
    let operators: Vec<&str> = lines[lines.len()-1].split_whitespace().collect();
    //Cephalopod to human
    let mut ceph: Vec<String> = Vec::new();
    for _ in 0..=lines[0].len() {
        ceph.push(String::new());
    }

    for line in 0..lines.len()-1 { //Don't include operator line
        //Translate cephalopod to human
        for (i, c) in String::from(lines[line]).chars().rev().enumerate() {
            ceph[i].push(c);
        }
        let mut parsed: Vec<i64> = Vec::new();
        for n in lines[line].split_whitespace() {
            parsed.push(n.parse().unwrap());
        }
        numbers.push(parsed);
    }

    //Convert ceph to ceph_numbers
    let mut column: Vec<i64> = Vec::new();
    for s in ceph {
        if s.trim().is_empty() {
            ceph_numbers.push(column.clone());
            column.clear();
        }
        else {
            column.push(s.trim().parse::<i64>().unwrap());
        }
    }

    //Perform operations
    let mut total: i64 = 0;
    let mut ceph_total: i64 = 0;
    let mut sums: Vec<i64> = Vec::new();
    for _ in 0..operators.len() {
        sums.push(0);
    }
    let mut ceph_sums: Vec<i64> = Vec::new();
    for _ in 0..operators.len() {
        ceph_sums.push(0);
    }

    //Part 1
    for line in numbers {
        for (index, number) in line.iter().enumerate() {
            match operators[index] {
                "+" => sums[index] += number,
                "*" => match sums[index] {
                    0 => sums[index] = *number,
                    _ => sums[index] *= number,
                },
                _ => sums[index] += 0,
            }
        }
    }
    //Part 2
    for (index, line) in ceph_numbers.iter().enumerate() {
        for number in line {
            match operators[operators.len() - index - 1] {
                "+" => ceph_sums[index] += number,
                "*" => match ceph_sums[index] {
                    0 => ceph_sums[index] = *number,
                    _ => ceph_sums[index] *= number,
                },
                _ => ceph_sums[index] += 0,
            }
        }
    }
    total += sums.iter().sum::<i64>();
    ceph_total += ceph_sums.iter().sum::<i64>();
    println!("Human total (Part 1): {total}\nCephalopod total (part 2): {ceph_total}");
}
