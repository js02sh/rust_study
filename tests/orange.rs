use std::io;
fn main() {
    println!("Hello, please input number:");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let num: u32 = num.trim().parse()
        .expect("Please input a valid number");

    for i in 1..=num {
        println!("{}", "*".repeat(i as usize));
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_miru() {
        main();
    }
}
