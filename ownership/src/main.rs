fn main() {
    let s = String::from("hello"); // Reference to the heap. This type implements the .drop method

    takes_ownership(s);

    println!("{s}");

    let x = 5; // References the stack. Implements the Copy trait.

    makes_copy(x);

    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
