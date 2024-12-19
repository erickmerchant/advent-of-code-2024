use std::collections::{HashMap, HashSet};

type Coords = (usize, usize);

#[derive(Debug, Clone, Copy)]
struct Cell {
    cost: Option<usize>,
    visited: bool,
}

fn get_answer(input: Vec<String>, max_xy: usize, byte_count: usize) -> usize {
    let mut cells = HashMap::<Coords, Cell>::new();
    let mut corrupted = HashSet::<Coords>::new();

    for line in input[0..byte_count].iter() {
        let parts: Vec<&str> = line.split(",").collect();
        let x = parts[0].parse::<_>().unwrap();
        let y = parts[1].parse::<_>().unwrap();
        corrupted.insert((x, y));
    }

    for y in 0..=max_xy {
        for x in 0..=max_xy {
            let coords = (x, y);

            if corrupted.contains(&coords) {
                continue;
            }

            if x == 0 && y == 0 {
                cells.insert(
                    coords,
                    Cell {
                        cost: Some(0),
                        visited: false,
                    },
                );
            } else {
                cells.insert(
                    coords,
                    Cell {
                        cost: None,
                        visited: false,
                    },
                );
            }
        }
    }

    loop {
        let mut current: Option<Coords> = None;

        for (position, cell) in &cells {
            if !cell.visited {
                if let Some(cost) = cell.cost {
                    if current.is_none()
                        || cost < cells.get(&current.unwrap()).unwrap().cost.unwrap()
                    {
                        current = Some(*position);
                    }
                }
            }
        }

        if let Some((current_x, current_y)) = current {
            if (current_x, current_y) == (max_xy, max_xy) {
                break;
            }

            let current_cost = cells.get(&(current_x, current_y)).unwrap().cost;

            let mut changes = Vec::<(isize, isize)>::new();

            if current_x > 0 {
                changes.push((-1, 0));
            }

            if current_x < max_xy {
                changes.push((1, 0));
            }

            if current_y > 0 {
                changes.push((0, -1));
            }

            if current_y < max_xy {
                changes.push((0, 1));
            }

            for (x, y) in changes {
                let cost = if let Some(cost) = current_cost {
                    cost + 1
                } else {
                    1
                };
                let position = (
                    (current_x as isize + x) as usize,
                    (current_y as isize + y) as usize,
                );

                if let Some(cell) = cells.get_mut(&position) {
                    if let Some(cell_cost) = cell.cost {
                        if cost < cell_cost {
                            cell.cost = Some(cost);
                        }
                    } else {
                        cell.cost = Some(cost);
                    }
                }
            }

            if let Some(cell) = cells.get_mut(&(current_x, current_y)) {
                cell.visited = true;
            }
        } else {
            break;
        }
    }

    let mut result = usize::MAX;

    if let Some(cell) = cells.get(&(max_xy, max_xy)) {
        if let Some(cost) = cell.cost {
            result = cost;
        }
    }

    result
}

fn main() {
    let input = advent::get_input();

    println!("{}", get_answer(input.clone(), 70, 1024));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0"
        .split('\n')
        .map(|s| s.trim().to_string())
        .collect()
    }

    #[test]
    fn test_get_answer() {
        let fixture = get_fixture();

        assert_eq!(get_answer(fixture, 6, 12), 22);
    }
}
