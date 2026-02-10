use std::io;

fn main() {
    let read_num = || -> f32 {

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().parse().expect("Not a valid number")
    };
    let read_op = || -> char {

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Not a valid operator")
    };

    println!("Enter the first number:");
    let num1 = read_num();

    println!("Enter the second number:");
    let num2 = read_num();

    println!("Enter operator");
    let op = read_op();

    match op {
        '+' => println!("Sum: {}", num1 + num2),
        '-' => println!("Sum: {}", num1 - num2),
        '*' => println!("Sum: {}", num1 * num2),
        '/' => println!("Sum: {}", num1 / num2),
        _ => println!("not"),
    }

}