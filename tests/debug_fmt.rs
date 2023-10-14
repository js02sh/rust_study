//to run it
//cargo test --test debug_fmt -- --nocapture
#![allow(dead_code)]
//#![allow(unused_variables)]

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("{:?} moths is a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor="actor's"
    );

    //'Structure' is printable!
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    //Pretty print
    println!("{:#?}", peter);
}

#[test]
fn test() {
    main();
}