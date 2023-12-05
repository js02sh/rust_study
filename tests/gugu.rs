//to run
//cargo test --test gugu -- --nocapture

fn main() {
    for i in 1..10 {
        for j in 1..10 {
            print!("{:3},", i * j); // {:3} 하게되면 오른쪽 정렬, 3칸 부여?
        }
        println!("");
    }
}


#[test]
fn test() {
    main();
}