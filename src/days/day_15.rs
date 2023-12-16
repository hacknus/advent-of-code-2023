use crate::problem::Problem;
use indexmap::IndexMap;
use itertools::Itertools;
use std::fs::read_to_string;

pub struct DayFifthteen {}

impl Problem for DayFifthteen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_to_string(input).unwrap();
        let strings = contents
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut sum = 0;

        for string in strings.iter() {
            let mut hashed = 0;
            for char in string.chars() {
                hashed += char as u32;
                hashed *= 17;
                hashed = hashed % 256;
            }
            sum += hashed;
        }
        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_to_string(input).unwrap();
        let strings = contents
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut lenses: IndexMap<String, u32> = IndexMap::new();

        for string in strings.iter() {
            if string.contains("-") {
                let parts = string
                    .split("-")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let label = &parts[0];
                lenses.shift_remove(label);
            } else if string.contains("=") {
                let parts = string
                    .split("=")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let label = &parts[0];
                let focal_length = &parts[1].parse::<u32>().unwrap();
                if lenses.contains_key(label) {
                    lenses
                        .entry(label.to_string())
                        .and_modify(|f| *f = *focal_length);
                } else {
                    lenses.insert(label.to_string(), *focal_length);
                }
            }
        }

        let mut boxes = vec![vec![]; 256];
        for (label, _) in lenses.iter() {
            let mut hashed_label = 0;
            for char in label.chars() {
                hashed_label += char as u32;
                hashed_label *= 17;
                hashed_label = hashed_label % 256;
            }
            if !boxes[hashed_label as usize].contains(label) {
                boxes[hashed_label as usize].push(label.to_string());
            }
        }

        let mut sum = 0;
        for (label, focal_length) in lenses.iter() {
            let box_number = boxes.iter().position(|a| a.contains(label)).unwrap();
            let slot = boxes[box_number].iter().position(|a| a == label).unwrap() + 1;
            sum += (slot as u32) * (box_number as u32 + 1) * focal_length;
        }
        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
