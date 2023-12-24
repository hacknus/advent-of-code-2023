use crate::io::read_file_lines;
use crate::problem::Problem;
use std::cell::RefCell;
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
}

struct Node {
    dir: Direction,
    position: (isize, isize),
    terminated: bool,
    path: HashSet<(isize, isize)>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn propagate(
        &mut self,
        map: &Vec<Vec<String>>,
        end: (isize, isize), // states: &mut HashMap<(usize, usize), Direction>,
    ) {
        // if let Some(dir) = states.get(&self.position) {
        //     if self.dir == *dir {
        //         // already been here
        //         if energized_map[self.position.1][self.position.0] == 1 {
        //             self.terminated = true;
        //             return;
        //         }
        //     }
        // }

        // states.insert(self.position, self.dir);

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
            let mut next_position = self.position;
            let mut path = self.path.clone();
            next_position.0 += dir.as_coordinates().0;
            next_position.1 += dir.as_coordinates().1;
            path.insert(next_position);
            if next_position.0 < 0
                || next_position.1 < 0
                || next_position.0 >= map[0].len() as isize
                || next_position.1 >= map.len() as isize
                || dir == self.dir.opposite()
                || self.path.contains(&next_position)
            {
                continue;
            }

            if map[next_position.1 as usize][next_position.0 as usize].as_str() == "." {
                self.children.push(Rc::new(RefCell::new(Node {
                    dir,
                    position: next_position,
                    terminated: false,
                    path,
                    children: vec![],
                })))
            } else if map[next_position.1 as usize][next_position.0 as usize].as_str() == ">"
                && dir == Direction::Right
            {
                self.children.push(Rc::new(RefCell::new(Node {
                    dir,
                    position: next_position,
                    terminated: false,
                    path,
                    children: vec![],
                })))
            } else if map[next_position.1 as usize][next_position.0 as usize].as_str() == "<"
                && dir == Direction::Left
            {
                self.children.push(Rc::new(RefCell::new(Node {
                    dir,
                    position: next_position,
                    terminated: false,
                    path,
                    children: vec![],
                })))
            } else if map[next_position.1 as usize][next_position.0 as usize].as_str() == "^"
                && dir == Direction::Up
            {
                self.children.push(Rc::new(RefCell::new(Node {
                    dir,
                    position: next_position,
                    terminated: false,
                    path,
                    children: vec![],
                })))
            } else if map[next_position.1 as usize][next_position.0 as usize].as_str() == "v"
                && dir == Direction::Down
            {
                self.children.push(Rc::new(RefCell::new(Node {
                    dir,
                    position: next_position,
                    terminated: false,
                    path,
                    children: vec![],
                })))
            }
        }

        for child in self.children.iter_mut() {
            child.borrow_mut().propagate(map, end);
        }
    }

    fn walk(&mut self, mut count: u32) -> u32 {
        if self.terminated {
            return self.path.len() as u32;
        }
        for child in self.children.iter_mut() {
            count = count.max(child.borrow_mut().walk(count));
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
            path: HashSet::new(),
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
            path: HashSet::new(),
            children: vec![],
        };

        let end = (contents[0].len() as isize - 2, contents.len() as isize - 1);

        root.propagate(&map, end);

        let count = root.walk(0);

        format!("{}", count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
