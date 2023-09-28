use std::io;
use std::cmp::Ordering;

fn fibonacci(n: u64) -> usize {
    match n.cmp(&2) {
        Ordering::Less => 1,
        Ordering::Equal => 1,
        Ordering::Greater => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Welcome to the Fibonacci calculator");
    loop {
        println!("Put number for nth Fibonacci number:");
        let mut n = String::new();

        match io::stdin().read_line(&mut n) {
            Ok(_) => {
                let n = n.trim(); // Remove newline character
                if n == "q" {
                    println!("Break");
                    break;
                } else {
                    // Parse the input as a u64
                    let n: u64 = n.parse().expect("Please write valid input");

                    let fib = fibonacci(n);

                    for i in 1..=n {
                        print!("{} ", fibonacci(i));
                    }
                    println!("\nThe nth Fibonacci number is {}", fib);
                    println!("Press 'q' to finish\n");
                }
            }
            Err(_err) => {
                println!("Failed to read line");
            }
        }
    }
}




//----------------------
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_miru() {
        main();
    }
}