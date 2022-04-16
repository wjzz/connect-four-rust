pub fn parse_string(option: Option<&String>, default: usize) -> usize {
    if let Some(s) = option {
        s.parse().unwrap_or(default)
    } else {
        default
    }
}
