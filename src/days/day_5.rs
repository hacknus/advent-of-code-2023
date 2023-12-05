use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayFive {}

pub enum ParseState {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl ParseState {
    pub fn from_str(s: &str) -> Option<Self> {
        if s.contains("seed-to-soil map") {
            Some(Self::SeedToSoil)
        } else if s.contains("soil-to-fertilizer map") {
            Some(Self::SoilToFertilizer)
        } else if s.contains("fertilizer-to-water map") {
            Some(Self::FertilizerToWater)
        } else if s.contains("water-to-light map") {
            Some(Self::WaterToLight)
        } else if s.contains("light-to-temperature map") {
            Some(Self::LightToTemperature)
        } else if s.contains("temperature-to-humidity map") {
            Some(Self::TemperatureToHumidity)
        } else if s.contains("humidity-to-location map") {
            Some(Self::HumidityToLocation)
        } else {
            None
        }
    }
}

fn map(val: &mut u64, destination: u64, source: u64, length: u64) -> bool {
    if (source..(source + length)).contains(val) {
        *val = destination + (*val - source);
        true
    } else {
        false
    }
}

impl Problem for DayFive {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut parser_state = ParseState::Seeds;
        let mut seeds = vec![];
        let mut seed_to_soil = vec![];
        let mut soil_to_fertilizer = vec![];
        let mut fertilizer_to_water = vec![];
        let mut water_to_light = vec![];
        let mut light_to_temperature = vec![];
        let mut temperature_to_humidity = vec![];
        let mut humidity_to_location = vec![];

        for line in contents {
            if line.is_empty() {
                continue;
            }

            if line.contains("seeds:") {
                let parts = line.split(": ").collect::<Vec<&str>>();
                seeds = parts[1].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                continue;
            }

            if let Some(state) = ParseState::from_str(&line) {
                parser_state = state;
                continue;
            }

            match parser_state {
                ParseState::Seeds => { unreachable!() }
                ParseState::SeedToSoil => {
                    let parts = line.split(": ").collect::<Vec<&str>>();
                    seed_to_soil.push(parts[0].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
                ParseState::SoilToFertilizer => {
                    let parts = line.split(": ").collect::<Vec<&str>>();
                    soil_to_fertilizer.push(parts[0].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
                ParseState::FertilizerToWater => {
                    let parts = line.split(": ").collect::<Vec<&str>>();
                    fertilizer_to_water.push(parts[0].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
                ParseState::WaterToLight => {
                    let parts = line.split(": ").collect::<Vec<&str>>();
                    water_to_light.push(parts[0].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
                ParseState::LightToTemperature => {
                    let parts = line.split(": ").collect::<Vec<&str>>();
                    light_to_temperature.push(parts[0].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
                ParseState::TemperatureToHumidity => {
                    let parts = line.split(": ").collect::<Vec<&str>>();
                    temperature_to_humidity.push(parts[0].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
                ParseState::HumidityToLocation => {
                    let parts = line.split(": ").collect::<Vec<&str>>();
                    humidity_to_location.push(parts[0].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
            }
        }
        for seed in seeds.iter_mut() {
            for map_vec in seed_to_soil.iter() {
                if map(seed, map_vec[0], map_vec[1], map_vec[2]) {
                    break;
                }
            }
            for map_vec in soil_to_fertilizer.iter() {
                if map(seed, map_vec[0], map_vec[1], map_vec[2]) {
                    break;
                }
            }
            for map_vec in fertilizer_to_water.iter() {
                if map(seed, map_vec[0], map_vec[1], map_vec[2]) {
                    break;
                }
            }
            for map_vec in water_to_light.iter() {
                if map(seed, map_vec[0], map_vec[1], map_vec[2]) {
                    break;
                }
            }
            for map_vec in light_to_temperature.iter() {
                if map(seed, map_vec[0], map_vec[1], map_vec[2]) {
                    break;
                }
            }
            for map_vec in temperature_to_humidity.iter() {
                if map(seed, map_vec[0], map_vec[1], map_vec[2]) {
                    break;
                }
            }
            for map_vec in humidity_to_location.iter() {
                if map(seed, map_vec[0], map_vec[1], map_vec[2]) {
                    break;
                }
            }
        }

        format!("{}", seeds.iter().min().unwrap())
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut parser_state = ParseState::Seeds;
        let mut seeds = vec![];
        let mut seed_to_soil = vec![];
        let mut soil_to_fertilizer = vec![];
        let mut fertilizer_to_water = vec![];
        let mut water_to_light = vec![];
        let mut light_to_temperature = vec![];
        let mut temperature_to_humidity = vec![];
        let mut humidity_to_location = vec![];

        for line in contents {
            if line.is_empty() {
                continue;
            }

            if line.contains("seeds:") {
                let parts = line.split(": ").collect::<Vec<&str>>();
                seeds = parts[1].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                continue;
            }

            if let Some(state) = ParseState::from_str(&line) {
                parser_state = state;
                continue;
            }

            match parser_state {
                ParseState::Seeds => { unreachable!() }
                ParseState::SeedToSoil => {
                    let parts = line.split(": ").collect::<Vec<&str>>();
                    seed_to_soil.push(parts[0].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
                ParseState::SoilToFertilizer => {
                    let parts = line.split(": ").collect::<Vec<&str>>();
                    soil_to_fertilizer.push(parts[0].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
                ParseState::FertilizerToWater => {
                    let parts = line.split(": ").collect::<Vec<&str>>();
                    fertilizer_to_water.push(parts[0].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
                ParseState::WaterToLight => {
                    let parts = line.split(": ").collect::<Vec<&str>>();
                    water_to_light.push(parts[0].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
                ParseState::LightToTemperature => {
                    let parts = line.split(": ").collect::<Vec<&str>>();
                    light_to_temperature.push(parts[0].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
                ParseState::TemperatureToHumidity => {
                    let parts = line.split(": ").collect::<Vec<&str>>();
                    temperature_to_humidity.push(parts[0].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
                ParseState::HumidityToLocation => {
                    let parts = line.split(": ").collect::<Vec<&str>>();
                    humidity_to_location.push(parts[0].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
            }
        }

        let mut new_seeds = (seeds[0]..(seeds[0] + seeds[1])).collect::<Vec<u64>>();

        for i in (2..seeds.len()).step_by(2) {
            new_seeds.append(&mut (seeds[i]..(seeds[i] + seeds[i + 1])).collect::<Vec<u64>>());
        }

        for seed in new_seeds.iter_mut() {
            for map_vec in seed_to_soil.iter() {
                if map(seed, map_vec[0], map_vec[1], map_vec[2]) {
                    break;
                }
            }
            for map_vec in soil_to_fertilizer.iter() {
                if map(seed, map_vec[0], map_vec[1], map_vec[2]) {
                    break;
                }
            }
            for map_vec in fertilizer_to_water.iter() {
                if map(seed, map_vec[0], map_vec[1], map_vec[2]) {
                    break;
                }
            }
            for map_vec in water_to_light.iter() {
                if map(seed, map_vec[0], map_vec[1], map_vec[2]) {
                    break;
                }
            }
            for map_vec in light_to_temperature.iter() {
                if map(seed, map_vec[0], map_vec[1], map_vec[2]) {
                    break;
                }
            }
            for map_vec in temperature_to_humidity.iter() {
                if map(seed, map_vec[0], map_vec[1], map_vec[2]) {
                    break;
                }
            }
            for map_vec in humidity_to_location.iter() {
                if map(seed, map_vec[0], map_vec[1], map_vec[2]) {
                    break;
                }
            }
        }

        format!("{}", new_seeds.iter().min().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}