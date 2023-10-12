//to run it
//cargo test --test calculator_self -- --nocapture
//use std::io;

struct Calculator {
    value: f64,
}

impl Calculator {
    fn new() -> Calculator {
        Calculator { value: 0.0 }
    }

    fn add(&mut self, x:f64) {
        self.value += x;
    }

    fn multiply(&mut self, x: f64) {
        self.value *= x;
    }

    fn clear(&mut self) {
        self.value = 0.0;
    }
}


fn main() {
    println!("Welcome to the Caculator");
    let mut calculator = Calculator::new();
    calculator.add(5.0);
    calculator.multiply(2.0);
    println!("Result: {}", calculator.value);
    calculator.clear();
}

#[test]
fn test() {
    main();
}