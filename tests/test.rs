use std::{cmp::Ordering, io};
//use std::cmp::Ordering;

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
        let n = n.trim();
        match n {
            "q" => {
                println!("See you later..");
                return;
            }
            _ => {
                let n: u64 = match n.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please put a valid number");
                        continue; // Continue to the next iteration of the loop
                    }
                };

                let fib = fibonacci(n);

                for i in 1..=n {
                    print!("{} ", fibonacci(i));
                }
                println!("\nThe nth Fibonacci number is {}", fib);
                println!("Press 'q' to finish\n");
            }
        }
    }
}



#[test]
fn test() {
    main();
}