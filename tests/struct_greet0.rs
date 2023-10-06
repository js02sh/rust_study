//cargo test --test struct_greet0 -- --nocapture
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }

    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old", self.name, self.age);
    }
}

fn main() {
    let person1 = Person::new("Jisang", 23);
    person1.greet();
}



#[test]
fn test() {
    main();
}