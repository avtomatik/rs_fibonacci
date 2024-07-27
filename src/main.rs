use std::io;

fn main() {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Please enter the cardinal number");

    let number: u32 = number
        .trim()
        .parse()
        .expect("Float-like is required");

    println!("Fibonacci number is {}", fib(number))
}

fn fib(number: u32) -> u32 {
    match number {
       0..=2 => 1,
       _ =>  fib(number - 2) + fib(number - 1)
    }
}
