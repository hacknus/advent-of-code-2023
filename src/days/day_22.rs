use crate::io::read_file_lines;
use crate::problem::Problem;
use itertools::Itertools;
use std::collections::HashSet;

pub struct DayTwentyTwo {}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Object {
    id: usize,
    p0: (u32, u32, u32),
    p1: (u32, u32, u32),
    cubes: Vec<(u32, u32, u32)>,
}

impl Problem for DayTwentyTwo {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut objects = vec![];
        for (id, line) in contents.iter().enumerate() {
            let positions = line.split("~").collect::<Vec<&str>>();
            let p0 = positions[0]
                .split(",")
                .map(|p| p.parse::<u32>().unwrap())
                .collect_tuple::<(u32, u32, u32)>()
                .unwrap();
            let p1 = positions[1]
                .split(",")
                .map(|p| p.parse::<u32>().unwrap())
                .collect_tuple::<(u32, u32, u32)>()
                .unwrap();
            let mut cubes = vec![];
            assert!(p0.0 <= p1.0);
            assert!(p0.1 <= p1.1);
            assert!(p0.2 <= p1.2);
            for x in p0.0..=p1.0 {
                for y in p0.1..=p1.1 {
                    for z in p0.2..=p1.2 {
                        cubes.push((x, y, z));
                    }
                }
            }
            let object = Object { id, p0, p1, cubes };
            objects.push(object);
        }

        // now sort the entries such that the lowest are first
        objects.sort_by_cached_key(|o| o.p0.2);

        let mut fallen_objects: Vec<Object> = vec![];

        for mut object in objects {
            'occupied_loop: loop {
                let mut occupied = false;
                'outer: for other in fallen_objects.iter() {
                    // check lower bound
                    if object.p0.2 > other.p1.2 {
                        // we are above this one
                        continue 'outer;
                    } else {
                        // check other cubes, if any are occupied
                        for cube in object.cubes.iter() {
                            if other.cubes.contains(&cube) {
                                occupied = true;
                                break 'outer;
                            }
                        }
                    }
                }
                if occupied || object.p0.2 == 0 {
                    // this one is occupied, let's move back up and exit the loop
                    object.p0.2 += 1;
                    object.p1.2 += 1;
                    for cube in object.cubes.iter_mut() {
                        cube.2 += 1;
                    }
                    break 'occupied_loop;
                }
                // fall one step further down
                object.p0.2 -= 1;
                object.p1.2 -= 1;
                for cube in object.cubes.iter_mut() {
                    cube.2 -= 1;
                }
            }

            fallen_objects.push(object);
        }

        let mut removable = HashSet::new();
        let mut excludes = HashSet::new();

        for this in fallen_objects.iter() {
            let mut supported_by = HashSet::new();
            let mut supporting = HashSet::new();

            // generate all cubes below
            let mut cubes_below = this.cubes.clone();
            for cube in cubes_below.iter_mut() {
                cube.2 -= 1;
            }

            // generate all cubes above
            let mut cubes_above = this.cubes.clone();
            for cube in cubes_above.iter_mut() {
                cube.2 += 1;
            }
            for other in fallen_objects.iter() {
                if this.id != other.id {
                    for cube in cubes_below.iter() {
                        // if another brick contains any of the cubes below
                        if other.cubes.contains(&cube) {
                            supported_by.insert(other);
                            break;
                        }
                    }

                    for cube in cubes_above.iter() {
                        // if another brick contains any of the cubes above
                        if other.cubes.contains(&cube) {
                            supporting.insert(other);
                            break;
                        }
                    }
                }
            }
            // if the brick is supported by more than 1 brick
            if supported_by.len() > 1 {
                for s in supported_by.iter() {
                    // add all of the supporting bricks to the removables set
                    removable.insert(s.id);
                }
            } else {
                if let Some(s) = supported_by.iter().next() {
                    excludes.insert(s.id);
                }
            }

            if supporting.len() == 0 {
                // if there is no brick on top, we add this one to the removables
                removable.insert(this.id);
            }

            if supported_by.len() == 0 && supporting.len() > 0 {
                assert_eq!(this.p0.2, 1);
            }
        }
        for ex in excludes.iter() {
            if removable.contains(ex) {
                removable.remove(ex);
            }
        }

        format!("{}", removable.len())
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
