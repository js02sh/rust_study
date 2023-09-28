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
    println!("------------------------------------");
    println!("Welcome to the Fibonacci calculator");
    loop {
        println!("------------------------------------");
        println!("Put number for nth Fibonacci number:");
        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Faild to read line");
        let n:u64 = n.trim().parse()
            .expect("Please input valid number");

        for i in 1..=n {
            print!("{} ", fibonacci(i));
            if i == n {
                println!("\nThe nth Fibonacci number is {}\n", fibonacci(i));
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