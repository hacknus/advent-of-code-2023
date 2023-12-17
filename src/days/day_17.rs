use crate::io::read_file_lines;
use crate::problem::Problem;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct DaySeventeen {}

#[derive(Clone, Debug)]
struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    position: (isize, isize),
    length: isize,
    g: i32,
    h: i32,
    f: i32,
}

pub fn a_star(
    map: &Vec<Vec<i32>>,
    start: (isize, isize),
    end: (isize, isize),
) -> Option<Vec<(isize, isize)>> {
    // create start and end node
    let start_node = Node {
        parent: None,
        position: start,
        length: 0,
        g: 0,
        h: 0,
        f: 0,
    };
    let end_node = Node {
        parent: None,
        position: end,
        length: 0,
        g: 0,
        h: 0,
        f: 0,
    };

    // initialize both open and closed list
    let mut open_list = vec![];
    let mut closed_list = vec![];

    open_list.push(start_node);

    // loop until we find the end
    while open_list.len() > 0 {
        let mut current_node = open_list[0].clone();
        let mut current_index = 0;

        for (index, item) in open_list.iter().enumerate() {
            if item.f < current_node.f {
                current_node = item.clone();
                current_index = index
            }
        }

        // Pop current off open list, add to closed list
        open_list.remove(current_index);
        closed_list.push(current_node.clone());

        // Found the end
        if current_node.position == end_node.position {
            let mut path = vec![current_node.position];
            let mut current = current_node.clone();
            while let Some(parent) = current.parent {
                path.push(parent.borrow().position);
                current = parent.borrow().clone();
            }
            return Some(path);
        }

        // generate children

        let mut children = vec![];
        for new_position in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
            // Get node position
            let node_position = (
                current_node.position.0 + new_position.0,
                current_node.position.1 + new_position.1,
            );

            // Make sure within range
            if node_position.0 > map[0].len() as isize - 1
                || node_position.0 < 0
                || node_position.1 > map.len() as isize - 1
                || node_position.1 < 0
            {
                continue;
            }

            // Make sure walkable terrain
            if map[node_position.1 as usize][node_position.0 as usize] != 0 {
                continue;
            }

            // Create new node
            let new_node = Node {
                parent: Some(Rc::new(RefCell::new(current_node.clone()))),
                position: node_position,
                length: 0,
                g: 0,
                h: 0,
                f: 0,
            };

            // Append
            children.push(new_node);
        }
        // Loop through children
        'outer: for mut child in children.iter_mut() {
            // Child is on the closed list
            for closed_child in closed_list.iter() {
                if child.position == closed_child.position {
                    continue 'outer;
                }
            }

            // Create the f, g, and h values
            // child.g = current_node.g
            //     + map[current_node.position.0 as usize][current_node.position.1 as usize];
            child.g = current_node.g + 1;
            child.h = ((child.position.0 as i32 - end_node.position.0 as i32).pow(2))
                + ((child.position.1 as i32 - end_node.position.1 as i32).pow(2));
            child.f = child.g + child.h;

            // Child is already in the open list
            for open_node in open_list.iter() {
                if child.position == open_node.position && child.g > open_node.g {
                    continue 'outer;
                }
            }

            // Add the child to the open list
            open_list.push(child.clone());
        }
    }
    None
}

impl Problem for DaySeventeen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = vec![vec![0; contents[0].len()]; contents.len()];
        for (y, line) in contents.iter().enumerate() {
            for (x, c) in line.char_indices() {
                map[y][x] = c.to_digit(10).unwrap() as i32;
            }
        }

        let start = (0, 0);
        let end = (map[0].len() as isize - 1, map.len() as isize - 1);

        let map = vec![
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let end = (7, 6);

        let path = a_star(&map, start, end).unwrap();

        dbg!(&path);
        for (y, line) in map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if path.contains(&(x as isize, y as isize)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
        format!("{}", "Part one not yet implemented.")
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
