#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
//use std::io;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // let mut s = String::new();
    // let mut f = File::open("hello.txt")?
    //     .read_to_string(&mut s)?;
    // println!("The hello.txt is {}", s);
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