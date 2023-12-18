use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayEighteen {}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "R" => Self::Right,
            "L" => Self::Left,
            "D" => Self::Down,
            _ => Self::Up,
        }
    }

    fn from_char(s: &char) -> Self {
        match s {
            '0' => Self::Right,
            '2' => Self::Left,
            '1' => Self::Down,
            _ => Self::Up,
        }
    }

    fn as_coordinates(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}
impl Problem for DayEighteen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut position = (0, 0);
        let mut position_2 = (0, 0);
        let mut positions = vec![position];
        let mut positions_2 = vec![position_2];

        for line in contents.iter() {
            let parts = line
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let direction = Direction::from_str(&parts[0]);
            let amount = parts[1].parse::<isize>().unwrap();
            let _color = parts[2].replace("(", "").replace(")", "");
            for _ in 0..amount {
                position.0 += direction.as_coordinates().0;
                position.1 += direction.as_coordinates().1;
                positions.push(position);
            }
            position_2.0 += direction.as_coordinates().0 * amount;
            position_2.1 += direction.as_coordinates().1 * amount;
            positions_2.push(position_2);
        }
        let mut area = 0;

        for i in 0..(positions_2.len() - 1) {
            area += (positions_2[i + 1].0 + positions_2[i].0)
                * (positions_2[i + 1].1 - positions_2[i].1);
        }

        let straight_pieces = positions.len() - positions_2.len();
        let corners = positions_2.len() - 1;

        area /= 2;

        area = area.abs();

        area += straight_pieces as isize / 2;

        let corners = (corners - 4) / 2 + 3;

        area += corners as isize;

        format!("{}", area.abs())
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut position = (0, 0);
        let mut position_2 = (0, 0);
        let mut positions = vec![position];
        let mut positions_2 = vec![position_2];
        for line in contents.iter() {
            let parts = line
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let _direction = Direction::from_str(&parts[0]);
            let _amount = parts[1].parse::<isize>().unwrap();
            let color = parts[2].replace("(", "").replace(")", "");
            let direction = Direction::from_char(&color.chars().last().unwrap());
            let amount = color.chars().skip(1).take(5).collect::<String>();
            let amount = isize::from_str_radix(&amount, 16).unwrap();
            for _ in 0..amount {
                position.0 += direction.as_coordinates().0;
                position.1 += direction.as_coordinates().1;
                positions.push(position);
            }
            position_2.0 += direction.as_coordinates().0 * amount;
            position_2.1 += direction.as_coordinates().1 * amount;
            positions_2.push(position_2);
        }
        let mut area = 0;

        for i in 0..(positions_2.len() - 1) {
            area += (positions_2[i + 1].0 + positions_2[i].0)
                * (positions_2[i + 1].1 - positions_2[i].1);
        }

        let straight_pieces = positions.len() - positions_2.len();
        let corners = positions_2.len() - 1;

        area /= 2;

        area = area.abs();

        area += straight_pieces as isize / 2;

        let corners = (corners - 4) / 2 + 3;

        area += corners as isize;

        format!("{}", area.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
