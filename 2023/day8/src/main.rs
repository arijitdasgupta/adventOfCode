use std::{io::{self, BufRead}, collections::HashMap, borrow::BorrowMut, thread};
use regex::Regex;
// (\w{3})\W=\W\((\w{3})\,\W(\w{3})\)

#[derive(Debug, Clone)]
enum Direction {
    L,
    R
}

type Position = u32;

type SignificantPosition = u8;

#[derive(Debug, Clone)]
struct NetworkUnit {
    l: u32,
    r: u32,
}

type Network = HashMap<Position, NetworkUnit>;

#[derive(Clone)]
struct Directions {
    sequence: Vec<Direction>,
    size: u64,
}

impl Directions {
    fn new(seq: &Vec<Direction>, size: usize) -> Self {
        return Directions {
            sequence: seq.to_vec(),
            size: size as u64,
        }
    }

    fn get(self: &Self, index: u64) -> Direction {
        let i = index % self.size;

        return self.sequence.get(i as usize).unwrap().clone();
    }
}

#[derive(Clone)]
struct PositionUnit {
    position: Position,
    direction_index: u64,
    directions: Directions,
    network: Network,
}

impl PositionUnit {
    fn new(position: Position, directions: &Directions, network: &Network) -> Self {
        PositionUnit { 
            position,
            directions: directions.clone(),
            network: network.clone(),
            direction_index: 0,
        }
    }

    fn until_position(self: &mut Self, final_pos: Position) -> &mut Self {
        let mut current_position = self.position;
        let mut current_direction_index = self.direction_index;

        while current_position != final_pos {
            let new_direction = self.directions.get(current_direction_index);
            let new_position = match new_direction {
                Direction::L => self.network.get(&current_position).unwrap().l,
                Direction::R => self.network.get(&current_position).unwrap().r,
            };
            current_direction_index = current_direction_index + 1;
            current_position = new_position;
        }

        self.position = current_position;
        self.direction_index = current_direction_index;

        return self;
    }

    fn until_last_pos(self: &mut Self, target: SignificantPosition) -> &mut Self {
        while !self.is_last_pos(target) {
            let new_direction = self.directions.get(self.direction_index);
            let new_position = match new_direction {
                Direction::L => self.network.get(&self.position).unwrap().l,
                Direction::R => self.network.get(&self.position).unwrap().r,
            };
            self.position = new_position;
            self.direction_index += 1;
        }

        return self;
    }

    fn until_step(self: &mut Self, target_direction_index: u64) {
        let mut current_position = self.position;
        let mut current_direction_index = self.direction_index;

        while current_direction_index < target_direction_index {
            let new_direction = self.directions.get(current_direction_index);
            let new_position = match new_direction {
                Direction::L => self.network.get(&current_position).unwrap().l,
                Direction::R => self.network.get(&current_position).unwrap().r,
            };

            current_position = new_position;
            current_direction_index += 1;
        }

        self.position = current_position;
        self.direction_index = current_direction_index;

        
        assert_eq!(current_direction_index, target_direction_index);
    }

    fn is_last_pos(self: &Self, c: SignificantPosition) -> bool {
        let pos = self.position;
        let last_byte = (pos >> 16) as u8;
        return (last_byte ^ c) == 0 ;
    }

    fn are_all_ending_with(selfs: &Vec<&mut Self>, target: SignificantPosition) -> bool {
        if let Some(_) = selfs.iter().find(|p| { !p.is_last_pos(target as u8)}) {
            return false;
        } else {
            return true;
        }
    }
}

fn str_to_position(s: &str) -> Position {
    let low_chars = s.to_ascii_lowercase();
    let chars = low_chars.as_bytes();
    let [c1, c2, c3] = [chars[0] as u32, chars[1] as u32, chars[2] as u32];

    return (c3 << 16) + (c2 << 8)+ (c1 << 0);
}

fn main() {
    // Reading file
    let stdin = io::stdin();
    let stdin_handler = stdin.lock();

    let node_regex = Regex::new(r"(\w{3})\W=\W\((\w{3})\,\W(\w{3})\)").unwrap();

    let mut direction_sequence: Vec<Direction> = Vec::new();
    let mut network: HashMap<u32, NetworkUnit> = HashMap::new();

    for line in stdin_handler.lines().into_iter() {
        if let Ok(l) = line {
            // If results don't match regex
            if node_regex.is_match(&l) {
                let (_, [origin, dest_l, dest_r]) = node_regex.captures(&l).map(|caps| caps.extract()).unwrap();

                network.insert(
                    str_to_position(origin), 
                    NetworkUnit { l: str_to_position(dest_l), r: str_to_position(dest_r) }
                );
            } else {
                for (i, char) in l.chars().enumerate() {
                    if char == 'L' {
                        direction_sequence.push(Direction::L);
                    } else if char == 'R' {
                        direction_sequence.push(Direction::R);
                    }
                }
            }
        }
    }

    let directions = Directions::new(
        &direction_sequence,
        direction_sequence.len()
    );

    let mut singular_position = PositionUnit::new(
        str_to_position("AAA"),
        &directions,
        &network
    );

    singular_position.until_position(str_to_position("ZZZ"));

    println!("First part: {}", singular_position.direction_index);

    let all_positions = network.keys().into_iter();

    let starting_positions = all_positions
        .map(|key| { PositionUnit::new(*key, &directions, &network) })
        .filter(|p| { p.is_last_pos('a' as u8)})
        .take(1)
        .map(|i| { &mut i });

    let positions: Vec<&mut PositionUnit> = Vec::from_iter(starting_positions);

    while !PositionUnit::are_all_ending_with(&positions, 'z' as u8) {
        let p_threads = positions
            .iter()
            .map(|p| { thread::spawn(move || {
                p.until_step(p.direction_index + 1);
                let _ = p.until_last_pos('z' as u8);
                return;
            } )});

        for t in p_threads {
            dbg!("Before join");
            t.join().expect("BOOM!");
            dbg!("After join");
        }
    } 

    println!("Part 2: {}", positions.first().unwrap().direction_index);

    // while let Some(pos) = positions.get_mut(current_position_index) {
    //     if current_position_index != 0 {
    //         pos.until_step(current_direction_index);
    //         if pos.is_last_pos('z' as u8) {
    //             current_position_index += 1;
    //         } else {
    //             current_position_index = 0;
                
    //         }
    //     } else {
    //         pos.until_step(pos.direction_index + 1);
    //         pos.until_last_pos('z' as u8);

    //         current_direction_index = pos.direction_index;
    //         current_position_index += 1;
    //     }
    // }
}

