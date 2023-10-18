//to run it
//cargo test --test display_fmt -- --nocapture
use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    println!("This is the Tester");
}

#[test]
fn test() {
    main();
}