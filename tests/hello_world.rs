//#[allow(dead_code)]
fn miru() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_miru() {
        miru();
    }
}
