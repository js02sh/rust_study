use std::io;

fn get_user_input() -> Option<String> {
    println!("Write down your username:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .ok()?;

    //Remove the newline character from the end of the input
    input.pop();

    //Return the input as an 'Option' value
    Some(input)
}

fn main(){
    let name = get_user_input();

    //Check if the user input is valid.
    if let Some(name) = name {
        println!("Hello, {}!", name);
    } else {
        println!("Invalid user input.");
    }
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