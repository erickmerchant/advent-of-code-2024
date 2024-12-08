use std::collections::{HashMap, HashSet};

fn part1(input: Vec<String>) -> usize {
    let lines: Vec<Vec<char>> = input.into_iter().map(|s| s.chars().collect()).collect();
    let last_x = (lines[0].len() - 1) as isize;
    let last_y = (lines.len() - 1) as isize;
    let mut antennas = HashMap::<char, Vec<(isize, isize)>>::new();
    let mut antinodes = HashSet::<(isize, isize)>::new();

    for (y, line) in lines.iter().enumerate() {
        let y = y as isize;

        for (x, c) in line.iter().enumerate() {
            let x = x as isize;

            if *c == '.' {
                continue;
            }

            let antenna = antennas.entry(*c).or_insert(Vec::new());

            for (xx, yy) in antenna.iter() {
                let x_delta = (x - *xx).abs();
                let y_delta = (y - *yy).abs();

                let potentials = [
                    (
                        if x < *xx { x - x_delta } else { x + x_delta },
                        if y < *yy { y - y_delta } else { y + y_delta },
                    ),
                    (
                        if *xx < x { xx - x_delta } else { xx + x_delta },
                        if *yy < y { yy - y_delta } else { yy + y_delta },
                    ),
                ];

                for (x, y) in potentials.iter() {
                    if *x < 0 || *y < 0 || *x > last_x || *y > last_y {
                        continue;
                    }

                    antinodes.insert((*x, *y));
                }
            }

            antenna.push((x, y));
        }
    }

    antinodes.len()
}

fn part2(input: Vec<String>) -> usize {
    let lines: Vec<Vec<char>> = input.into_iter().map(|s| s.chars().collect()).collect();
    let last_x = (lines[0].len() - 1) as isize;
    let last_y = (lines.len() - 1) as isize;
    let mut antennas = HashMap::<char, Vec<(isize, isize)>>::new();
    let mut antinodes = HashSet::<(isize, isize)>::new();

    for (y, line) in lines.iter().enumerate() {
        let y = y as isize;

        for (x, c) in line.iter().enumerate() {
            let x = x as isize;

            if *c == '.' {
                continue;
            }

            let antenna = antennas.entry(*c).or_insert(Vec::new());

            for (xx, yy) in antenna.iter() {
                antinodes.insert((*xx, *yy));
                antinodes.insert((x, y));

                let x_delta = (x - *xx).abs();
                let y_delta = (y - *yy).abs();
                let gcd = num::integer::gcd(x_delta, y_delta);
                let x_delta = x_delta / gcd;
                let y_delta = y_delta / gcd;
                let mut px = if x < *xx { x - x_delta } else { x + x_delta };
                let mut py = if y < *yy { y - y_delta } else { y + y_delta };

                loop {
                    if px < 0 || py < 0 || px > last_x || py > last_y {
                        break;
                    }

                    antinodes.insert((px, py));

                    px = if x < *xx { px - x_delta } else { px + x_delta };
                    py = if y < *yy { py - y_delta } else { py + y_delta };
                }

                let mut px = if *xx < x { xx - x_delta } else { xx + x_delta };
                let mut py = if *yy < y { yy - y_delta } else { yy + y_delta };

                loop {
                    if px < 0 || py < 0 || px > last_x || py > last_y {
                        break;
                    }

                    antinodes.insert((px, py));

                    px = if *xx < x { px - x_delta } else { px + x_delta };
                    py = if *yy < y { py - y_delta } else { py + y_delta };
                }
            }

            antenna.push((x, y));
        }
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
