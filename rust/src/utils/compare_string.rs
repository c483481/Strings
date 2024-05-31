pub fn compare_string(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false
    }

    for (x, y) in a.bytes().zip(b.bytes()) {
        if x != y {
            return false;
        }
    }

    true
}

pub fn safe_compare_string(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false
    }

    let mut result = 0;

    for (x, y) in a.bytes().zip(b.bytes()) {
       result |= x ^ y;
    }

    result == 0
}
