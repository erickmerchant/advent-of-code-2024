use std::collections::HashSet;

enum Direction {
    North,
    South,
    East,
    West,
}

fn part1(input: Vec<String>) -> usize {
    let lines: Vec<Vec<char>> = input
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let last_x = lines[0].len() - 1;
    let last_y = lines.len() - 1;
    let mut current: (usize, usize, Direction);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] == '^' {
                current = (x, y, Direction::North);

                loop {
                    let (x, y, direction) = &current;

                    visited.insert((*x, *y));

                    let next: (usize, usize);

                    match direction {
                        Direction::North if *y > 0 => {
                            next = (*x, y - 1);
                        }
                        Direction::South if y < &last_y => {
                            next = (*x, y + 1);
                        }
                        Direction::East if x < &last_x => {
                            next = (x + 1, *y);
                        }
                        Direction::West if *x > 0 => {
                            next = (x - 1, *y);
                        }
                        _ => break,
                    }

                    let (x, y) = next;

                    if lines[y][x] == '#' {
                        current.2 = match current.2 {
                            Direction::North => Direction::East,
                            Direction::East => Direction::South,
                            Direction::South => Direction::West,
                            Direction::West => Direction::North,
                        };
                    } else {
                        current = (x, y, current.2);
                    }
                }

                break;
            }
        }
    }

    visited.len()
}

fn part2(_input: Vec<String>) -> usize {
    todo!()
}

fn main() {
    let input = advent::get_input();

    println!("{}", part1(input.clone()));
    println!("{}", part2(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#..."
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        let fixture = get_fixture();

        assert_eq!(part1(fixture), 41);
    }

    #[test]
    fn test_part2() {
        let fixture = get_fixture();

        assert_eq!(part2(fixture), 0);
    }
}
