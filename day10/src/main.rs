use std::{fs, ops::BitXor};
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
struct Binary(Vec<bool>);

impl BitXor for Binary {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let Self(lhs) = self;
        Self(
            lhs.iter()
                .zip(rhs.0.iter())
                .map(|(x, y)| *x ^ *y)
                .collect()
        )
    }
}

#[derive(Debug)]
struct Machine {
    lights: Binary,
    buttons: Vec<Binary>,
}

impl Machine {
    fn test_button_combos(&self) -> usize {
        let mut smallest_number: usize = 0;
        for len in 1..=self.buttons.len() {
            for combo in self.buttons.iter().combinations(len) {
                let test = combo.iter().fold(Binary(vec![false; self.lights.0.len()]), |x, &y| x.clone() ^ y.clone());
                if test == self.lights {
                    smallest_number = len;
                    break;
                }
            }
            if smallest_number > 0 {break}
        }
        smallest_number
    }
}

fn main() {
    let input_string = match fs::read_to_string("input.txt") {
        Ok(x) => x,
        Err(_) => String::new(),
    };
    let mut machines: Vec<Machine> = Vec::new();
    
    let cleaned = input_string
        .replace(['(', '[', '{', ')', ']', '}'],"");

    let lines = cleaned.lines();

    //Populate machines vector with data
    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();
        //Convert target to binary
        let mut binary_switch: Binary = Binary(Vec::new());
        for char in split[0].chars() {
            match char {
                '.' => binary_switch.0.push(false),
                _ => binary_switch.0.push(true),
            }
        }
        //Convert buttons to binary format
        let mut new_buttons: Vec<Binary> = Vec::new();
        for button in &split[1..split.len() - 1] { //Don't include vals in curly braces
            let mut new_button :Binary = Binary(Vec::new());
            new_button.0.resize(binary_switch.0.len(), false);
            //new_button.resize(100, false);
            button.split(',')
                .map(|x|x.parse::<i32>().unwrap())
                .for_each(|x|new_button.0[x as usize] = true);
            new_buttons.push(new_button);
        }
        let new_machine = Machine {
            lights: binary_switch,
            buttons: new_buttons,
        };
        machines.push(new_machine);
    }

    let mut total = 0;
    for m in &machines {
        total += m.test_button_combos();
    }
    println!("Smallest number of button presses (Part 1): {total}");
}
