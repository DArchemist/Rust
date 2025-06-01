fn main() {
    let number = 3;

    if number < 5 { // Condition must be a bool
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let condition: bool = true;
    let number2 = if condition { 5 } else { 6 }; // Equivalent to the ternary operator in JavaScript 
}
