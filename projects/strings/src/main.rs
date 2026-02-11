fn main() {
    let mut count = 0;
    let hello = String::from("Hello");
    println!("{}", hello); // "Hello, world!"
    loop{
        count += 1 ;
        println!("{},{}", count, hello);
        if count == 101 {
            break;
        }
    }
}

