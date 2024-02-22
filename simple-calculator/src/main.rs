use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the first number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let a: i32 = input.trim().parse().expect("Please type a number!");

    input.clear();
    println!("Enter the second number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let b: i32 = input.trim().parse().expect("Please type a number!");

    println!("Choose an operation: 1) Add 2) Subtract 3) Multiply 4) Divide");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: i32 = choice.trim().parse().expect("Please type a number!");

    let result = match choice {
        1 => add(a, b),
        2 => subtract(a, b),
        3 => multiply(a, b),
        4 => divide(a, b),
        _ => panic!("Invalid choice!"),
    };

    println!("The result is: {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero!");
    } else {
        a / b
    }
}
