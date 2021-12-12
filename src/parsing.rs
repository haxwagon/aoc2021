use std::borrow::Cow;

pub fn parse_input<T, F>(s: &str, mut f: F) -> Vec<T>
where
    F: FnMut(Vec<&str>) -> T,
{
    s.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.split_whitespace().collect())
        .map(|parts| f(parts))
        .collect()
}

pub fn parse_lines(s: &str) -> Vec<Cow<str>> {
    s.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|s| Cow::from(s))
        .collect()
}
