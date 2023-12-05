use core::num;
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

enum DataType {
    Seed,
    Soil,
    Fert,
    Water,
    Temp,
    Hum,
    Loc
}
// I can probably do it better than manually defining these two.
enum DataUnit {
    Seed(u64),
    Soil(u64),
    Fert(u64),
    Water(u64),
    Temp(u64),
    Hum(u64),
    Loc(u64)
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

    fn get_dest(self: &Self, src: u64) -> u64 {
        if (self.src + self.range) < src {
            let difference = src - self.src;
            return self.dest + difference;
        } else {
            return src;
        }
    }
}

impl DataUnit {
    fn new(v: u64, d: DataType) -> Self {
        match d {
            DataType::Fert => DataUnit::Fert(v),
            DataType::Seed => DataUnit::Seed(v),
            DataType::Hum => DataUnit::Hum(v),
            DataType::Loc => DataUnit::Loc(v),
            DataType::Soil => DataUnit::Soil(v),
            DataType::Temp => DataUnit::Temp(v),
            DataType::Water => DataUnit::Water(v)
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



fn main() {
    if let Ok(lines) = read_lines(Path::new("./day5input.txt")) {
        let mut reading_state = ReadState::Seeds; // Assuming the input file starts with seeds
        // Stores a vector of numbers, initially those are seeds, but eventually they become something else.
        let mut store: &mut Vec<DataUnit> = &mut Vec::new();
        for line in lines {
            if let Ok(line) = line {
                if line.starts_with("seeds:") {
                    reading_state = ReadState::Seeds;
                    let words_tail: Vec<&str> = line.split(" ").filter(|x| { !x.eq(&"seeds:") }).collect();

                    // debug_assert!(words_tail.len() > 0);
                    let numbers = numbers_from_strings(&words_tail);
                    // Clearing vector just in case, hopefully it never gets called again
                    // store.clear(); 
                    for n in numbers.iter() {
                        store.push(DataUnit::new(*n, DataType::Seed));
                    }
                } else if line.starts_with("soil-to-fertilizer map:") {
                    reading_state = ReadState::SeedToSoil
                } else {
                    if line.trim().len() > 0 {
                        match reading_state {
                            ReadState::SeedToSoil => {
                                let words_in_line = line.split(" ").collect();
                                let numbers = numbers_from_strings(&words_in_line);
                                let range_spec = RangeSpec::from_numbers(&numbers);

                                let new_store: &mut Vec<DataUnit> = &mut Vec::new();

                                for unit in store {
                                    match unit {
                                        DataUnit::Seed(val) => {
                                            let v = val.clone();
                                            let new_data_unit = DataUnit::new(range_spec.get_dest(v), DataType::Soil);        
                                            new_store.push(new_data_unit);
                                        },
                                        _ => (),
                                    }
                                }

                                store = new_store;
                            },
                            ReadState::SoilToFert => {

                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }

}

