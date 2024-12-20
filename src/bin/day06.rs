use itertools::Itertools;
use rayon::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Debug, Copy, Clone, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Step {
    point: Point,
    direction: Direction,
}

fn part1(input: Vec<String>) -> usize {
    let lines: Vec<Vec<char>> = input.into_iter().map(|s| s.chars().collect()).collect();
    let (path, _) = get_path(&lines, get_start(&lines), None);
    let visited = path.iter().map(|step| step.point).unique();

    visited.count()
}

fn part2(input: Vec<String>) -> usize {
    let lines: Vec<Vec<char>> = input.into_iter().map(|s| s.chars().collect()).collect();
    let last_x = lines[0].len() - 1;
    let last_y = lines.len() - 1;
    let start = get_start(&lines);
    let (path, _) = get_path(&lines, start, None);
    let results: Vec<Option<Point>> = path
        .par_iter()
        .map(|step| {
            let placed_object = match get_move(*step, last_y, last_x) {
                Some(point) => point,
                None => return None,
            };

            if start.point == placed_object || lines[placed_object.y][placed_object.x] == '#' {
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
    let mut visited: Vec<Step> = Vec::new();
    let mut is_loop = false;

    loop {
        if visited.contains(&current) {
            is_loop = true;

            break;
        }

        visited.push(current);

        let next: Point = match get_move(current, last_y, last_x) {
            Some(point) => point,
            None => break,
        };

        if lines[next.y][next.x] == '#' || placed_object == Some(next) {
            current.direction = match current.direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
        } else {
            current = Step {
                point: next,
                direction: current.direction,
            };
        }
    }

    (visited, is_loop)
}

fn get_start(lines: &[Vec<char>]) -> Step {
    for (y, line) in lines.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if cell == &'^' {
                return Step {
                    point: Point { x, y },
                    direction: Direction::North,
                };
            }
        }
    }

    panic!()
}

fn get_move(step: Step, last_y: usize, last_x: usize) -> Option<Point> {
    match step.direction {
        Direction::North if step.point.y > 0 => Some(Point {
            x: step.point.x,
            y: step.point.y - 1,
        }),
        Direction::South if step.point.y < last_y => Some(Point {
            x: step.point.x,
            y: step.point.y + 1,
        }),
        Direction::East if step.point.x < last_x => Some(Point {
            x: step.point.x + 1,
            y: step.point.y,
        }),
        Direction::West if step.point.x > 0 => Some(Point {
            x: step.point.x - 1,
            y: step.point.y,
        }),
        _ => None,
    }
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
