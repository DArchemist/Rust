fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.")
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutable_reference() {
    let mut s = String::from("hello");

    change(&mut s); // Only one mutable reference can be created
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}