pub fn convert_to_number(s: &str) -> i32 {
    s.parse::<i32>().unwrap_or(0)
}