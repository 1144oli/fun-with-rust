// my fizzbuzz 
fn main() {
    let mut count = 0;
    loop{
        count += 1;
        if count % 3 == 0 && count % 5 == 0 {
        println!("fizzbuzz");
        }
        else if count % 3 == 0 { 
        println!("fizz");
        }
        else if count % 5 == 0 { 
        println!("buzz");
        }
        else if count == 101{
            break;
        }
        else { 
        println!("{}",count);
        }
    }
}
// most optimal 
/* fn main() {
    for i in 1..=100 {
        match (i % 3 == 0, i % 5 == 0) {
            (true, true) => println!("fizzbuzz"),
            (true, false) => println!("fizz"),
            (false, true) => println!("buzz"),
            _ => println!("{}", i),
        }
    }
} */