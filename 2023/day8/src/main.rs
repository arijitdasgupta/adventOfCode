use std::{io::{self, BufRead}, collections::HashMap};
use regex::Regex;
// (\w{3})\W=\W\((\w{3})\,\W(\w{3})\)

#[derive(Debug)]
enum Direction {
    L,
    R
}

#[derive(Debug)]
struct NetworkUnit {
    l: u32,
    r: u32,
}

type Network = HashMap<position, NetworkUnit>;

type Directions = Vec<u64, Direction>;

type Position = u32;

struct PositionUnit {
    position: &Position,
    direction_index: &u64,
    directions: &Directions,
    network: &Network,
}

impl PositionUnit {
    fn traverse(self: &Self) -> () {
        
    }
}

fn string_to_u32(s: &str) -> Position {
    let low_chars = s.to_ascii_lowercase();
    let chars = low_chars.as_bytes();
    let [c1, c2, c3] = [chars[0] as u32, chars[1] as u32, chars[2] as u32];

    return (c3 << 16) + (c2 << 8)+ (c1 << 0);
}

fn is_last_pos(i: Position, c: u8) -> bool {
    let last_byte = (i >> 16) as u8;
    return (last_byte ^ c) == 0 ;
}

fn main() {
    // Reading file
    let stdin = io::stdin();
    let stdin_handler = stdin.lock();

    let node_regex = Regex::new(r"(\w{3})\W=\W\((\w{3})\,\W(\w{3})\)").unwrap();

    let mut directions: HashMap<u64, Directions> = HashMap::new();
    let mut direction_size: u64 = 0;
    let mut network: HashMap<u32, NetworkUnit> = HashMap::new();

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

}

