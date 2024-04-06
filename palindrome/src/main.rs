fn main() {
    let s: String = String::from("naman");
    check_string(&s);
}

fn check_string(s: &String) {
    let reversed_string: String = s.chars().rev().collect();
    // println!("Reversed string: {}", reversed_string);
    let is_palindrome: bool = s == &reversed_string;
    println!("Is Palindrome: {}", is_palindrome);
}
