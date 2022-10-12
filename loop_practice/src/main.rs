fn main() {
    let mut count = 100;
    let result = loop {
        if count <= 20{
            break count * 10;
        }
        count -= 2;
        println!("count is {count}");
    };
    println!("End Point");
    println!("result is {result}");
}
