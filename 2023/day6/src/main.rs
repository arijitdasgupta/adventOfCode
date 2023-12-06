use core::num;
use std::{path::Path, fs::{File, self}};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn compute_combination_for_a_race(race: &Race) -> u64 {
    let mut comb: u64 = 0;
    for t in 0..race.time {
        if (t * race.time - u64::pow(t, 2)) >= race.distance {
            comb += 1;
        }
    }
    return comb;
}

fn compute_combinations(races: &Vec<Race>) -> u64 {
    let mut mul: u64 = 1;
    for race in races.iter() {
        mul *= compute_combination_for_a_race(race);
    }

    return mul;
}

fn main() {
    let filepath = Path::new("./input.txt");
    let input_str = fs::read_to_string(filepath);

    if let Ok(result) = input_str {
        let lines = result.split("\n");
        // Loading the input and parsing
        let mut times: Vec<u64> = Vec::new();
        let mut distances: Vec<u64> = Vec::new();

        for line in lines.clone() { // Yuck! I really am shit at Rust! TODO
            println!("{}", line);
            if line.starts_with("Time:") {
                let mut splitter = line.split(" ");
                splitter.next();
                while let Some(i) = splitter.next() {
                    if let Ok(n) = i.parse::<u64>() {
                        times.push(n);
                    }
                }
            } else {
                let mut splitter = line.split(" ");
                splitter.next();
                while let Some(i) = splitter.next() {
                    if let Ok(n) = i.parse::<u64>() {
                        distances.push(n);
                    }
                }
            }
        }

        let races_iter = times.iter().zip(distances.iter());
        let mut races = Vec::new();
        for (t, d) in races_iter {
            races.push(Race {
                time: *t,
                distance: *d
            });
        }
        
        let result = compute_combinations(&races);

        println!("For bad kerning of results: {:?}", result);

        let mut numeric_words: Vec<u64> = Vec::new();
        for line in lines {
            if !line.is_empty() {
                if line.starts_with("Time:") {
                    let numbers_only = line.replace("Time:", "").replace(" ", "");
                    numeric_words.push(numbers_only.trim().parse::<u64>().unwrap())
                } else {
                    let numbers_only = line.replace("Distance:", "").replace(" ", "");
                    numeric_words.push(numbers_only.trim().parse::<u64>().unwrap())
                }
            }
        }

        let one_more_race = Race {
            time: *numeric_words.get(0).unwrap(),
            distance: *numeric_words.get(1).unwrap(),
        };

        let one_more_result = compute_combination_for_a_race(&one_more_race);
        println!("{:?}", one_more_result);
    }
}
