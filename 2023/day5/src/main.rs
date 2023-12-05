use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
seeds: SEEDS[]
seed-to-soil map:
DEST SRC RANGE[\n]
soil-to-fertilizer map:
DEST SRC RANGE[\n]
fertilizer-to-water map:
DEST SRC RANGE[\n]
water-to-light map:
DEST SRC RANGE[\n]
light-to-temperature map:
DEST SRC RANGE[\n]
temperature-to-humidity map:
DEST SRC RANGE[\n]
humidity-to-location map:
DEST SRC RANGE[\n] 
 */


enum ReadState {
    Seeds,
    SeedToSoil,
    SoilToFert,
    // FertToWater,
    // WaterToLight,
    // LightToTemp,
    // TempToHum,
    // HumToLoc,
}

#[derive(Clone)]
enum UnitType {
    Seed,
    Soil,
    Fert,
    Water,
    Light,
    Temp,
    Hum,
    Loc
}

impl UnitType {
    fn transition(self: &Self) -> Self {
        match self {
            UnitType::Seed => UnitType::Soil,
            UnitType::Soil => UnitType::Fert,
            UnitType::Fert => UnitType::Water,
            UnitType::Water => UnitType::Light,
            UnitType::Light => UnitType::Temp,
            UnitType::Temp => UnitType::Hum,
            UnitType::Hum => UnitType::Loc,
            _ => UnitType::Loc,
        }
    }
}
// I can probably do it better than manually defining these two.

struct DataUnit {
    value: u64,
    kind: UnitType,
}

struct RangeSpec {
    dest: u64,
    src: u64,
    range: u64,
}

impl RangeSpec {
    fn from_numbers(n: &Vec<u64>) -> Self {
        let dest = *n.get(0).unwrap(); // Might panic but YOLO!
        let src = *n.get(1).unwrap(); // Duh
        let range = *n.get(2).unwrap(); // Duuuh!

        return Self { dest, src, range };
    }

    fn get_dest(self: &Self, src: &DataUnit) -> DataUnit {
        let val = src.value;

        if val > (self.src + self.range) {
            let difference = val - self.src;
            return DataUnit {
                value: self.dest + difference,
                kind: src.kind.transition(),
            }; 
        } else {
            return *src;
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

fn numbers_from_strings(ss: &Vec<&str>) -> Box<Vec<u64>> {
   let mut result: Vec<u64> = Vec::new();
   for s in ss {
        result.push(s.parse::<u64>().unwrap());
   }

   return Box::new(result);
}

fn process_line(line: &str, store: &mut Box<Vec<DataUnit>>) -> Box<Vec<DataUnit>> {
    let words_in_line = line.split(" ").collect();
    let numbers = numbers_from_strings(&words_in_line);
    let range_spec = RangeSpec::from_numbers(&numbers);
    
    let mut new_store = Box::new(Vec::new());

    for unit in store.iter() {
        let new_data_unit = range_spec.get_dest(&unit);
        new_store.push(new_data_unit);
    }

    return new_store;
} 

fn main() {
    if let Ok(lines) = read_lines(Path::new("./day5input.txt")) {
        let mut reading_state = UnitType::Seed; // Assuming the input file starts with seeds
        // Stores a vector of numbers, initially those are seeds, but eventually they become something else.
        let mut store: Box<Vec<DataUnit>> = Box::new(Vec::new());
        for line in lines {
            if let Ok(line) = line {
                if line.starts_with("seeds:") {
                    reading_state = UnitType::Seed;
                    let words_tail: Vec<&str> = line.split(" ").filter(|x| { !x.eq(&"seeds:") }).collect();

                    // debug_assert!(words_tail.len() > 0);
                    let numbers = numbers_from_strings(&words_tail);
                    // Clearing vector just in case, hopefully it never gets called again
                    // store.clear(); 
                    for n in numbers.iter() {
                        store.push(DataUnit{
                            value: *n,
                            kind: UnitType::Seed
                        });
                    }
                } else if line.starts_with("soil-to-fertilizer map:") {
                    reading_state = UnitType::Soil;
                } else {
                    if line.trim().len() > 0 {
                        match reading_state {
                            UnitType::Seed => {
                                store = process_line(&line, &mut store);
                            },
                            _ => {
                                // Check of unconverted stuff, and convert them into something new.
                                store = process_line(&line, &mut store);
                            },
                        }
                    }
                }
            }
        }
    }
}

