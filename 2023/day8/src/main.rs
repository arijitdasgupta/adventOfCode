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

enum From {
    L,
    R
}


struct InverseNode {
    node: u32,
    from_direction: Directions,
}

fn main() {
    // Reading file
    let stdin = io::stdin();
    let stdin_handler = stdin.lock();

    let node_regex = Regex::new(r"(\w{3})\W=\W\((\w{3})\,\W(\w{3})\)").unwrap();

    let mut directions: HashMap<u64, Directions> = HashMap::new();
    let mut direction_size: u64 = 0;
    let mut network: HashMap<u32, NetworkUnit> = HashMap::new();
    
    // 26 * 3
    let _: Vec<NetworkUnit> = Vec::with_capacity(17576);

    assert_eq!(does_all_ends_with(&vec!(string_to_u32("AAZ"),string_to_u32("ASZ"), string_to_u32("FFZ"), string_to_u32("BBZ"), string_to_u32("CCZ")), &'Z'), true);
    assert_eq!(does_all_ends_with(&vec!(string_to_u32("AAZ"),string_to_u32("AAX"), string_to_u32("FFS")), &'Z'), false);

    for line in stdin_handler.lines().into_iter() {
        if let Ok(l) = line {
            // If results don't match regex
            if node_regex.is_match(&l) {
                let (_, [origin, dest_l, dest_r]) = node_regex.captures(&l).map(|caps| caps.extract()).unwrap();

                network.insert(
                    string_to_u32(origin), 
                    NetworkUnit { l: string_to_u32(dest_l), r: string_to_u32(dest_r) }
                );
            } else {
                for (i, char) in l.chars().enumerate() {
                    if char == 'L' {
                        directions.insert(i.try_into().unwrap(), Directions::L);
                    } else if char == 'R' {
                        directions.insert(i.try_into().unwrap(), Directions::R);
                    }
                    direction_size += 1;
                }
            }
        }
    }

    // Immutable now :-D
    let direction_size = direction_size;
    let directions = directions;

    let mut steps: u64 = 0;
    let mut current_position = string_to_u32("AAA");

    let final_position = string_to_u32("ZZZ");

    while current_position != final_position {
        let directional_index = steps % direction_size;
        let direction = directions.get(&directional_index).unwrap();

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

    // End of part 1

    let key_iter = network.keys().into_iter();

    let end_positions = Vec::from_iter(
        key_iter.clone()
        .filter(|k| { does_all_ends_with(&vec!(**k), &'z') })
        .map(|x| { *x })
    );

    let mut inverse_tree: Vec<&mut Vec<Directions>> = Vec::new();

    while true {
        // Find things that are coming from end_positions
        for (index, node) in end_positions.iter().enumerate() {
            // find where i can come from.
            for network_key in network.keys().into_iter() {
                let network_value = network.get(network_key).unwrap();
                if network_value.l == *node {
                    match inverse_tree.get(index) {
                        Some(vec) => {
                            *vec.push(new_value);
                        },
                        None => {
                            let mut vec = Vec::new()
                            vec.push(new_value);
    
                            inverse_tree.
                        }
                    }
                } else if network_value.r == *node {
                    new_value = Directions::R;
                }
            }
        }
    }

}

