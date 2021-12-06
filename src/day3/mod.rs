use std::fs;

pub fn part1() {
    let path = "src/day3/inputs/input-part-1.txt";
    let contents = fs::read_to_string(path).expect("File reading failed.");
    let binary_list: Vec<&str> = contents.split("\n").collect();
    let binary_list_size = binary_list.len();

    let mut positive_bit_count = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for binary in binary_list {
        let bits: Vec<&str> = binary.split("").collect();

        // The 1 ↓ and the plus 1 ↓ -- are for the blank spaces left from the split("")
        for i in 1..binary.len() + 1 {
            if bits[i] == "1" {
                positive_bit_count[i - 1] += 1;
            }
        }
    }

    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    for count in positive_bit_count {
        if count > (binary_list_size / 2) {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    // println!("Gamma: {}", gamma);
    // println!("Epsilon: {}", epsilon);

    let gamma_decimal = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_decimal = isize::from_str_radix(&epsilon, 2).unwrap();

    // println!("Gamma Decimal: {}", gamma_decimal);
    // println!("Epsilon Decimal: {}", epsilon_decimal);

    println!("Day 3 - Part 1 / Result:");
    println!("Final result: {}", gamma_decimal * epsilon_decimal);
    println!("");
}