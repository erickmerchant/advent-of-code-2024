use std::collections::HashMap;

#[derive(Debug)]
enum Square {
    Wall,
    Box,
    Open,
    Robot,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn get_answer(input: Vec<String>) -> usize {
    let mut grid: HashMap<Point, Square> = HashMap::new();
    let mut grid_done = false;
    let mut directions: Vec<char> = Vec::new();
    let mut current = Point { x: 0, y: 0 };
    let last_x = input[0].len() - 1;
    let last_y = input.len() - 1;

    for (y, line) in input.iter().enumerate() {
        if line.is_empty() {
            grid_done = true;
            continue;
        }

        let mut chars: Vec<char> = line.chars().collect();

        if !grid_done {
            for (x, c) in chars.iter().enumerate() {
                let square = match c {
                    '#' => Square::Wall,
                    'O' => Square::Box,
                    '.' => Square::Open,
                    '@' => {
                        current = Point { x, y };

                        Square::Robot
                    }
                    _ => panic!(),
                };

                grid.insert(Point { x, y }, square);
            }
        } else {
            directions.append(&mut chars);
        }
    }

    for direction in directions {
        let (delta_x, delta_y) = match direction {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => continue,
        };

        if current.y == 0 || current.y >= last_y || current.x == 0 || current.x >= last_x {
            continue;
        }

        let next = Point {
            x: (current.x as isize + delta_x) as usize,
            y: (current.y as isize + delta_y) as usize,
        };
        let mut next_open = None;
        let mut next_next = Point {
            x: next.x,
            y: next.y,
        };

        loop {
            if let Some(&Square::Open) = grid.get(&next_next) {
                next_open = Some(next_next.clone());
                break;
            }

            if next_next.y == 0
                || next_next.y >= last_y
                || next_next.x == 0
                || next_next.x >= last_x
            {
                break;
            }

            if let Some(&Square::Box) = grid.get(&next_next) {
                next_next = Point {
                    x: (next_next.x as isize + delta_x) as usize,
                    y: (next_next.y as isize + delta_y) as usize,
                };
            } else {
                break;
            }
        }

        if let Some(Square::Box) = grid.get(&next) {
            if let Some(position) = next_open {
                grid.insert(position, Square::Box);
                grid.insert(next.clone(), Square::Open);
            }
        }

        if let Some(Square::Open) = grid.get(&next) {
            grid.insert(current, Square::Open);
            grid.insert(next.clone(), Square::Robot);
            current = next;
        }
    }

    let mut total = 0;

    for (point, square) in grid {
        if let Square::Box = square {
            total += (point.y * 100) + point.x;
        }
    }

    total
}

fn main() {
    let input = advent::get_input();

    println!("{}", get_answer(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture1() -> Vec<String> {
        "########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    fn get_fixture2() -> Vec<String> {
        "##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test1_get_answer() {
        let fixture = get_fixture1();

        assert_eq!(get_answer(fixture), 2028);
    }

    #[test]
    fn test2_get_answer() {
        let fixture = get_fixture2();

        assert_eq!(get_answer(fixture), 10092);
    }
}
