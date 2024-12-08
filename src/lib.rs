use std::io::{self, BufRead};

pub fn get_input() -> Vec<String> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    handle.lines().map_while(Result::ok).collect()
}
