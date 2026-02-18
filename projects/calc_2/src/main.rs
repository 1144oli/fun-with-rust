use std::io;


fn main() {
    let mut histvect: Vec<String> = Vec::new();
    loop{ 
        println!("C for calc or H for history");
         let read_history:char = {

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().parse().expect("Not a valid char")
        };
        let history = read_history;
        if history == 'H' || history == 'H' {
            histfunc(&histvect);
        }
        else{
            let result:String = calc();
            histvect.push(result);
        }

    }
}

fn calc() -> String{
    println!("Calc");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let parts: Vec<&str> = input.split_whitespace().collect(); // turn into 3 thingsd  

    if parts.len() != 3 {
        println!("Invalid format. Use: number operator number\nexample: 12 + 12");
        calc();
    }
    let left: f32 = parts[0].parse().expect("Invalid number");
    let right: f32 = parts[2].parse().expect("Invalid number");

    match parts[1] {
        "+" => {println!("Sum: {}", left + right);
        format!("{} + {} = {}", left, right,left+right)
        },
        "-" => {println!("Sum: {}", left - right);
        format!("{} - {} = {}", left, right,left-right)
        },
        "*" => {println!("Sum: {}", left * right);
        format!("{} * {} = {}", left, right,left*right)
        },
        "/" => {println!("Sum: {}", left / right);
        format!("{} / {} = {}", left, right,left/right)
        },
        _ => "no".to_string()
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
