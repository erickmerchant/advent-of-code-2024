use regex::Regex;
use std::sync::LazyLock;

static ROBOT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap());

fn get_answer(input: Vec<String>, (width, height): (isize, isize)) -> usize {
    let mut nw_total = 0;
    let mut ne_total = 0;
    let mut se_total = 0;
    let mut sw_total = 0;
    let max_w_x = width / 2;
    let min_e_x = max_w_x + (width % 2) - 1;
    let max_n_y = height / 2;
    let min_s_y = max_n_y + (height % 2) - 1;

    for line in input {
        for (_, [a, b, c, d]) in ROBOT_REGEX.captures_iter(&line).map(|c| c.extract()) {
            let mut x: isize = a.parse().unwrap();
            let mut y: isize = b.parse().unwrap();
            let x_velocity: isize = c.parse().unwrap();
            let y_velocity: isize = d.parse().unwrap();

            x += x_velocity * 100;

            while x < 0 {
                x += width;
            }

            y += y_velocity * 100;

            while y < 0 {
                y += height;
            }

            x %= width;
            y %= height;

            if y < max_n_y && x < max_w_x {
                nw_total += 1;
            }

            if y < max_n_y && x > min_e_x {
                ne_total += 1;
            }

            if y > min_s_y && x > min_e_x {
                se_total += 1;
            }

            if y > min_s_y && x < max_w_x {
                sw_total += 1;
            }
        }
    }

    nw_total * ne_total * se_total * sw_total
}

fn main() {
    let input = advent::get_input();

    println!("{}", get_answer(input.clone(), (101, 103)));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_get_answer() {
        let fixture = get_fixture();

        assert_eq!(get_answer(fixture, (11, 7)), 12);
    }
}
