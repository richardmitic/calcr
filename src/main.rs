extern crate meval;

use std::env;
use std::process::exit;

fn print_usage() -> i32 {
    println!("Usage: calcr \"1 + (2 * 3)\"");
    1
}

fn evaluate(expr: &String) -> i32 {
    let r = meval::eval_str(&expr);
    match r {
        Ok(value) => println!("{:?}", value),
        _ => println!("Something went horribly wrong")
    }
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let r = match args.len() {
        1 => {
            print_usage()
        },
        _ => evaluate(&args[1])
    };

    exit(r);
}
