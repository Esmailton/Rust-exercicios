use std::io;

fn perform_operation(expression: &str, signal: char, op: impl Fn(f32, f32) -> f32) {
    let values: Vec<&str> = expression.split(signal).collect();
    let f = op(
        values[0].trim().parse::<f32>().unwrap(),
        values[1].trim().parse::<f32>().unwrap(),
    );
    println!("The result is ->: {}", f);
}

fn main() {
    println!("============================= Simples calculator Example =====================================");
    println!("Your operations need to follow the pattern as in the example: '5-2', '5+2', '5*2', '5/2'");

    let signals_aritimetic = ['+', '-', '/', '*'];

    println!("Enter your operation: ");

    let mut operation = String::new();

    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to get operation");

    for signal in signals_aritimetic {
        if let Some(_result) = operation.find(signal) {
            match signal {
                '-' => perform_operation(&operation, signal, |x, y| x - y),
                '+' => perform_operation(&operation, signal, |x, y| x + y),
                '*' => perform_operation(&operation, signal, |x, y| x * y),
                '/' => perform_operation(&operation, signal, |x, y| x / y),
                _ => todo!(),
            };
        }
    }
}
