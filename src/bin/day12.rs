#[derive(Debug, Copy, Clone)]
struct Plot {
    kind: char,
    cell_count: usize,
    fence_count: usize,
}

fn part1(input: Vec<String>) -> usize {
    let lines: Vec<Vec<char>> = input.into_iter().map(|s| s.chars().collect()).collect();
    let last_x = lines[0].len() - 1;
    let last_y = lines.len() - 1;
    let mut processed: Vec<(usize, usize)> = vec![];
    let mut plots: Vec<Plot> = vec![];

    for (y, line) in lines.iter().enumerate() {
        for (x, kind) in line.iter().enumerate() {
            if processed.contains(&(x, y)) {
                continue;
            }

            let mut plot = Plot {
                kind: *kind,
                cell_count: 0,
                fence_count: 0,
            };
            let mut stack = vec![(x, y)];

            while let Some((x, y)) = stack.pop() {
                if processed.contains(&(x, y)) {
                    if lines[y][x] != plot.kind {
                        plot.fence_count += 1;
                    }

                    continue;
                }

                if lines[y][x] != plot.kind {
                    plot.fence_count += 1;
                    continue;
                }

                plot.cell_count += 1;

                if x == 0 {
                    plot.fence_count += 1;
                }

                if x == last_x {
                    plot.fence_count += 1;
                }

                if y == 0 {
                    plot.fence_count += 1;
                }

                if y == last_y {
                    plot.fence_count += 1;
                }

                processed.push((x, y));

                if x > 0 {
                    stack.push((x - 1, y));
                }

                if x < last_x {
                    stack.push((x + 1, y));
                }

                if y > 0 {
                    stack.push((x, y - 1));
                }

                if y < last_y {
                    stack.push((x, y + 1));
                }
            }

            plots.push(plot);
        }
    }

    let mut result = 0;

    for plot in plots.iter() {
        result += plot.cell_count * plot.fence_count;
    }

    result
}

fn main() {
    let input = advent::get_input();

    println!("{}", part1(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        let fixture = get_fixture();

        assert_eq!(part1(fixture), 1930);
    }
}
