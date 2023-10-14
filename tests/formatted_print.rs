//to run it
//cargo test --test formatted_print -- --nocapture

use core::fmt;

fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over");
    
    println!("Base 10:               {}", 69420);
    println!("Base 2 (binary):       {:b}", 69420);
    println!("Base 8 (octal):        {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);
    println!("Base 16 (hesxadecimal: {:X}", 69420);

    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);
    println!("{number:0>width$}", number = 1, width = 5);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)] //disable 'dead_code' with warn againist
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    // fmt::Display.
    println!("This struct '{}' won't print...", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);


}

#[test]
fn test() {
    main();
}