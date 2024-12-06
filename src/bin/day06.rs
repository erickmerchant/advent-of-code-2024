use itertools::Itertools;
use rayon::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

type Point = (usize, usize);
type Step = (usize, usize, Direction);

fn part1(input: Vec<String>) -> usize {
    let lines: Vec<Vec<char>> = input
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let (path, _) = get_path(&lines, get_start(&lines), None);
    let visited = path.iter().map(|(a, b, _d)| (a, b)).unique();

    visited.count()
}

fn part2(input: Vec<String>) -> usize {
    let lines: Vec<Vec<char>> = input
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let last_x = lines[0].len() - 1;
    let last_y = lines.len() - 1;
    let start = get_start(&lines);
    let (path, _) = get_path(&lines, start, None);
    let results: Vec<Option<Point>> = path
        .par_iter()
        .map(|(x, y, d)| {
            let placed_object = match d {
                Direction::North if *y > 0 => (*x, y - 1),
                Direction::South if y < &last_y => (*x, y + 1),
                Direction::East if x < &last_x => (x + 1, *y),
                Direction::West if *x > 0 => (x - 1, *y),
                _ => return None,
            };

            let (a, b) = placed_object;

            if start == (a, b, *d) || lines[b][a] == '#' {
                return None;
            }

            let (_, is_loop) = get_path(&lines, start, Some(placed_object));

            if is_loop {
                Some(placed_object)
            } else {
                None
            }
        })
        .collect();

    results.iter().flatten().unique().count()
}

fn get_path(lines: &[Vec<char>], start: Step, placed_object: Option<Point>) -> (Vec<Step>, bool) {
    let last_x = lines[0].len() - 1;
    let last_y = lines.len() - 1;
    let mut current: Step = start;
    let mut visited: Vec<Step> = vec![];
    let mut is_loop = false;

    loop {
        if visited.contains(&current) {
            is_loop = true;

            break;
        }

        visited.push(current);

        let (x, y, direction) = &current;
        let next: Point = match direction {
            Direction::North if *y > 0 => (*x, y - 1),
            Direction::South if y < &last_y => (*x, y + 1),
            Direction::East if x < &last_x => (x + 1, *y),
            Direction::West if *x > 0 => (x - 1, *y),
            _ => break,
        };

        let (x, y) = next;

        if lines[y][x] == '#' || placed_object == Some(next) {
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

    (visited, is_loop)
}

fn get_start(lines: &[Vec<char>]) -> Step {
    for (y, line) in lines.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if cell == &'^' {
                return (x, y, Direction::North);
            }
        }
    }

    panic!()
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

        assert_eq!(part2(fixture), 6);
    }
}
