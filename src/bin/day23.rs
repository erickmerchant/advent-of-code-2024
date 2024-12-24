use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn get_answer(input: Vec<String>) -> usize {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input {
        let line = line.to_owned();
        let parts: Vec<&str> = line.split('-').collect();

        let a = connections.entry(parts[0].to_string()).or_default();

        a.insert(parts[1].to_string());

        let b = connections.entry(parts[1].to_string()).or_default();

        b.insert(parts[0].to_string());
    }

    let mut results: Vec<Vec<String>> = Vec::new();

    for (key_a, set_a) in connections.clone() {
        if key_a.starts_with('t') {
            for key_b in set_a.clone() {
                let set_b = connections.get(&key_b).unwrap();

                let keys = set_a.intersection(set_b).collect::<Vec<_>>();

                for key_c in keys {
                    let mut result = vec![key_a.clone(), key_b.clone(), key_c.clone()];

                    result.sort();

                    results.push(result);
                }
            }
        }
    }

    results.iter().unique().collect::<Vec<_>>().len()
}

fn main() {
    let input = advent::get_input();

    println!("{}", get_answer(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_get_answer() {
        let fixture = get_fixture();

        assert_eq!(get_answer(fixture), 7);
    }
}
