use std::io;
fn main() {
    let mut histvect: Vec<String> = Vec::new();
    loop{ 
        println!("Type Y for history N for calc");
         let read_history:char = {

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().parse().expect("Not a valid char")
        };
        let history = read_history;
        if history == 'Y' || history == 'y' {
            histfunc(&histvect);
        }
        else{
            let result:String = calc();
            histvect.push(result);
        }

    }
}

fn calc() -> String {
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

    println!("Enter operator");
    let op = read_op();
    
    println!("Enter the second number:");
    let num2 = read_num();


    match op {
        '+' => {println!("Sum: {}", num1 + num2);
        format!("{} + {} = {}", num1, num2,num1+num2)
        },
        '-' => {println!("Sum: {}", num1 - num2);
        format!("{} - {} = {}", num1, num2,num1-num2)
        },
        '*' => {println!("Sum: {}", num1 * num2);
        format!("{} * {} = {}", num1, num2,num1*num2)
        },
        '/' => {println!("Sum: {}", num1 / num2);
        format!("{} / {} = {}", num1, num2,num1/num2)
        },
        _ => "no".to_string(),
    }
}

fn histfunc(histvect:&Vec<String>) {
    println!("History: {}",histvect.len());
//    println!("{:#?}",histvect);
    let histvect_iter = histvect.iter();
    let mut count:i32 = 0; 
    for x in histvect_iter{
        count += 1; 
        println!("{} history {}",count,x);
    }
}