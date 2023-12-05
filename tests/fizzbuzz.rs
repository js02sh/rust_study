//to run
//cargo test --test fizzbuzz -- --nocapture

fn main() {
    //1에서 100까지 반복
    for i in 1..101 {
        if i % 3 == 0 && i % 5== 0 { // 15의 배수이면 FizzBuzz
            println!("FizzBuzz");
        } else if i % 3 == 0 && i % 5 != 0 { //3의 배수이기만 하면 Fizz
            println!("Fizz");
        } else if i % 3 != 0 && i % 5 == 0 { //5의 배수이기만 하면 Buzz
            println!("Buzz");
        } else { // 그 외엔 그냥 숫자 출력
            println!("{}", i);
        }
    }
}

#[test]
fn test() {
    main();
}