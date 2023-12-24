use crate::io::read_file_lines;
use crate::problem::Problem;
use itertools::Itertools;
use std::collections::HashSet;

pub struct DayTwentyFour {}

struct Particle {
    id: usize,
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

fn check_intersection(a: &Particle, b: &Particle, lower_bound: f64, upper_bound: f64) -> bool {
    let slope_a = a.velocity.1 / a.velocity.0;
    let offset_a = a.position.1 - slope_a * a.position.0;
    let slope_b = b.velocity.1 / b.velocity.0;
    let offset_b = b.position.1 - slope_b * b.position.0;

    // y = m * x + b

    if slope_a == slope_b {
        // parallel
        return false;
    }

    let x = (offset_b - offset_a) / (slope_a - slope_b);

    if x > upper_bound || x < lower_bound {
        return false;
    }

    let y = slope_a * x + offset_a;

    if y > upper_bound || y < lower_bound {
        return false;
    }

    if (x - a.position.0) / a.velocity.0 < 0.0 {
        // paths crossed in past for a
        return false;
    }

    if (x - b.position.0) / b.velocity.0 < 0.0 {
        // paths crossed in past for b
        return false;
    }

    true
}

impl Problem for DayTwentyFour {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut particles = vec![];
        for (id, line) in contents.iter().enumerate() {
            let parts = line.split(" @ ").collect::<Vec<&str>>();
            let position = parts[0]
                .split(", ")
                .map(|s| s.parse::<f64>().unwrap())
                .collect_tuple::<(f64, f64, f64)>()
                .unwrap();
            let velocity = parts[1]
                .split(", ")
                .map(|s| s.parse::<f64>().unwrap())
                .collect_tuple::<(f64, f64, f64)>()
                .unwrap();
            let particle = Particle {
                id,
                position,
                velocity,
            };
            particles.push(particle);
        }

        let mut counter = 0;
        let lb = 200000000000000.0;
        let ub = 400000000000000.0;

        for this in particles.iter() {
            for other in particles.iter() {
                if this.id != other.id {
                    counter += check_intersection(this, other, lb, ub) as u32;
                }
            }
        }

        format!("{}", counter / 2)
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
