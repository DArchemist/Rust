fn main() {
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }
}

fn for_loops() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
}