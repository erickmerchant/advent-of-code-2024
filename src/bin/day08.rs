use std::{
    collections::{HashMap, HashSet},
    ops::{Div, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    pub x: isize,
    pub y: isize,
}

impl Div<isize> for Point {
    type Output = Self;

    fn div(self, rhs: isize) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn step(&self, a: Self, b: Self) -> Self {
        Self::new(
            if a.x < b.x {
                a.x - self.x
            } else {
                a.x + self.x
            },
            if a.y < b.y {
                a.y - self.y
            } else {
                a.y + self.y
            },
        )
    }

    fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

struct Lines {
    pub points: Vec<(char, Point)>,
    last: Point,
}

impl Lines {
    fn new(input: Vec<String>) -> Self {
        let lines: Vec<Vec<char>> = input.into_iter().map(|s| s.chars().collect()).collect();
        let mut points: Vec<(char, Point)> = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            let y = y as isize;

            for (x, c) in line.iter().enumerate() {
                let x = x as isize;
                let point = Point::new(x, y);

                if *c == '.' {
                    continue;
                }

                points.push((*c, point));
            }
        }

        let last = Point::new((lines[0].len() - 1) as isize, (lines.len() - 1) as isize);

        Self { points, last }
    }

    fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x <= self.last.x && point.y <= self.last.y
    }
}

fn part1(input: Vec<String>) -> usize {
    let lines = Lines::new(input);
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut antinodes: HashSet<Point> = HashSet::new();

    for (c, current) in lines.points.iter() {
        let antenna = antennas.entry(*c).or_default();

        for previous in antenna.iter() {
            let delta = (*current - *previous).abs();
            let potentials = [
                delta.step(*current, *previous),
                delta.step(*previous, *current),
            ];

            for potential in potentials.iter() {
                if !lines.contains(*potential) {
                    continue;
                }

                antinodes.insert(*potential);
            }
        }

        antenna.push(*current);
    }

    antinodes.len()
}

fn part2(input: Vec<String>) -> usize {
    let lines = Lines::new(input);
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut antinodes: HashSet<Point> = HashSet::new();

    for (c, current) in lines.points.iter() {
        let antenna = antennas.entry(*c).or_default();

        for previous in antenna.iter() {
            let delta = (*current - *previous).abs();
            let gcd = num::integer::gcd(delta.x, delta.y);
            let delta = delta / gcd;
            let mut point = *current;

            loop {
                if !lines.contains(point) {
                    break;
                }

                antinodes.insert(point);

                point = delta.step(point, *previous);
            }

            let mut point = *previous;

            loop {
                if !lines.contains(point) {
                    break;
                }

                antinodes.insert(point);

                point = delta.step(point, *current);
            }
        }

        antenna.push(*current);
    }

    antinodes.len()
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
        "............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        let fixture = get_fixture();

        assert_eq!(part1(fixture), 14);
    }

    #[test]
    fn test_part2() {
        let fixture = get_fixture();

        assert_eq!(part2(fixture), 34);
    }
}
