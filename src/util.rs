pub fn read_line() -> String {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer.pop(); // remove newline
    buffer
}

pub fn rand() -> f64 {
    rand::random::<f64>()
}

pub fn parse_string(option: Option<&String>, default: usize) -> usize {
    if let Some(s) = option {
        s.parse().unwrap_or(default)
    } else {
        default
    }
}
