//cargo test --test gcd_euclid -- --nocapture

use std::io;

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while n != 0 { 
        if n < m {
            let t = n;
            n = m;
            m = t;
        }
        n %= m;
    }
    m
}

fn main() {
    println!("Let's find the Greatest Common Divisor of two Integer");
    println!("Please write two numbers");
    let (mut m,mut n) = (String::new(), String::new());
    println!("First: ");
    io::stdin()
        .read_line(&mut m)
        .expect("Faild to read line");
    println!("Second:");
    io::stdin()
        .read_line(&mut n)
        .expect("Faild to read line");

    let m: u64 = m.trim().parse().expect("Please input valid number");
    let n: u64 = n.trim().parse().expect("Please input valid number");

    println!("{} is The Greatest Common Divisor between {}, {} by Euclid's Algorithm", gcd(m,n), m, n);

}




#[test]
fn test() {
    main();
}