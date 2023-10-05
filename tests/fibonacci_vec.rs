use std::io;

// Function to calculate the Fibonacci sequence up to the nth number
fn fibonacci(n: u128) -> Vec<u128> {
    // Create a vector to store the Fibonacci sequence
    let mut fib_sequence = vec![0; n as usize];
    
    // The first two numbers in the sequence are 1
    fib_sequence[0] = 1;
    fib_sequence[1] = 1;

    // Calculate the Fibonacci numbers iteratively
    for i in 2..n as usize {
        fib_sequence[i] = fib_sequence[i - 1] + fib_sequence[i - 2];
    }

    fib_sequence // Return the vector containing the Fibonacci sequence
}

fn main() {
    println!("------------------------------------");
    println!("Welcome to the Fibonacci calculator");

    loop {
        println!("------------------------------------");
        println!("Enter the number for nth Fibonacci number (or 'q' to quit):");

        // Read user input from the console
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        match input {
            "q" => {
                println!("See you later..");
                break; // Exit the loop if 'q' is entered
            }
            _ => {
                let n: u128 = match input.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid number");
                        continue; // Continue to the next iteration of the loop
                    }
                };

                // Calculate the Fibonacci sequence up to the nth number
                let fib_sequence = fibonacci(n);

                // Print the Fibonacci sequence and the nth Fibonacci number
                println!("The Fibonacci sequence up to the {}th number is:", n);
                for &num in &fib_sequence {
                    print!("{} ", num);
                }
                println!("\n\nThe {}th Fibonacci number is {}", n, fib_sequence[n as usize - 1]);
            }
        }
    }
}


#[test]
fn test() {
    main();
}