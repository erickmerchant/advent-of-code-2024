use std::collections::HashMap;

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
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Weight {
    cost: usize,
    direction: Direction,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Current {
    position: Position,
    weight: Weight,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Node {
    weight: Option<Weight>,
    visited: bool,
}

fn get_answer(input: Vec<String>) -> usize {
    let mut nodes: HashMap<Position, Node> = HashMap::new();
    let mut end: Option<Position> = None;

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    nodes.insert(
                        Position { x, y },
                        Node {
                            weight: None,
                            visited: false,
                        },
                    );
                }
                'S' => {
                    nodes.insert(
                        Position { x, y },
                        Node {
                            weight: Some(Weight {
                                cost: 0,
                                direction: Direction::East,
                            }),
                            visited: false,
                        },
                    );
                }
                'E' => {
                    end = Some(Position { x, y });

                    nodes.insert(
                        Position { x, y },
                        Node {
                            weight: None,
                            visited: false,
                        },
                    );
                }
                _ => {}
            }
        }
    }

    loop {
        let mut current: Option<Current> = None;

        for (position, node) in &nodes {
            if node.visited == false {
                if let Some(weight) = node.clone().weight {
                    if current.is_none() || weight.cost < current.clone().unwrap().weight.cost {
                        current = Some(Current {
                            position: Position {
                                x: position.x,
                                y: position.y,
                            },
                            weight,
                        });
                    }
                }
            }
        }

        if let Some(current) = current {
            if current.position == end.clone().unwrap() {
                break;
            }

            for direction in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ] {
                let change = match current.weight.direction {
                    Direction::North => match direction {
                        Direction::North => Some((0, -1, 1, Direction::North)),
                        Direction::West => Some((-1, 0, 1001, Direction::West)),
                        Direction::South => None,
                        Direction::East => Some((1, 0, 1001, Direction::East)),
                    },
                    Direction::West => match direction {
                        Direction::North => Some((0, -1, 1001, Direction::North)),
                        Direction::West => Some((-1, 0, 1, Direction::West)),
                        Direction::South => Some((0, 1, 1001, Direction::South)),
                        Direction::East => None,
                    },
                    Direction::South => match direction {
                        Direction::North => None,
                        Direction::West => Some((-1, 0, 1001, Direction::West)),
                        Direction::South => Some((0, 1, 1, Direction::South)),
                        Direction::East => Some((1, 0, 1001, Direction::East)),
                    },
                    Direction::East => match direction {
                        Direction::North => Some((0, -1, 1001, Direction::North)),
                        Direction::West => None,
                        Direction::South => Some((0, 1, 1001, Direction::South)),
                        Direction::East => Some((1, 0, 1, Direction::East)),
                    },
                };

                if let Some((diff_x, diff_y, diff_cost, direction)) = change {
                    let cost = current.weight.cost + diff_cost;

                    let position = Position {
                        x: (current.position.x as isize + diff_x) as usize,
                        y: (current.position.y as isize + diff_y) as usize,
                    };

                    if let Some(node) = nodes.get_mut(&position) {
                        if let Some(weight) = node.weight.clone() {
                            if cost < weight.cost {
                                node.weight = Some(Weight { cost, direction });
                            }
                        } else {
                            node.weight = Some(Weight { cost, direction });
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

    if let Some(end) = end {
        nodes.get(&end).unwrap().clone().weight.unwrap().cost
    } else {
        usize::MAX
    }
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
