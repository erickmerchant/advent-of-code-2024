use regex::Regex;
use std::{collections::HashMap, sync::LazyLock};

static VALUE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([a-z0-9]+): (0|1)").unwrap());
static CHANGE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([a-z0-9]+) (XOR|AND|OR) ([a-z0-9]+) -> ([a-z0-9]+)").unwrap());

fn get_answer(input: Vec<String>) -> usize {
    let mut is_values = true;
    let mut values: HashMap<String, bool> = HashMap::new();
    let mut changes: Vec<(String, String, String, String)> = Vec::new();

    for line in input {
        if line.is_empty() {
            is_values = false;

            continue;
        }

        if is_values {
            for (_, [a, b]) in VALUE_REGEX.captures_iter(&line).map(|c| c.extract()) {
                values.insert(a.to_string(), b.parse::<u8>().unwrap() == 1);
            }
        } else {
            for (_, [a, b, c, d]) in CHANGE_REGEX.captures_iter(&line).map(|c| c.extract()) {
                changes.push((a.to_string(), b.to_string(), c.to_string(), d.to_string()));
            }
        }
    }

    while !changes.is_empty() {
        let mut new_changes = Vec::new();

        for (a, b, c, d) in changes {
            if let (Some(a), Some(c)) = (values.get(&a), values.get(&c)) {
                if !values.contains_key(&d) {
                    match b.as_str() {
                        "AND" => {
                            values.insert(d.to_string(), a & c);
                        }
                        "OR" => {
                            values.insert(d.to_string(), a | c);
                        }
                        "XOR" => {
                            values.insert(d.to_string(), a ^ c);
                        }
                        _ => panic!(),
                    }
                }
            } else {
                new_changes.push((a, b, c, d));
            }
        }

        changes = new_changes;
    }

    let mut result = 0;

    for (key, value) in values {
        if let Some(key) = key.strip_prefix("z") {
            let key = key.parse::<usize>().unwrap();

            if value {
                result |= 1 << key;
            }
        }
    }

    result
}

fn main() {
    let input = advent::get_input();

    println!("{}", get_answer(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_get_answer() {
        let fixture = get_fixture();

        assert_eq!(get_answer(fixture), 2024);
    }
}
