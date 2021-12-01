use std::fs;

pub fn part1() {
    let path = "src/day1/inputs/input-part-1.txt";

    let contents = fs::read_to_string(path).expect("File reading failed.");

    let readings = contents.split("\n");

    let mut last_reading = -1;
    let mut increase_count = 0;

    for reading in readings {
        let current_reading = reading.parse().unwrap_or(0);
        
        if last_reading == -1 {
            last_reading = current_reading;
            // println!("{} (N/A - no previous measurement)", current_reading);
            continue;
        }

        if last_reading < current_reading {
            increase_count = increase_count + 1;
            last_reading = current_reading;
            // println!("{} (increased)", current_reading);
            continue;
        }

        last_reading = current_reading;
        // println!("{} (decreased)", current_reading);
    }

    println!("Day 1 - Part 1 / Result:");
    println!("Incresed total count: {}", increase_count);
    println!("")
}

fn parse(value: std::string::String) -> Vec<i32> {
    let readings: Vec<&str>  = value.split("\n").collect();

    let mut readings_parsed: Vec<i32> = Vec::new();

    for reading in readings {
        let parsed = reading.parse().unwrap_or(0);

        readings_parsed.push(parsed);
    }

    return readings_parsed;
}


pub fn part2() {
    let path = "src/day1/inputs/input-part-2.txt";
    let contents = fs::read_to_string(path).expect("File reading failed.");
    let slice = parse(contents);

    let mut increase_count = 0;

    for window in slice.windows(4) {
        if let [first, second, third, fourth] = window {
            let a = first + second + third;
            let b = second + third + fourth;

            if a < b {
                increase_count = increase_count + 1;
                // println!("{} - {} (increased)", a, b);
                continue;
            }

            if a == b {
                // println!("{} - {} (no change)", a, b);
                continue;
            }

            if a > b {
                // println!("{} - {} (decreased)", a, b);
                continue;
            }
        }
    }

    println!("Day 1 - Part 2 / Result:");
    println!("Incresed total count: {}", increase_count);
    println!("")
}
