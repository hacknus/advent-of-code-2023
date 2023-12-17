use crate::io::read_file_lines;
use crate::problem::Problem;
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};
pub struct DaySeventeen {}
fn dijkstra(
    start: Vertex,
    initial_direction: Direction,
    adjacency_list: &HashMap<Vertex, Vec<(Vertex, i32, Direction)>>,
) -> HashMap<Vertex, (i32, Direction)> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert(start, (0, initial_direction.clone()));
    to_visit.push(Visit {
        vertex: start,
        directions: vec![initial_direction.clone()],
        distance: 0,
    });

    while let Some(Visit {
        vertex, distance, ..
    }) = to_visit.pop()
    {
        if !visited.insert(vertex) {
            // Already visited this node
            continue;
        }

        if let Some(neighbors) = adjacency_list.get(&vertex) {
            for (neighbor, cost, direction) in neighbors {
                let new_distance = distance as i32 + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |&(current, _)| new_distance < current);

                let mut directions = if let Some(v) = to_visit.iter().last() {
                    v.directions.clone()
                } else {
                    vec![]
                };
                directions.push(direction.clone());
                let n = directions.len();
                let three_consecutive_moves = if n > 3 {
                    directions[n - 1] == directions[n - 2]
                        && directions[n - 1] == directions[n - 3]
                        && directions[n - 1] == directions[n - 4]
                } else {
                    false
                };

                if is_shorter {
                    if three_consecutive_moves {
                        distances.insert(*neighbor, (new_distance, direction.clone()));
                        to_visit.push(Visit {
                            vertex: *neighbor,
                            directions,
                            distance: usize::MAX,
                        });
                    } else {
                        distances.insert(*neighbor, (new_distance, direction.clone()));
                        to_visit.push(Visit {
                            vertex: *neighbor,
                            directions,
                            distance: new_distance as usize,
                        });
                    }
                }
            }
        }
    }

    distances
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_coordinates(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Direction::Up => "^".to_string(),
            Direction::Down => "v".to_string(),
            Direction::Left => "<".to_string(),
            Direction::Right => ">".to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vertex {
    x: usize,
    y: usize,
    heat_loss: u32,
}

impl Vertex {
    fn new(x: usize, y: usize, heat_loss: u32) -> Vertex {
        Vertex { x, y, heat_loss }
    }
}

#[derive(Debug)]
struct Visit<V> {
    vertex: V,
    directions: Vec<Direction>,
    distance: usize,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}

impl Problem for DaySeventeen {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = vec![vec![0; contents[0].len()]; contents.len()];
        let mut vertices = HashMap::new();
        for (y, line) in contents.iter().enumerate() {
            for (x, c) in line.char_indices() {
                map[y][x] = c.to_digit(10).unwrap() as i32;
                vertices.insert(
                    (x as isize, y as isize),
                    Vertex::new(x, y, c.to_digit(10).unwrap()),
                );
            }
        }

        let mut adjacency_list = HashMap::new();
        for position in vertices.keys() {
            let mut neighbours = vec![];

            let direction = Direction::Left.as_coordinates();
            let mut neighbour_position = position.clone();
            neighbour_position.0 += direction.0;
            if neighbour_position.0 >= 0 {
                neighbours.push((
                    vertices[&neighbour_position],
                    map[neighbour_position.1 as usize][neighbour_position.0 as usize],
                    Direction::Left,
                ))
            }

            let direction = Direction::Right.as_coordinates();
            let mut neighbour_position = position.clone();
            neighbour_position.0 += direction.0;
            if neighbour_position.0 < map[0].len() as isize {
                neighbours.push((
                    vertices[&neighbour_position],
                    map[neighbour_position.1 as usize][neighbour_position.0 as usize],
                    Direction::Right,
                ))
            }

            let direction = Direction::Up.as_coordinates();
            let mut neighbour_position = position.clone();
            neighbour_position.1 += direction.1;
            if neighbour_position.1 >= 0 {
                neighbours.push((
                    vertices[&neighbour_position],
                    map[neighbour_position.1 as usize][neighbour_position.0 as usize],
                    Direction::Up,
                ))
            }

            let direction = Direction::Down.as_coordinates();
            let mut neighbour_position = position.clone();
            neighbour_position.1 += direction.1;
            if neighbour_position.1 < map.len() as isize {
                neighbours.push((
                    vertices[&neighbour_position],
                    map[neighbour_position.1 as usize][neighbour_position.0 as usize],
                    Direction::Down,
                ))
            }

            adjacency_list.insert(vertices[position], neighbours);
        }

        let distances = dijkstra(vertices[&(1, 0)], Direction::Right, &adjacency_list);

        let mut heat_loss = 0;
        for (v, (d, _)) in &distances {
            if v.x == map[0].len() - 1 && v.y == map.len() - 1 {
                println!("name: ({},{}), distance: {}", v.x, v.y, d);
                heat_loss = *d;
            }
        }

        for (y, line) in map.iter().enumerate() {
            for (x, local_heat_loss) in line.iter().enumerate() {
                let mut b = false;
                for (v, (d, dir)) in distances.iter() {
                    if x == v.x && y == v.y {
                        b = true;
                        print!("{}", dir.to_string());
                    }
                }
                if !b {
                    print!("{local_heat_loss}");
                }
            }
            println!();
        }

        format!("{}", heat_loss)
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
