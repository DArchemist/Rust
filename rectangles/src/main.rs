// fn main() {
//     let width1: usize = 30;
//     let height1: usize = 50;

//     println!("The area of the rectangle is {} square pixels.", area(width1, height1));
// }

// fn area(width: usize, height: usize) -> usize {
//     width * height
// }

// Refactoring with tuples

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }


// Refactoring with structs and methods

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
}
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    // println!("rect1 is {rect1:#?}");
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// Way of calling methods instance.method()
// Way of calling associated functions Type::function()