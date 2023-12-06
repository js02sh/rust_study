//ex_ "cargo test --test match_shape -- --nocapture"

use std::f64::consts;

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn caclulate_are(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => consts::PI * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(length, width) => length * width,

    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    println!("Area of Circle: {:.2}", caclulate_are(&circle ));
    println!("Area of Suqare: {:2}", caclulate_are(&square));
    println!("Area of Rectangle: {:2}", caclulate_are(&rectangle));
}

#[test]
fn test() {
    main();
}