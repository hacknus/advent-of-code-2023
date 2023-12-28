use crate::io::read_file_lines;
use crate::problem::Problem;
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub struct DayTwentyThree {}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            ">" => Self::Right,
            "<" => Self::Left,
            "v" => Self::Down,
            "^" => Self::Up,
            _ => Self::Unknown,
        }
    }

    fn as_coordinates(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            _ => (0, 0),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Self::Down,
            Direction::Down => Self::Up,
            Direction::Left => Self::Right,
            Direction::Right => Self::Left,
            Direction::Unknown => Self::Unknown,
        }
    }

    fn perpendicular(&self) -> [Self; 2] {
        match self {
            Direction::Up => [Self::Right, Self::Left],
            Direction::Down => [Self::Right, Self::Left],
            Direction::Left => [Self::Up, Self::Down],
            Direction::Right => [Self::Up, Self::Down],
            Direction::Unknown => [Self::Unknown, Self::Unknown],
        }
    }
}

struct Node {
    dir: Direction,
    position: (isize, isize),
    terminated: bool,
    length: u32,
    path: Vec<(isize, isize)>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn propagate(&mut self, map: &Vec<Vec<String>>, end: (isize, isize)) {
        if self.position == end {
            self.terminated = true;
            return;
        }

        for dir in [
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::Up,
        ] {
            if dir == self.dir.opposite() {
                continue;
            }
            let mut next_position = self.position;
            let mut child = None;
            let mut counter = 0;
            loop {
                next_position.0 += dir.as_coordinates().0;
                next_position.1 += dir.as_coordinates().1;
                counter += 1;
                if next_position.0 < 0
                    || next_position.1 < 0
                    || next_position.0 >= map[0].len() as isize
                    || next_position.1 >= map.len() as isize
                    || self.path.contains(&next_position)
                {
                    break;
                }

                // check intersection
                let mut look_out = next_position;
                let mut wall_counter = 0;
                for dir_2 in dir.perpendicular() {
                    look_out.0 += dir_2.as_coordinates().0;
                    look_out.1 += dir_2.as_coordinates().1;
                    if look_out.0 < 0
                        || look_out.1 < 0
                        || look_out.0 >= map[0].len() as isize
                        || look_out.1 >= map.len() as isize
                    {
                        continue;
                    }
                    if map[look_out.1 as usize][look_out.0 as usize].as_str() == "#" {
                        wall_counter += 1;
                    }
                }

                if map[next_position.1 as usize][next_position.0 as usize].as_str() == "."
                    || (map[next_position.1 as usize][next_position.0 as usize].as_str() == ">"
                        && dir == Direction::Right)
                    || (map[next_position.1 as usize][next_position.0 as usize].as_str() == "<"
                        && dir == Direction::Left)
                    || (map[next_position.1 as usize][next_position.0 as usize].as_str() == "^"
                        && dir == Direction::Up)
                    || (map[next_position.1 as usize][next_position.0 as usize].as_str() == "v"
                        && dir == Direction::Down)
                {
                    child = Some(Rc::new(RefCell::new(Node {
                        dir,
                        position: next_position,
                        terminated: false,
                        length: self.length + counter,
                        path: vec![],
                        children: vec![],
                    })));
                } else {
                    // cannot go straight on any further
                    break;
                }
                if wall_counter < 2 {
                    break;
                }
            }
            if let Some(c) = child {
                self.children.push(c);
            }
        }

        let intersection = if self.children.len() > 1 { true } else { false };

        for child in self.children.iter_mut() {
            let mut path = self.path.clone();
            if intersection {
                path.push(self.position)
            }
            child.borrow_mut().path = path;
            child.borrow_mut().propagate(map, end);
        }
    }

    fn walk(&mut self, mut count: u32) -> u32 {
        if self.terminated {
            return self.length;
        }
        for child in self.children.iter_mut() {
            count = count.max(child.borrow_mut().walk(count));
        }
        count
    }

    fn walk_and_extract(&mut self, mut count: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
        if self.terminated {
            return self.path.clone();
        }
        for child in self.children.iter_mut() {
            let child_count = child.borrow_mut().walk_and_extract(count.clone());
            if count.len() < child_count.len() {
                count = child_count
            }
        }
        count
    }
}

impl Problem for DayTwentyThree {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = vec![vec![".".to_string(); contents[0].len()]; contents.len()];
        for (y, line) in contents.iter().enumerate() {
            for (x, field) in line.char_indices() {
                map[y][x] = field.to_string();
            }
        }

        let mut root = Node {
            dir: Direction::Down,
            position: (1, 0),
            terminated: false,
            length: 0,
            path: vec![],
            children: vec![],
        };

        let end = (contents[0].len() as isize - 2, contents.len() as isize - 1);

        root.propagate(&map, end);

        let count = root.walk(0);

        format!("{}", count)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = vec![vec![".".to_string(); contents[0].len()]; contents.len()];
        for (y, line) in contents.iter().enumerate() {
            for (x, field) in line.char_indices() {
                let field = match field {
                    '#' => '#',
                    _ => '.',
                };
                map[y][x] = field.to_string();
            }
        }

        let mut root = Node {
            dir: Direction::Down,
            position: (1, 0),
            terminated: false,
            length: 0,
            path: vec![],
            children: vec![],
        };

        let end = (contents[0].len() as isize - 2, contents.len() as isize - 1);

        root.propagate(&map, end);

        println!("propagated");

        let count = root.walk(0);
        let count_walk = root.walk_and_extract(vec![]);
        dbg!(&count_walk);

        for (y, line) in contents.iter().enumerate() {
            for (x, field) in line.char_indices() {
                let field = match field {
                    '#' => '#',
                    _ => '.',
                };
                if count_walk.contains(&(x as isize, y as isize)) {
                    print!("0");
                } else {
                    print!("{}", field);
                }
            }
            println!("");
        }

        format!("{}", count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
