//#![allow(dead_code)]
//#![allow(unused_variables)]
//to run it
//cargo test --test <your file name> -- --nocapture
//ex_ "cargo test --test 00_default -- --nocapture"

fn main() {
    println!("This is the Tester");
}

#[test]
fn test() {
    main();
}