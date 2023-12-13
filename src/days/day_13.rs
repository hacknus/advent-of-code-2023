use std::collections::HashMap;
use std::time::Instant;
use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayThirteen {}


fn has_one_difference(s1: &str, s2: &str) -> bool {
    let mut found_one_difference = false;

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            if found_one_difference {
                return false;
            } else {
                found_one_difference = true
            }
        }
    }

    found_one_difference
}

impl Problem for DayThirteen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut patterns = vec![];
        let mut temp_pattern = vec![];
        for line in contents {
            if line != "" {
                temp_pattern.push(line);
            } else {
                patterns.push(temp_pattern);
                temp_pattern = vec![];
            }
        }
        patterns.push(temp_pattern);

        let mut sum = 0;

        for (n_pattern, pattern) in patterns.iter().enumerate() {
            // check vertical
            let mut vertical_mirror = None;
            let mut equals = vec![];
            for i in 0..pattern.len() {
                for j in (i + 1..pattern.len()).rev() {
                    if pattern[i] == pattern[j] {
                        equals.push((i, j));
                    }
                }
            }

            let mut pairs = vec![];
            for (i, equal) in equals.iter().enumerate() {
                if equal.1 - equal.0 == 1 {
                    pairs.push(i);
                    // horizontal_mirror = Some(equal.0 + 1);
                }
            }

            'outer: for pair in pairs {
                let mut other = equals[pair];
                loop {
                    if !equals.contains(&other) {
                        break;
                    }
                    if other.0 > 0 && other.1 < pattern.len() - 1 {
                        other.0 -= 1;
                        other.1 += 1;
                    } else {
                        vertical_mirror = Some(equals[pair].0 + 1);
                        break 'outer;
                    }
                }
            }

            if let Some(mirror) = vertical_mirror {
                sum += mirror * 100;
            }

            // check horizontal
            let mut horizontal_mirror = None;
            let mut equals = vec![];
            for i in 0..pattern[0].chars().count() {
                for j in (i + 1..pattern[0].chars().count()).rev() {
                    let mut row_counter = 0;
                    for row in pattern.iter() {
                        if row.chars().nth(i).unwrap() == row.chars().nth(j).unwrap() {
                            row_counter += 1;
                        }
                    }
                    if row_counter == pattern.len() {
                        equals.push((i, j))
                    }
                }
            }

            let mut pairs = vec![];
            for (i, equal) in equals.iter().enumerate() {
                if equal.1 - equal.0 == 1 {
                    pairs.push(i);
                    // horizontal_mirror = Some(equal.0 + 1);
                }
            }

            'outer: for pair in pairs {
                let mut other = equals[pair];
                loop {
                    if !equals.contains(&other) {
                        break;
                    }
                    if other.0 > 0 && other.1 < pattern[0].chars().count() - 1 {
                        other.0 -= 1;
                        other.1 += 1;
                    } else {
                        horizontal_mirror = Some(equals[pair].0 + 1);
                        break 'outer;
                    }
                }
            }

            if let Some(mirror) = horizontal_mirror {
                sum += mirror;
            }
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut patterns = vec![];
        let mut temp_pattern = vec![];
        for line in contents {
            if line != "" {
                temp_pattern.push(line);
            } else {
                patterns.push(temp_pattern);
                temp_pattern = vec![];
            }
        }
        patterns.push(temp_pattern);

        let mut sum = 0;

        let mut cache = HashMap::new();


        for (n_pattern, pattern) in patterns.iter().enumerate() {
            // check vertical
            let mut vertical_mirror = None;
            let mut equals = vec![];
            for i in 0..pattern.len() {
                for j in (i + 1..pattern.len()).rev() {
                    if pattern[i] == pattern[j] {
                        equals.push((i, j));
                    }
                }
            }

            let mut pairs = vec![];
            for (i, equal) in equals.iter().enumerate() {
                if equal.1 - equal.0 == 1 {
                    pairs.push(i);
                }
            }

            'outer: for pair in pairs {
                let mut other = equals[pair];
                loop {
                    if !equals.contains(&other) {
                        break;
                    }
                    if other.0 > 0 && other.1 < pattern.len() - 1 {
                        other.0 -= 1;
                        other.1 += 1;
                    } else {
                        vertical_mirror = Some(equals[pair].0 + 1);
                        break 'outer;
                    }
                }
            }

            if let Some(mirror) = vertical_mirror {
                cache.insert(n_pattern, (-1, mirror as isize));
            }

            // check horizontal
            let mut horizontal_mirror = None;
            let mut equals = vec![];
            for i in 0..pattern[0].chars().count() {
                for j in (i + 1..pattern[0].chars().count()).rev() {
                    let mut row_counter = 0;
                    for row in pattern.iter() {
                        if row.chars().nth(i).unwrap() == row.chars().nth(j).unwrap() {
                            row_counter += 1;
                        }
                    }
                    if row_counter == pattern.len() {
                        equals.push((i, j))
                    }
                }
            }

            let mut pairs = vec![];
            for (i, equal) in equals.iter().enumerate() {
                if equal.1 - equal.0 == 1 {
                    pairs.push(i);
                }
            }

            'outer: for pair in pairs {
                let mut other = equals[pair];
                loop {
                    if !equals.contains(&other) {
                        break;
                    }
                    if other.0 > 0 && other.1 < pattern[0].chars().count() - 1 {
                        other.0 -= 1;
                        other.1 += 1;
                    } else {
                        horizontal_mirror = Some(equals[pair].0 + 1);
                        break 'outer;
                    }
                }
            }

            if let Some(mirror) = horizontal_mirror {
                cache.insert(n_pattern, (mirror as isize, -1));
            }
        }


        // cache build complete, now do the rest


        for (n_pattern, pattern) in patterns.iter().enumerate() {

            // check vertical
            let mut vertical_mirror = None;
            let mut flagged_equals = vec![];
            let mut equals = vec![];
            for i in 0..pattern.len() {
                for j in (i + 1..pattern.len()).rev() {
                    if pattern[i] == pattern[j] {
                        equals.push((i, j));
                        flagged_equals.push(0);
                    }
                    if has_one_difference(&pattern[i], &pattern[j]) {
                        equals.push((i, j));
                        flagged_equals.push(1);
                    }
                }
            }

            let mut pairs = vec![];
            for (i, equal) in equals.iter().enumerate() {
                if equal.1 - equal.0 == 1 {
                    pairs.push(i);
                }
            }

            'outer: for pair in pairs {
                let mut other = equals[pair];
                let mut number_of_flagged_equals = 0;
                loop {
                    if !equals.contains(&other) {
                        break;
                    } else {
                        number_of_flagged_equals += flagged_equals[equals.iter().position(|&r| r == other).unwrap()];
                    }
                    if other.0 > 0 && other.1 < pattern.len() - 1 {
                        other.0 -= 1;
                        other.1 += 1;
                    } else {
                        if number_of_flagged_equals > 1 {
                            break;
                        }
                        if cache.get_key_value(&n_pattern) == Some((&n_pattern, &(-1, equals[pair].0 as isize + 1))) {
                            break;
                        }
                        vertical_mirror = Some(equals[pair].0 + 1);
                        break 'outer;
                    }
                }
            }

            if let Some(mirror) = vertical_mirror {
                sum += mirror * 100;
            }

            // check horizontal
            let mut horizontal_mirror = None;
            let mut equals = vec![];
            let mut flagged_equals = vec![];
            for i in 0..pattern[0].chars().count() {
                for j in (i + 1..pattern[0].chars().count()).rev() {
                    let mut row_counter = 0;
                    for row in pattern.iter() {
                        if row.chars().nth(i).unwrap() == row.chars().nth(j).unwrap() {
                            row_counter += 1;
                        }
                    }
                    if row_counter == pattern.len() {
                        equals.push((i, j));
                        flagged_equals.push(0);
                    }
                    if row_counter == pattern.len() - 1 {
                        equals.push((i, j));
                        flagged_equals.push(1);
                    }
                }
            }

            let mut pairs = vec![];
            for (i, equal) in equals.iter().enumerate() {
                if equal.1 - equal.0 == 1 {
                    pairs.push(i);
                }
            }

            'outer: for pair in pairs {
                let mut other = equals[pair];
                let mut number_of_flagged_equals = 0;
                loop {
                    if !equals.contains(&other) {
                        break;
                    } else {
                        number_of_flagged_equals += flagged_equals[equals.iter().position(|&r| r == other).unwrap()];
                    }
                    if other.0 > 0 && other.1 < pattern[0].chars().count() - 1 {
                        other.0 -= 1;
                        other.1 += 1;
                    } else {
                        if number_of_flagged_equals > 1 {
                            break;
                        }
                        if cache.get_key_value(&n_pattern) == Some((&n_pattern, &(equals[pair].0 as isize + 1, -1))) {
                            break;
                        }
                        horizontal_mirror = Some(equals[pair].0 + 1);
                        break 'outer;
                    }
                }
            }

            if let Some(mirror) = horizontal_mirror {
                sum += mirror;
            }
        }

        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}