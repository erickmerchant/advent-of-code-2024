use std::io::{self, BufRead};

pub fn get_input() -> Vec<String> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    handle.lines().map_while(Result::ok).collect()
}

pub fn parse_vec_from_line<T>(line: String) -> Result<Vec<T>, T::Err>
where
    T: std::str::FromStr,
{
    let mut parts: Vec<T> = vec![];

    for t in line.split_whitespace() {
        parts.push(t.parse::<T>()?);
    }

    Ok(parts)
}
