use std::io;
use meval;

pub fn get_inputs() {
    let mut expr_str = String::new();

    let mut first = String::new();
    let mut op = String::new();
    let mut second = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut first).expect("Failed to read first number.");

    println!("Enter operator (+, -, *, /):");
    io::stdin().read_line(&mut op).expect("Failed to read operator.");

    println!("Enter second number:");
    io::stdin().read_line(&mut second).expect("Failed to read second number.");

    // Build expression
    expr_str.push_str(first.trim());
    expr_str.push_str(op.trim());
    expr_str.push_str(second.trim());

    match meval::eval_str(&expr_str) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error evaluating expression: {}", e),
    }
}

fn main() {
    get_inputs();
}

