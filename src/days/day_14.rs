use itertools::Itertools;
use crate::io::read_file_lines;
use crate::problem::Problem;
use ndarray::{Array, Array2, s};

pub struct DayFourteen {}

impl Problem for DayFourteen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = Array2::zeros((contents[0].len(), contents.len()));

        for (y, line) in contents.iter().enumerate() {
            for (x, c) in line.char_indices() {
                match c {
                    '#' => map[[x, y]] = 1,
                    'O' => map[[x, y]] = 2,
                    _ => {}
                }
            }
        }

        // roll to north
        for mut row in map.rows_mut() {
            let mut cubes: Vec<isize> = row
                .iter()
                .enumerate()
                .filter(|&(_, &x)| x == 1)
                .map(|(i, _)| i as isize)
                .collect();
            cubes.push(contents.len() as isize);

            let mut cubes_total = vec![-1];
            cubes_total.extend(cubes);
            let cubes = cubes_total;

            for i in 0..cubes.len() - 1 {
                let slice_to_sort = &mut row.slice_mut(s![cubes[i] + 1..cubes[i + 1]]);
                let sorted_slice = slice_to_sort.iter().sorted().map(|i| *i).rev().collect_vec();
                slice_to_sort.assign(&Array::from_vec(sorted_slice));
            }
        }

        // sum all loads
        let mut summed_load = 0;
        let max_y = contents.len();
        for row in map.rows() {
            let rounds: Vec<usize> = row
                .iter()
                .enumerate()
                .filter(|&(_, &x)| x == 2)
                .map(|(i, _)| i)
                .collect();
            for round_rock in rounds {
                summed_load += max_y - round_rock
            }
        }

        format!("{}", summed_load)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = Array2::zeros((contents[0].len(), contents.len()));

        for (y, line) in contents.iter().enumerate() {
            for (x, c) in line.char_indices() {
                match c {
                    '#' => map[[x, y]] = 1,
                    'O' => map[[x, y]] = 2,
                    _ => {}
                }
            }
        }

        for _cycle in 0..1000000000 {
            if _cycle % 100000 == 0 {
                println!("cycle: {} %", 100.0 / 1000000000.0 * _cycle as f32);
                // sum all loads
                let mut summed_load = 0;
                let max_y = contents.len();
                for row in map.rows() {
                    let rounds: Vec<usize> = row
                        .iter()
                        .enumerate()
                        .filter(|&(_, &x)| x == 2)
                        .map(|(i, _)| i)
                        .collect();
                    for round_rock in rounds {
                        summed_load += max_y - round_rock
                    }
                }

                println!("loads: {summed_load}");
            }

            // roll to north
            for mut row in map.rows_mut() {
                let mut cubes: Vec<isize> = row
                    .iter()
                    .enumerate()
                    .filter(|&(_, &x)| x == 1)
                    .map(|(i, _)| i as isize)
                    .collect();
                cubes.push(contents.len() as isize);

                let mut cubes_total = vec![-1];
                cubes_total.extend(cubes);
                let cubes = cubes_total;

                for i in 0..cubes.len() - 1 {
                    let slice_to_sort = &mut row.slice_mut(s![cubes[i] + 1..cubes[i + 1]]);
                    let sorted_slice = slice_to_sort.iter().sorted().map(|i| *i).rev().collect_vec();
                    slice_to_sort.assign(&Array::from_vec(sorted_slice));
                }
            }


            // roll to west
            for mut col in map.columns_mut() {
                let mut cubes: Vec<isize> = col
                    .iter()
                    .enumerate()
                    .filter(|&(_, &x)| x == 1)
                    .map(|(i, _)| i as isize)
                    .collect();
                cubes.push(contents[0].len() as isize);

                let mut cubes_total = vec![-1];
                cubes_total.extend(cubes);
                let cubes = cubes_total;

                for i in 0..cubes.len() - 1 {
                    let slice_to_sort = &mut col.slice_mut(s![cubes[i] + 1..cubes[i + 1]]);
                    let sorted_slice = slice_to_sort.iter().sorted().map(|i| *i).rev().collect_vec();
                    slice_to_sort.assign(&Array::from_vec(sorted_slice));
                }
            }

            // roll to east
            for mut col in map.columns_mut() {
                let mut cubes: Vec<isize> = col
                    .iter()
                    .enumerate()
                    .filter(|&(_, &x)| x == 1)
                    .map(|(i, _)| i as isize)
                    .collect();
                cubes.push(contents[0].len() as isize);

                let mut cubes_total = vec![-1];
                cubes_total.extend(cubes);
                let cubes = cubes_total;

                for i in 0..cubes.len() - 1 {
                    let slice_to_sort = &mut col.slice_mut(s![cubes[i] + 1..cubes[i + 1]]);
                    let sorted_slice = slice_to_sort.iter().sorted().map(|i| *i).collect_vec();
                    slice_to_sort.assign(&Array::from_vec(sorted_slice));
                }
            }

            // roll to south
            for mut row in map.rows_mut() {
                let mut cubes: Vec<isize> = row
                    .iter()
                    .enumerate()
                    .filter(|&(_, &x)| x == 1)
                    .map(|(i, _)| i as isize)
                    .collect();
                cubes.push(contents.len() as isize);

                let mut cubes_total = vec![-1];
                cubes_total.extend(cubes);
                let cubes = cubes_total;

                for i in 0..cubes.len() - 1 {
                    let slice_to_sort = &mut row.slice_mut(s![cubes[i] + 1..cubes[i + 1]]);
                    let sorted_slice = slice_to_sort.iter().sorted().map(|i| *i).collect_vec();
                    slice_to_sort.assign(&Array::from_vec(sorted_slice));
                }
            }
        }

        // sum all loads
        let mut summed_load = 0;
        let max_y = contents.len();
        for row in map.rows() {
            let rounds: Vec<usize> = row
                .iter()
                .enumerate()
                .filter(|&(_, &x)| x == 2)
                .map(|(i, _)| i)
                .collect();
            for round_rock in rounds {
                summed_load += max_y - round_rock
            }
        }

        format!("{}", summed_load)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}