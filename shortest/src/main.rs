fn shortest_word(s: &str) -> &str {
    let mut shortest = "";

    for word in s.split_whitespace() {
        if shortest.is_empty() || word.len() < shortest.len() {
            shortest = word;
        }
    }

    shortest
}

fn main() {
    let sentence = "This is a sample sentence with some words";
    let shortest = shortest_word(sentence);

    println!("The shortest word is: {}", shortest);
}
