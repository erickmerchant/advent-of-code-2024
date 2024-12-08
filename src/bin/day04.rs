fn part1(input: Vec<String>) -> usize {
    let w = input[0].len();
    let lines: Vec<Vec<char>> = input.into_iter().map(|s| s.chars().collect()).collect();
    let mut result = 0;

    for line in lines.clone() {
        for x in 0..(w - 3) {
            if line[x] == 'X' && line[x + 1] == 'M' && line[x + 2] == 'A' && line[x + 3] == 'S' {
                result += 1;
            }

            if line[x] == 'S' && line[x + 1] == 'A' && line[x + 2] == 'M' && line[x + 3] == 'X' {
                result += 1;
            }
        }
    }

    for y in 0..(lines.len() - 3) {
        for x in 0..(w - 3) {
            if lines[y][x] == 'X'
                && lines[y + 1][x + 1] == 'M'
                && lines[y + 2][x + 2] == 'A'
                && lines[y + 3][x + 3] == 'S'
            {
                result += 1;
            }

            if lines[y][x] == 'S'
                && lines[y + 1][x + 1] == 'A'
                && lines[y + 2][x + 2] == 'M'
                && lines[y + 3][x + 3] == 'X'
            {
                result += 1;
            }
        }
    }

    for y in 0..(lines.len() - 3) {
        for x in 0..w {
            if lines[y][x] == 'X'
                && lines[y + 1][x] == 'M'
                && lines[y + 2][x] == 'A'
                && lines[y + 3][x] == 'S'
            {
                result += 1;
            }

            if lines[y][x] == 'S'
                && lines[y + 1][x] == 'A'
                && lines[y + 2][x] == 'M'
                && lines[y + 3][x] == 'X'
            {
                result += 1;
            }
        }
    }

    for y in 0..(lines.len() - 3) {
        for x in 3..w {
            if lines[y][x] == 'X'
                && lines[y + 1][x - 1] == 'M'
                && lines[y + 2][x - 2] == 'A'
                && lines[y + 3][x - 3] == 'S'
            {
                result += 1;
            }

            if lines[y][x] == 'S'
                && lines[y + 1][x - 1] == 'A'
                && lines[y + 2][x - 2] == 'M'
                && lines[y + 3][x - 3] == 'X'
            {
                result += 1;
            }
        }
    }

    result
}

fn part2(input: Vec<String>) -> usize {
    let w = input[0].len();
    let lines: Vec<Vec<char>> = input.into_iter().map(|s| s.chars().collect()).collect();
    let mut result = 0;

    for y in 0..(lines.len() - 2) {
        for x in 0..(w - 2) {
            if lines[y + 1][x + 1] != 'A' {
                continue;
            }

            if lines[y][x] == 'M'
                && lines[y + 2][x + 2] == 'S'
                && lines[y][x + 2] == 'M'
                && lines[y + 2][x] == 'S'
            {
                result += 1;
            }

            if lines[y][x] == 'S'
                && lines[y + 2][x + 2] == 'M'
                && lines[y][x + 2] == 'M'
                && lines[y + 2][x] == 'S'
            {
                result += 1;
            }

            if lines[y][x] == 'M'
                && lines[y + 2][x + 2] == 'S'
                && lines[y][x + 2] == 'S'
                && lines[y + 2][x] == 'M'
            {
                result += 1;
            }

            if lines[y][x] == 'S'
                && lines[y + 2][x + 2] == 'M'
                && lines[y][x + 2] == 'S'
                && lines[y + 2][x] == 'M'
            {
                result += 1;
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
        "MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        let fixture = get_fixture();

        assert_eq!(part1(fixture), 18);
    }

    #[test]
    fn test_part2() {
        let fixture = get_fixture();

        assert_eq!(part2(fixture), 9);
    }
}
