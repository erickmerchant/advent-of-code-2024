use regex::Regex;
use std::sync::LazyLock;

static GAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"Button A: X\+([0-9]+), Y\+([0-9]+)
Button B: X\+([0-9]+), Y\+([0-9]+)
Prize: X\=([0-9]+), Y\=([0-9]+)",
    )
    .unwrap()
});

#[derive(Debug, Copy, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
struct Game {
    button_a: Coords,
    button_b: Coords,
    prize: Coords,
}

fn part1(input: String) -> usize {
    let mut games: Vec<Game> = vec![];

    for (_, [a, b, c, d, e, f]) in GAME_REGEX.captures_iter(&input).map(|c| c.extract()) {
        games.push(Game {
            button_a: Coords {
                x: a.parse().unwrap(),
                y: b.parse().unwrap(),
            },
            button_b: Coords {
                x: c.parse().unwrap(),
                y: d.parse().unwrap(),
            },
            prize: Coords {
                x: e.parse().unwrap(),
                y: f.parse().unwrap(),
            },
        });
    }

    let mut total_cost = 0;

    for game in games {
        let mut prev_cost = usize::MAX;

        for a in 0..=1000 {
            for b in 0..=1000 {
                let cost = (a * 3) + b;
                let x = (game.button_a.x * a) + (game.button_b.x * b);
                let y = (game.button_a.y * a) + (game.button_b.y * b);

                if x == game.prize.x && y == game.prize.y && cost < prev_cost {
                    prev_cost = cost;
                }
            }
        }

        if prev_cost < usize::MAX {
            total_cost += prev_cost;
        }
    }

    total_cost
}

fn main() {
    let input = advent::get_input().join("\n");

    println!("{}", part1(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> String {
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
            .to_string()
    }

    #[test]
    fn test_part1() {
        let fixture = get_fixture();

        assert_eq!(part1(fixture), 480);
    }
}
