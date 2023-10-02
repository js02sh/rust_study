#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    println!("Rust Tester");
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