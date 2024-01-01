// Advent of Code 2023: Day 5
// https://adventofcode.com/2023/day/5

use crate::util::read_input;

pub fn part_one() -> std::io::Result<()> {
    // Read the input file.
    let input = read_input("05")?;

    // Use the input to create a series of vectors containing maps.
    // The numbers supplied in the input file are in the form
    // <initial_index><start_number><series_length>
    // Maps are allowed to have blank spaces - these will be checked later.

    // Create the base input vector.
    let input = input.split("\r\n\r\n").collect::<Vec<&str>>();

    // Create the maps.
    println!("Creating maps...");
    let mut maps: Vec<Map> = Vec::new();
    for i in 1..input.len() {
        maps.push(create_map(input[i]));
    }

    // Create the vector of seeds.
    println!("Creating seeds vector...");
    let seeds = input[0]
        .split_whitespace()
        .filter(|x| x.parse::<usize>().is_ok())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("Seeds: {:?}", seeds);

    // There should be 7 maps in total.
    // These start with seed-to-soil and end with humidity-to-location.
    // At this point, we query the maps to link the numbers together.
    // We must link the seeds given in the first line to the soil given in the map, and so on.
    let mut seed_to_soil: Vec<usize> = Vec::new();
    for seed in seeds.iter() {
        let mut current_value = *seed;
        for map in maps.iter() {
            // Check if the current value is contained within the map.
            // If it is, we need to find the value.
            for map_range in map.iter() {
                if map_range.contains(current_value) {
                    println!("Value {} is contained within map range {:?}", current_value, map_range);
                    let value = map_range.get_value(current_value);
                    current_value = value;
                    break;
                }
            }
        }

        // We have now mapped the seed to the location.
        println!("Pushing value {}.", current_value);
        seed_to_soil.push(current_value);
    }

    // The seeds should now be linked to the soil locations.
    // We now need to find the minimum values.
    println!("Location map: {:?}", seed_to_soil);
    let min_location = seed_to_soil.iter().min().unwrap();

    println!("The minimum location is: {}", min_location);

    Ok(())
}

pub fn part_two() -> std::io::Result<()> {
    // Do the same thing, however the seeds are given in ranges.
    // There will be an exessive number of seeds.
    
    // Read the input file. and split.
    let input = read_input("05")?.split("\r\n\r\n").collect::<Vec<&str>>();

    // Create the maps.
    println!("Creating maps...");
    let mut maps: Vec<Map> = Vec::new();
    for i in 1..input.len() {
        maps.push(create_map(input[i]));
    }

    // Get the seed ranges.
    let seed_ranges = create_seed_ranges(input[0]);

    // We now need to find the smallest value that is contained within all of the maps
    // AND all of the seed ranges.

    Ok(())
}


fn create_map(input: &str) -> Vec<MapRange> {
    // Build the vector of map ranges. Each map range will contain the start
    // and end values, as well as the initial value of the range.
    let mut map_ranges: Vec<MapRange> = Vec::new();

    // Split the input into a vector of strings.
    // We will ignore the first string as it is the type of map.
    let inputs = input.split("\r\n").collect::<Vec<&str>>();

    // Use the second string and onwards to create the MapRanges,
    // which we'll push to the vector.
    for i in 1..inputs.len() {
        // Split the string into a vector of strings
        let series = inputs[i].split_whitespace().collect::<Vec<&str>>();

        // Get the values from the string.
        let initial_value = series[0].parse::<usize>().unwrap();
        let start = series[1].parse::<usize>().unwrap();
        let end = series[2].parse::<usize>().unwrap();

        // Create the MapRange.
        map_ranges.push(MapRange::new(start, start + end, initial_value));
    }

    map_ranges
}

fn create_seed_ranges(input: &str) -> Vec<SeedRange> {
    // Build the vector of seed ranges from the input.
    let inputs = input.split_whitespace().collect::<Vec<&str>>();

    // We will ignore the first string as it just says "seeds:"
    // We will have to take every other string to create the seed ranges.
    let mut seed_ranges: Vec<SeedRange> = Vec::new();

    for i in (1..inputs.len()).step_by(2) {
        let start = inputs[i].parse::<usize>().unwrap();
        let length = inputs[i + 1].parse::<usize>().unwrap();

        seed_ranges.push(SeedRange::new(start, start + length));
    }

    seed_ranges
}

// Instead of creating a vector (because that takes forever) we can create
// an algorithm to find the values we need.
// We should query the maps to find the ranges of numbers that are contained
// within them. We can then use these to find the values we need.
#[derive(Debug)]
struct MapRange {
    start: usize,
    end: usize,
    initial_value: usize
}

impl MapRange {
    fn new(start: usize, end: usize, initial_value: usize) -> Self {
        MapRange {
            start,
            end,
            initial_value
        }
    }

    fn contains(&self, value: usize) -> bool {
        value >= self.start && value <= self.end
    }

    fn get_value(&self, value: usize) -> usize {
        println!("{} + ({} - {})", self.initial_value, value, self.start);
        self.initial_value + (value - self.start)
    }
}

type Map = Vec<MapRange>;  // May need to create a new type for this, not an alias.

#[derive(Debug)]
struct SeedRange {
    start: usize,
    end: usize,
}

impl SeedRange {
    fn new(start: usize, end: usize) -> Self {
        SeedRange {
            start,
            end
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_five::{part_one, part_two};

    #[test]
    fn day_five() {
        part_one().unwrap();
        part_two().unwrap();
    }
}
