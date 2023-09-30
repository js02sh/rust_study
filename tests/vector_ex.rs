fn main() {
    //let v: Vec<i32> = Vec::new();
    //let mut v = Vec::new();
    let v = vec![1,2,3,4,5,6,7];

    // v.push(5);
    // v.push(7);

    println!("{:?}", v);

    match v.get(2) {
        Some(t) => println!("The third element is {}", t),
        None => println!("There is no third element"),
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for (i, element) in v.iter().enumerate() {
        println!("index: {}, element: {}", i, element);
    }

    println!("{:?}", v);

    let s = vec!["first", "second", "third"].join("/");

    println!("{}", s);


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