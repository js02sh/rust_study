//cargo test --test fibonacci_hashmap -- --nocapture
use std::{collections::HashMap, cmp::Ordering, io};

// Recursive Fibonacci function with memoization
fn fibonacci(n: u128, cache: &mut HashMap<u128, u128>) -> u128 {
    // Check if the result is already in the cache
    if let Some(&result) = cache.get(&n) {
        return result;
    }

    // Calculate the Fibonacci number if it's not in the cache
    let result = match n.cmp(&2) {
        Ordering::Less => 1,
        Ordering::Equal => 1,
        Ordering::Greater => fibonacci(n - 1, cache) + fibonacci(n - 2, cache),
    };

    // Store the result in the cache for future use
    cache.insert(n, result);
    result
}

fn main() {
    println!("------------------------------------");
    println!("Welcome to the Fibonacci calculator");

    // Initialize a cache to store computed Fibonacci numbers
    let mut cache = HashMap::new();

    loop {
        println!("------------------------------------");
        println!("Put a number for the nth Fibonacci number:");
        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");
        let n = n.trim();
        match n {
            "q" => {
                println!("See you later..");
                return;
            }
            _ => {
                let n: u128 = match n.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please put a valid number");
                        continue; // Continue to the next iteration of the loop
                    }
                };

                // Calculate the nth Fibonacci number using memoization
                let fib = fibonacci(n, &mut cache);

                // Print the Fibonacci sequence up to the nth number
                for i in 1..=n {
                    print!("{} ", fibonacci(i, &mut cache));
                }
                println!("\n\nThe nth Fibonacci number is {}", fib);
                println!("Press 'q' to finish\n");
            }
        }
    }
}


#[test]
fn test() {
    main();
}