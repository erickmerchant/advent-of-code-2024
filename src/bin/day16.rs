use std::collections::HashMap;

type Cost = usize;

#[derive(Debug, Eq, Hash, PartialEq, Clone, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Position {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Current {
    position: Position,
    cost: Cost,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Node {
    cost: Option<Cost>,
    visited: bool,
}

fn get_answer(input: Vec<String>) -> usize {
    let mut nodes: HashMap<Position, Node> = HashMap::new();
    let mut end: Option<(usize, usize)> = None;

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    for direction in [
                        Direction::North,
                        Direction::West,
                        Direction::South,
                        Direction::East,
                    ] {
                        nodes.insert(
                            Position { x, y, direction },
                            Node {
                                cost: None,
                                visited: false,
                            },
                        );
                    }
                }
                'S' => {
                    for direction in [
                        Direction::North,
                        Direction::West,
                        Direction::South,
                        Direction::East,
                    ] {
                        nodes.insert(
                            Position {
                                x,
                                y,
                                direction: direction.clone(),
                            },
                            Node {
                                cost: if direction == Direction::East {
                                    Some(0)
                                } else {
                                    None
                                },
                                visited: false,
                            },
                        );
                    }
                }
                'E' => {
                    end = Some((x, y));
                    for direction in [
                        Direction::North,
                        Direction::West,
                        Direction::South,
                        Direction::East,
                    ] {
                        nodes.insert(
                            Position { x, y, direction },
                            Node {
                                cost: None,
                                visited: false,
                            },
                        );
                    }
                }
                _ => {}
            }
        }
    }

    loop {
        let mut current: Option<Current> = None;

        for (position, node) in &nodes {
            if !node.visited {
                if let Some(cost) = node.clone().cost {
                    if current.is_none() || cost < current.clone().unwrap().cost {
                        current = Some(Current {
                            position: position.clone(),
                            cost,
                        });
                    }
                }
            }
        }

        if let Some(current) = current {
            if (current.position.x, current.position.y) == end.unwrap() {
                break;
            }

            for direction in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ] {
                let change = match current.position.direction {
                    Direction::North => match direction {
                        Direction::North => Some((0, -1, Direction::North, 1)),
                        Direction::West => Some((-1, 0, Direction::West, 1001)),
                        Direction::South => None,
                        Direction::East => Some((1, 0, Direction::East, 1001)),
                    },
                    Direction::West => match direction {
                        Direction::North => Some((0, -1, Direction::North, 1001)),
                        Direction::West => Some((-1, 0, Direction::West, 1)),
                        Direction::South => Some((0, 1, Direction::South, 1001)),
                        Direction::East => None,
                    },
                    Direction::South => match direction {
                        Direction::North => None,
                        Direction::West => Some((-1, 0, Direction::West, 1001)),
                        Direction::South => Some((0, 1, Direction::South, 1)),
                        Direction::East => Some((1, 0, Direction::East, 1001)),
                    },
                    Direction::East => match direction {
                        Direction::North => Some((0, -1, Direction::North, 1001)),
                        Direction::West => None,
                        Direction::South => Some((0, 1, Direction::South, 1001)),
                        Direction::East => Some((1, 0, Direction::East, 1)),
                    },
                };

                if let Some((x, y, direction, cost)) = change {
                    let cost = current.cost + cost;
                    let position = Position {
                        x: (current.position.x as isize + x) as usize,
                        y: (current.position.y as isize + y) as usize,
                        direction,
                    };

                    if let Some(node) = nodes.get_mut(&position) {
                        if let Some(node_cost) = node.cost {
                            if cost < node_cost {
                                node.cost = Some(cost);
                            }
                        } else {
                            node.cost = Some(cost);
                        }
                    }
                }
            }

            if let Some(node) = nodes.get_mut(&current.position) {
                node.visited = true;
            }
        } else {
            break;
        }
    }

    let mut result = usize::MAX;

    if let Some((x, y)) = end {
        for direction in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            if let Some(node) = nodes.get(&Position { x, y, direction }) {
                if let Some(cost) = node.clone().cost {
                    if cost < result {
                        result = cost;
                    }
                }
            }
        }
    }

    result
}

fn main() {
    let input = advent::get_input();

    println!("{}", get_answer(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture1() -> Vec<String> {
        "###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    fn get_fixture2() -> Vec<String> {
        "#################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test1_get_answer() {
        let fixture = get_fixture1();

        assert_eq!(get_answer(fixture), 7036);
    }

    #[test]
    fn test2_get_answer() {
        let fixture = get_fixture2();

        assert_eq!(get_answer(fixture), 11048);
    }
}
