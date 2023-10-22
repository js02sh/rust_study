//cargo test --test factorial -- --nocapture

use std::io;

fn factorial(n: u64) -> u64 {
    match n {
        0 => 1,
        _ => n * factorial(n-1),
    }
}
fn main() {
    println!("===== Factorial Caculator =====");
    println!("Enter your number:");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Faild to read line");
    let num: u64 = num.trim().parse()
        .expect("Enter a valid number");

    let fact_n = factorial(num);
    println!("\nFactorial of {} is: {}", num, fact_n);
}

#[test]
fn test() {
    main();
}