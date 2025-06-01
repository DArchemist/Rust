fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is {x}");

    // Tuples
    let tup = (500, 6.4, 1);
    let (m, n, k) = tup;
    println!("The value of n is: {n}");
    let first_index = tup.0;
    println!("The value of n is: {first_index}");

    // Arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a1 = [3; 5]; // Five elements with the value of 3
    let first = a[0];
    
}
