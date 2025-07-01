fn main() {
    let s = String::from("Hello world!");

    let hello = &s[0..5];
    let world = &s[6..11];

    // The following are different ways to express the same
    let slice = &s[0..2];
    let slice = &s[..0];

    // The following are different ways to express the same
    let slice = &s[3..len];
    let slice = &s[3..];

    // To take a slice of the entire string, do:
    let slice = &s[..];
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
