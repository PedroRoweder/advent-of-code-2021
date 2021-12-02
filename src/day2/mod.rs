use std::fs;

pub fn part1() {
    let path = "src/day2/inputs/input-part-1.txt";
    let contents = fs::read_to_string(path).expect("File reading failed.");
    let directions = contents.split("\n");

    let mut depth = 0;
    let mut h_position = 0;

    for d in directions {
        let direction: Vec<&str> = d.split(" ").collect();
        
        let movement = direction[0];
        let amount = direction[1].parse().unwrap_or(0);

        match movement {
            "forward" => { h_position += amount },
            "back" => { h_position -= amount },
            "up" => { depth -= amount },
            "down" => { depth += amount },
            _ => { println!("Movement not recognized.") }
        }
    }

    // println!("Final horizontal position: {}", h_position);
    // println!("Final depth: {}", depth);

    let final_position = h_position * depth;

    println!("Day 2 - Part 1 / Result:");
    println!("Final position: {}", final_position);
    println!("")
}