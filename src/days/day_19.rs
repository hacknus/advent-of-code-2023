use crate::io::read_file_lines;
use crate::problem::Problem;
use itertools::izip;
use regex::Regex;
use std::collections::HashMap;
use std::num::Wrapping;

pub struct DayNineteen {}

#[derive(Debug, Clone)]
struct Workflow {
    keys: Vec<String>,
    operators: Vec<String>,
    numbers: Vec<u32>,
    outputs: Vec<String>,
    otherwise: String,
}

impl Workflow {
    fn run(&mut self, input: &HashMap<String, u32>) -> String {
        for (key, operator, number, output) in izip!(
            self.keys.clone(),
            self.operators.clone(),
            self.numbers.clone(),
            self.outputs.clone()
        ) {
            match operator.as_str() {
                ">" => {
                    if let Some(val) = input.get(&key) {
                        if *val > number {
                            return output.clone();
                        }
                    }
                }
                "<" => {
                    if let Some(val) = input.get(&key) {
                        if *val < number {
                            return output.clone();
                        }
                    }
                }
                _ => panic!("invalid operator"),
            }
        }
        self.otherwise.clone()
    }
}

impl Problem for DayNineteen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut rules = vec![];
        let mut parts_string = vec![];
        let mut newline_passed = false;
        for line in contents {
            if line == "".to_string() {
                newline_passed = true;
                continue;
            }
            if !newline_passed {
                rules.push(line);
            } else {
                parts_string.push(line);
            }
        }
        // this would be the complete regex: "([a-z]+)\{(([amsx]+)([><])([0-9]+):([AR]|[a-z]*),)+([AR]|[a-z]*)\}"
        let rules_regex =
            r"(?<key>[amsx]+)*(?<operator>[><])*(?<number>[0-9]+)*:*(?<output>[AR]|[a-z]*)";
        let rg = Regex::new(rules_regex).unwrap();

        let mut workflows = HashMap::new();
        for rule in rules {
            let parts = rule
                .split("{")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let name = parts[0].clone();
            let rule = parts[1].replace("}", "");
            let rule_parts = rule
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let mut operators = vec![];
            let mut numbers = vec![];
            let mut keys = vec![];
            let mut outputs = vec![];
            let mut otherwise = "".to_string();
            for rule_part in rule_parts {
                match rg.captures(&rule_part) {
                    // Access captures groups via Captures::at
                    // Prints Some("2016")
                    Some(x) => {
                        if let Some(operator) = x.name("operator") {
                            operators.push(operator.as_str().to_string());
                            numbers
                                .push(x.name("number").unwrap().as_str().parse::<u32>().unwrap());
                            keys.push(x.name("key").unwrap().as_str().to_string());
                            outputs.push(x.name("output").unwrap().as_str().to_string());
                        } else {
                            otherwise = x.name("key").map(|s| s.as_str()).unwrap_or("").to_string()
                                + x.name("output").map(|s| s.as_str()).unwrap_or("");
                        }
                    }
                    None => {}
                }
            }
            let workflow = Workflow {
                keys,
                operators,
                numbers,
                outputs,
                otherwise,
            };
            workflows.insert(name, workflow);
        }

        let parts_regex = r"(?<name>[axms])=(?<number>[0-9]+)";
        let rg = Regex::new(parts_regex).unwrap();
        let mut parts = vec![];
        for part_string in parts_string {
            let mut part = HashMap::new();
            for capture in rg.find_iter(&part_string) {
                // Access captures groups via Captures::at
                // Prints Some("2016")
                let x = capture;
                let p = x
                    .as_str()
                    .split("=")
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>();
                part.insert(p[0].clone(), p[1].parse::<u32>().unwrap());
            }
            parts.push(part);
        }

        let mut sum = 0;

        for part in parts.iter() {
            let workflow = workflows.get_mut("in").unwrap();
            let mut next = workflow.run(part);
            while &next != "A" && &next != "R" {
                let workflow = workflows.get_mut(&next).unwrap();
                next = workflow.run(part);
            }
            if &next == "A" {
                sum += part.values().sum::<u32>();
            }
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
