//to run
//cargo test --test gugu -- --nocapture

fn main() {
    for i in 1..10 {
        for j in 1..10 {
            print!("{:3},", i * j); // {:3} 하게되면 오른쪽 정렬, 3칸 부여?
        }
        println!("\n");
    }

    for y in 1..10 {
        let s = (1..10)
            .map(|x| format!("{:3}", x * y))
            .collect::<Vec<String>>().join(",");
        println!("{}", s);
    }
}


#[test]
fn test() {
    main();
}