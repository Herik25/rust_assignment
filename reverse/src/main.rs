fn main() {
    let s: String = String::from("hello world");
    check_string(&s);
}

fn check_string(s: &String) {
    let reversed_string: String = s.chars().rev().collect();
    println!("Reversed string: {}", reversed_string);
}