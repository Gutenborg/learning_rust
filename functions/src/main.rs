fn main() {
    let sentence = String::from("This is a sentence.");
    let first = first_word(&sentence);

    println!("{first}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
