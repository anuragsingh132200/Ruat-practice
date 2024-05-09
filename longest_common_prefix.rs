pub fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = &strings[0];
    let mut prefix = String::new();

    'outer: for (i, c) in first_string.chars().enumerate() {
        for string in &strings[1..] {
            if let Some(next_char) = string.chars().nth(i) {
                if next_char != c {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(c);
    }

    prefix
}
