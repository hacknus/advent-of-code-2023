use std::collections::HashMap;
use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayEleven {}

impl Problem for DayEleven {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = contents.iter().map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();

        // search empty column
        let mut col_inds = vec![];
        for i in 0..map[0].len() {
            let mut counter = 0;
            for line in map.iter() {
                if line[i] == "." {
                    counter += 1;
                }
            }
            if counter == map.len() {
                col_inds.push(i);
            }
        }
        for i in col_inds.iter().rev() {
            for line in map.iter_mut() {
                line.insert(*i, ".".to_string());
            }
        }

        // search for row
        let row_inds = map.iter().enumerate().filter_map(|(i, &ref v)| if *v == vec![".".to_string(); map[0].len()] { Some(i) } else { None }).collect::<Vec<usize>>();
        for i in row_inds.iter().rev() {
            map.insert(*i, vec![".".to_string(); map[0].len()]);
        }

        // space is expanded now
        let mut galaxies = vec![];
        for (y, line) in map.iter().enumerate() {
            for (x, field) in line.iter().enumerate() {
                if *field == "#".to_string() {
                    galaxies.push((x, y));
                }
            }
        }

        let mut distances: HashMap<String, isize> = HashMap::new();
        for i in 0..galaxies.len() {
            for j in 0..galaxies.len() {
                if i != j {
                    let d = (galaxies[i].0 as isize - galaxies[j].0 as isize).abs() + (galaxies[i].1 as isize - galaxies[j].1 as isize).abs();
                    let smaller = i.min(j) + 1;
                    let bigger = i.max(j) + 1;
                    distances.insert(format!("{smaller}-{bigger}").to_string(), d);
                }
            }
        }

        format!("{}", distances.values().sum::<isize>())
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = contents.iter().map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();

        // search empty column
        let mut col_inds = vec![];
        for i in 0..map[0].len() {
            let mut counter = 0;
            for line in map.iter() {
                if line[i] == "." {
                    counter += 1;
                }
            }
            if counter == map.len() {
                col_inds.push(i);
            }
        }

        // search for row
        let row_inds = map.iter().enumerate().filter_map(|(i, &ref v)| if *v == vec![".".to_string(); map[0].len()] { Some(i) } else { None }).collect::<Vec<usize>>();

        // space is expanded now
        let mut galaxies = vec![];
        for (y, line) in map.iter().enumerate() {
            for (x, field) in line.iter().enumerate() {
                if *field == "#".to_string() {
                    galaxies.push((x, y));
                }
            }
        }

        let scaling_factor = 1000000;

        let mut distances: HashMap<String, isize> = HashMap::new();
        for i in 0..galaxies.len() {
            for j in 0..galaxies.len() {
                if i != j {
                    let mut d = (galaxies[i].0 as isize - galaxies[j].0 as isize).abs() + (galaxies[i].1 as isize - galaxies[j].1 as isize).abs();
                    let smaller = i.min(j) + 1;
                    let bigger = i.max(j) + 1;
                    for row in row_inds.iter() {
                        if (galaxies[i].1 < *row && galaxies[j].1 > *row) || (galaxies[j].1 < *row && galaxies[i].1 > *row) {
                            d += scaling_factor - 1;
                        }
                    }
                    for col in col_inds.iter() {
                        if (galaxies[i].0 < *col && galaxies[j].0 > *col) || (galaxies[j].0 < *col && galaxies[i].0 > *col) {
                            d += scaling_factor - 1;
                        }
                    }
                    distances.insert(format!("{smaller}-{bigger}").to_string(), d);
                }
            }
        }

        format!("{}", distances.values().sum::<isize>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}