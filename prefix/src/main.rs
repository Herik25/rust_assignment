fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = strings[0];
    let mut prefix = String::new();

    for (i, c) in first_string.chars().enumerate() {
        if strings.iter().all(|s| s.starts_with(&first_string[..=i])) {
            prefix.push(c);
        } else {
            break;
        }
    }

    prefix
}

fn main() {
    let strings = ["flower", "flow", "flight"];
    println!("Longest common prefix: {}", longest_common_prefix(&strings));

    let strings = ["dog", "racecar", "car"];
    println!("Longest common prefix: {}", longest_common_prefix(&strings));
}
