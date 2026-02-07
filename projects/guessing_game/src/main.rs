use std::cmp::Ordering;
use std::time::Instant;
use std::io;

use rand::Rng;

fn main() {
    let now = Instant::now();
    println!("Guess the number!");
    let mut counter = 0;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number!");
                continue;
            },
        }; 

        counter += 1;

        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                let elapsed = now.elapsed();
                println!("You spent: {elapsed:.2?}");
                break;
            }
        }

        if counter == 3 {
            println!("3 guess over, you lose!");
            let elapsed = now.elapsed();
            println!("You spent: {elapsed:.2?}");
            break;
        }
    }
}


/* fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
//this is a comment
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
} */

/* fn main() {
    let number = 7;
    println!("Guess the number!");
    println!("Please say your number.");

    let mut guess = String::new();
//hello
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    if guess == number.to_string() {
        println!("You win!");
    } else {
        println!("You lose!");
    }
} */

