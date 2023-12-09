use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayNine {}


pub fn get_diff(vec: &Vec<i32>) -> Vec<i32> {
    let mut d = vec![];
    for i in 0..vec.len() - 1 {
        d.push(vec[i + 1] - vec[i])
    }
    d
}

impl Problem for DayNine {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut values = vec![];
        for line in contents {
            values.push(line.split(" ").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>());
        }

        let mut sum = 0;

        for history in values.iter_mut() {
            let mut diffs = vec![history.clone()];

            loop {
                let diff = get_diff(&diffs.last().unwrap());
                diffs.push(diff.clone());
                if diff.iter().filter(|&v| *v == 0 ).count() == diff.len() {
                    break;
                }
            }

            let i = diffs.len()-1;

            diffs[i].push(0);

            for i in (0..diffs.len()-1).rev() {
                let val = diffs[i+1].last().unwrap() + diffs[i].last().unwrap();
                diffs[i].push(val);
            }

            sum += diffs[0].last().unwrap();

        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut values = vec![];
        for line in contents {
            values.push(line.split(" ").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>());
        }

        let mut sum = 0;

        for history in values.iter_mut() {
            let mut diffs = vec![history.clone()];

            loop {
                let diff = get_diff(&diffs.last().unwrap());
                diffs.push(diff.clone());
                if diff.iter().filter(|&v| *v == 0 ).count() == diff.len() {
                    break;
                }
            }

            let i = diffs.len()-1;

            diffs[i].insert(0,0);

            for i in (0..diffs.len()-1).rev() {
                let val = - diffs[i+1].first().unwrap() + diffs[i].first().unwrap();
                diffs[i].insert(0, val);
            }

            sum += diffs[0].first().unwrap();

        }

        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}