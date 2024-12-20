use itertools::Itertools;

type Key = (usize, usize);

struct Node {
    value: u32,
    keys: Vec<Key>,
}

fn get_grid(input: Vec<String>) -> Vec<Vec<Node>> {
    let mut grid = Vec::new();
    let mut trails = Vec::new();

    for (y, line) in input.iter().enumerate() {
        let mut row = Vec::new();

        for (x, value) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            let cell = Node {
                value,
                keys: Vec::new(),
            };

            row.push(cell);

            if value == 9 {
                trails.push(((x, y), x, y));
            }
        }

        grid.push(row);
    }

    for step in (0..=8).rev() {
        let mut next_trails = Vec::new();

        for (key, x, y) in trails.clone() {
            for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let x = x as isize + dx;
                let y = y as isize + dy;

                if x < 0 || y < 0 {
                    continue;
                }

                let x = x as usize;
                let y = y as usize;

                if let Some(row) = grid.get_mut(y) {
                    if let Some(cell) = row.get_mut(x) {
                        if cell.value == step {
                            next_trails.push((key, x, y));

                            cell.keys.push(key);
                        }
                    }
                }
            }
        }

        trails = next_trails;
    }

    grid
}

fn part1(input: Vec<String>) -> usize {
    let mut result = 0;
    let grid = get_grid(input);

    for row in grid {
        for cell in row {
            if cell.value == 0 {
                result += cell.keys.iter().unique().count();
            }
        }
    }

    result
}

fn part2(input: Vec<String>) -> usize {
    let mut result = 0;
    let grid = get_grid(input);

    for row in grid {
        for cell in row {
            if cell.value == 0 {
                result += cell.keys.len();
            }
        }
    }

    result
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
        "89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        let fixture = get_fixture();

        assert_eq!(part1(fixture), 36);
    }

    #[test]
    fn test_part2() {
        let fixture = get_fixture();

        assert_eq!(part2(fixture), 81);
    }
}
