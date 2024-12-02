pub mod day01;
pub mod day02;

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
