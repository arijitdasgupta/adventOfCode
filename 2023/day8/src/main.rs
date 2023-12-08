use std::{io::{self, BufRead}, collections::HashMap};
use regex::Regex;
// (\w{3})\W=\W\((\w{3})\,\W(\w{3})\)

#[derive(Debug)]
enum Directions {
    L,
    R
}

#[derive(Debug)]
struct NetworkUnit {
    l: u32,
    r: u32,
}

fn string_to_u32(s: &str) -> u32 {
    let low_chars = s.to_ascii_lowercase();
    let chars = low_chars.as_bytes();
    let [c1, c2, c3] = [chars[0] as u32, chars[1] as u32, chars[2] as u32];

    return (c3 << 16) + (c2 << 8)+ (c1 << 0);
}

fn is_last_pos(i: u32, c: u8) -> bool {
    let last_byte = (i >> 16) as u8;
    return (last_byte ^ c) == 0 ;
}

fn does_all_ends_with(positions: &Vec<u32>, ch: &char) -> bool {
    let low_ch = ch.to_ascii_lowercase();
    for i in positions.iter() {
        if !is_last_pos(*i, low_ch as u8) {
            return false
        }
    }

    return true;
}

fn main() {
    // Reading file
    let stdin = io::stdin();
    let stdin_handler = stdin.lock();

    let direction_regex = Regex::new(r"(\w{3})\W=\W\((\w{3})\,\W(\w{3})\)").unwrap();

    let mut directions: Vec<Directions> = Vec::new();
    let mut network: HashMap<u32, NetworkUnit> = HashMap::new();

    assert_eq!(does_all_ends_with(&vec!(string_to_u32("AAZ"),string_to_u32("ASZ"), string_to_u32("FFZ")), &'Z'), true);
    assert_eq!(does_all_ends_with(&vec!(string_to_u32("AAZ"),string_to_u32("AAX"), string_to_u32("FFS")), &'Z'), false);

    for line in stdin_handler.lines().into_iter() {
        if let Ok(l) = line {
            // If results don't match regex
            if direction_regex.is_match(&l) {
                let (_, [origin, dest_l, dest_r]) = direction_regex.captures(&l).map(|caps| caps.extract()).unwrap();

                network.insert(string_to_u32(origin), NetworkUnit { l: string_to_u32(dest_l), r: string_to_u32(dest_r) });
            } else {
                for char in l.chars() {
                    if char == 'L' {
                        directions.push(Directions::L);
                    } else if char == 'R' {
                        directions.push(Directions::R);
                    }
                }
            }
        }
    }

    let mut steps: usize = 0;
    let mut current_position = string_to_u32("AAA");

    let final_position = string_to_u32("ZZZ");

    while current_position != final_position {
        let direction = directions.get(steps % directions.len()).unwrap();

        let next_position;
        
        match direction {
            Directions::L => next_position = network.get(&current_position).unwrap().l, 
            Directions::R => next_position = network.get(&current_position).unwrap().r,
        } 
        
        // Updating next position
        current_position = next_position;
        steps += 1;
    }

    println!("Part 1: {}", steps);

    let mut steps: usize = 0;

    let mut current_positions_v: Vec<u32> = Vec::new();
    for key in network.keys().into_iter() {
        if does_all_ends_with(&vec!(*key), &'A') {
            println!("{:b}", key);
            current_positions_v.push(*key);
        }
    }

    while !does_all_ends_with(&current_positions_v, &'Z') {
        let direction = directions.get(steps % directions.len()).unwrap();

        let mut next_positions: Vec<u32> = Vec::new();
        
        match direction {
            Directions::L => {
                for position in current_positions_v.iter() {
                    next_positions.push(network.get(position).unwrap().l);
                }
             },
            Directions::R => {
                for position in current_positions_v.iter() {
                    next_positions.push(network.get(position).unwrap().r);
                }
            }
        } 
        
        // Updating next position
        steps += 1;

        current_positions_v.clear();
        for p in next_positions.iter() {
            current_positions_v.push(*p);
        }
    }

    println!("Part 2: {}", steps);
}

